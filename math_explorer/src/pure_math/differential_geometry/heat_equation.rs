use super::surface::ParametricSurface;

/// Solves the Heat Equation $\frac{\partial u}{\partial t} = \alpha \Delta_S u$ on a parametric surface.
/// Uses explicit Euler time-stepping with a discrete 5-point stencil for the Laplace-Beltrami operator.
pub struct HeatEquationSolver<'a, S: ParametricSurface> {
    pub surface: &'a S,
    pub alpha: f64, // Thermal diffusivity
    pub u_grid: Vec<Vec<f64>>, // Current temperature state
    pub grid_res: (usize, usize), // (n_u, n_v)
    pub range_u: (f64, f64),
    pub range_v: (f64, f64),
}

impl<'a, S: ParametricSurface> HeatEquationSolver<'a, S> {
    pub fn new(
        surface: &'a S,
        alpha: f64,
        range_u: (f64, f64),
        range_v: (f64, f64),
        grid_res: (usize, usize),
        initial_condition: impl Fn(f64, f64) -> f64,
    ) -> Self {
        let (nu, nv) = grid_res;
        let mut grid = vec![vec![0.0; nv]; nu];

        let du = (range_u.1 - range_u.0) / (nu as f64 - 1.0);
        let dv = (range_v.1 - range_v.0) / (nv as f64 - 1.0);

        for (i, row) in grid.iter_mut().enumerate().take(nu) {
            for (j, val) in row.iter_mut().enumerate().take(nv) {
                let u = range_u.0 + i as f64 * du;
                let v = range_v.0 + j as f64 * dv;
                *val = initial_condition(u, v);
            }
        }

        Self {
            surface,
            alpha,
            u_grid: grid,
            grid_res,
            range_u,
            range_v,
        }
    }

    /// Advances the simulation by time step `dt`.
    /// Uses $\Delta_S f = \frac{1}{\sqrt{g}} [ \partial_u (\frac{G f_u - F f_v}{\sqrt{g}}) + \partial_v (\frac{E f_v - F f_u}{\sqrt{g}}) ]$
    pub fn step(&mut self, dt: f64) {
        let (nu, nv) = self.grid_res;
        let du = (self.range_u.1 - self.range_u.0) / (nu as f64 - 1.0);
        let dv = (self.range_v.1 - self.range_v.0) / (nv as f64 - 1.0);

        let mut new_grid = self.u_grid.clone();

        // Safe getter with wrapping for periodicity (assuming closed surface or periodic domain like torus)
        // For general surfaces, Neumann conditions (replicate boundary) are often safer,
        // but for Sphere/Torus physics, wrapping is often desired on angles.
        // Let's implement clamp for generic stability in this library context.
        let get = |grid: &Vec<Vec<f64>>, i: i32, j: i32| -> f64 {
            let i_idx = i.clamp(0, (nu - 1) as i32) as usize;
            let j_idx = j.clamp(0, (nv - 1) as i32) as usize;
            grid[i_idx][j_idx]
        };

        #[allow(clippy::needless_range_loop)]
        for i in 0..nu {
            for j in 0..nv {
                let u = self.range_u.0 + i as f64 * du;
                let v = self.range_v.0 + j as f64 * dv;

                // 1. Calculate derivatives of f at (i, j)
                // We need half-step derivatives for the divergence.
                // Or we can use central differences for the whole operator.
                // Let's use central differences for the outer derivatives and central for inner.
                // Standard discretization of divergence form:
                // d_u (A f_u) ~ (A_{i+1/2} (f_{i+1} - f_i)/du - A_{i-1/2} (f_i - f_{i-1})/du) / du

                // Let's compute flux terms at half-points.

                // Flux U terms: A = (G f_u - F f_v) / sqrt(g)
                // We need A at (i+0.5, j) and (i-0.5, j)

                let compute_flux_u = |i_base: i32, j_base: i32| -> f64 {
                    // Evaluate metric at half step u
                    let u_half = self.range_u.0 + (i_base as f64 + 0.5) * du;
                    let v_curr = self.range_v.0 + j_base as f64 * dv;
                    let (e, f, g_metric) = self.surface.first_fundamental_form(u_half, v_curr);
                    let sqrt_g = (e * g_metric - f * f).sqrt();

                    // f_u at i+0.5 is (f_{i+1} - f_i) / du
                    let f_u = (get(&self.u_grid, i_base + 1, j_base) - get(&self.u_grid, i_base, j_base)) / du;

                    // f_v at i+0.5 is average of f_v at i and i+1?
                    // f_v at i = (f_{i, j+1} - f_{i, j-1}) / 2dv
                    let f_v_i = (get(&self.u_grid, i_base, j_base + 1) - get(&self.u_grid, i_base, j_base - 1)) / (2.0 * dv);
                    let f_v_ip1 = (get(&self.u_grid, i_base + 1, j_base + 1) - get(&self.u_grid, i_base + 1, j_base - 1)) / (2.0 * dv);
                    let f_v = 0.5 * (f_v_i + f_v_ip1);

                    (g_metric * f_u - f * f_v) / sqrt_g
                };

                let flux_u_plus = compute_flux_u(i as i32, j as i32);
                let flux_u_minus = compute_flux_u(i as i32 - 1, j as i32);

                // Flux V terms: B = (E f_v - F f_u) / sqrt(g)
                // We need B at (i, j+0.5) and (i, j-0.5)

                let compute_flux_v = |i_base: i32, j_base: i32| -> f64 {
                    let u_curr = self.range_u.0 + i_base as f64 * du;
                    let v_half = self.range_v.0 + (j_base as f64 + 0.5) * dv;
                    let (e, f, g_metric) = self.surface.first_fundamental_form(u_curr, v_half);
                    let sqrt_g = (e * g_metric - f * f).sqrt();

                    // f_v at j+0.5 is (f_{j+1} - f_j) / dv
                    let f_v = (get(&self.u_grid, i_base, j_base + 1) - get(&self.u_grid, i_base, j_base)) / dv;

                    // f_u at j+0.5 is average
                    let f_u_j = (get(&self.u_grid, i_base + 1, j_base) - get(&self.u_grid, i_base - 1, j_base)) / (2.0 * du);
                    let f_u_jp1 = (get(&self.u_grid, i_base + 1, j_base + 1) - get(&self.u_grid, i_base - 1, j_base + 1)) / (2.0 * du);
                    let f_u = 0.5 * (f_u_j + f_u_jp1);

                    (e * f_v - f * f_u) / sqrt_g
                };

                let flux_v_plus = compute_flux_v(i as i32, j as i32);
                let flux_v_minus = compute_flux_v(i as i32, j as i32 - 1);

                // Divergence
                let term1 = (flux_u_plus - flux_u_minus) / du;
                let term2 = (flux_v_plus - flux_v_minus) / dv;

                let area = self.surface.area_element(u, v);
                let laplacian = (1.0 / area) * (term1 + term2);

                new_grid[i][j] += dt * self.alpha * laplacian;
            }
        }

        self.u_grid = new_grid;
    }
}
