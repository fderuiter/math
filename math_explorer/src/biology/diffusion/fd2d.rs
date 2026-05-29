use super::SpatialDiffusion;

/// A 2D Finite Difference implementation using a 5-point stencil.
///
/// This struct computes the discrete Laplacian operator $\nabla^2 u$ on a 2D rectangular grid.
/// It uses a standard 5-point stencil (center, left, right, up, down).
///
/// # The Stencil
///
/// The Laplacian is approximated using values from the center cell and its four immediate neighbors:
///
/// ```mermaid
/// graph TD
///     Up[u_{i, j+1}] --> Center[u_{i, j}]
///     Down[u_{i, j-1}] --> Center
///     Left[u_{i-1, j}] --> Center
///     Right[u_{i+1, j}] --> Center
///
///     style Center fill:#f9f,stroke:#333,stroke-width:2px
/// ```
///
/// $$ \nabla^2 u \approx \frac{u_{i+1,j} - 2u_{i,j} + u_{i-1,j}}{\Delta x^2} + \frac{u_{i,j+1} - 2u_{i,j} + u_{i,j-1}}{\Delta y^2} $$
///
/// # Boundary Conditions
///
/// This implementation enforces **Neumann Boundary Conditions** (zero-flux) at the edges:
/// $\frac{\partial u}{\partial n} = 0$.
/// Effectively, values at the boundary are mirrored from their immediate neighbors.
///
/// # Data Layout
///
/// The grid is flattened into a 1D array using **row-major order**:
/// `index = y * width + x`.
///
/// # Example
///
/// ```rust
/// use math_explorer::biology::diffusion::FiniteDifference2D;
///
/// // Create a 10x10 grid with unit spacing
/// let diff_solver = FiniteDifference2D::new(10, 10, 1.0, 1.0);
///
/// assert_eq!(diff_solver.width, 10);
/// assert_eq!(diff_solver.dx, 1.0);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct FiniteDifference2D {
    /// The number of grid points in the horizontal (x) direction.
    pub width: usize,
    /// The number of grid points in the vertical (y) direction.
    pub height: usize,
    /// The grid spacing (step size) in the x-direction ($\Delta x$).
    pub dx: f64,
    /// The grid spacing (step size) in the y-direction ($\Delta y$).
    pub dy: f64,
}

impl FiniteDifference2D {
    /// Creates a new 2D finite difference strategy.
    ///
    /// # Arguments
    ///
    /// * `width` - Number of columns in the grid.
    /// * `height` - Number of rows in the grid.
    /// * `dx` - Physical distance between horizontal grid points.
    /// * `dy` - Physical distance between vertical grid points.
    pub fn new(width: usize, height: usize, dx: f64, dy: f64) -> Self {
        Self {
            width,
            height,
            dx,
            dy,
        }
    }
}

impl crate::biology::reaction_diffusion::DiffusionModel for FiniteDifference2D {
    fn apply(
        &self,
        state: &crate::biology::reaction_diffusion::ChemicalState,
        out: &mut crate::biology::reaction_diffusion::ChemicalState,
        coeffs: &[f64],
    ) {
        let n_species = state.num_species();
        if n_species == 0 {
            return;
        }

        // Security Check: Use checked multiplication to avoid integer overflow
        let n_grid = self
            .width
            .checked_mul(self.height)
            .expect("Grid dimensions overflow usize");

        // In release mode, these checks are cheap. In debug, they catch errors.
        // We use assert! because dimension mismatch is a critical bug.
        assert_eq!(
            state.grid_size(),
            n_grid,
            "ChemicalState grid size mismatch with FiniteDifference2D"
        );
        assert_eq!(out.grid_size(), n_grid, "Output state grid size mismatch");
        assert_eq!(
            coeffs.len(),
            n_species,
            "Diffusion coefficients count mismatch"
        );

        for (s, coeff) in coeffs.iter().enumerate().take(n_species) {
            let src = state.species(s);
            let dst = out.species_mut(s);
            let coeff = *coeff;

            apply_2d_stencil_optimized(self.width, self.height, self.dx, self.dy, src, dst, coeff);
        }
    }
}

impl<const N: usize> SpatialDiffusion<N> for FiniteDifference2D {
    fn map_diffusion<F>(&self, state: [&[f64]; N], coeffs: [f64; N], mut op: F)
    where
        F: FnMut(usize, [f64; N], [f64; N]),
    {
        if N == 0 {
            return;
        }

        // Security Check: Use checked multiplication to avoid integer overflow
        let n = self
            .width
            .checked_mul(self.height)
            .expect("Grid dimensions overflow usize");

        if n == 0 {
            return;
        }

        // Basic check for one buffer length to ensure safety
        if state[0].len() < n {
            panic!("Buffer size mismatch in FiniteDifference2D");
        }

        let inv_dx_sq = 1.0 / (self.dx * self.dx);
        let inv_dy_sq = 1.0 / (self.dy * self.dy);

        let mut cx = [0.0; N];
        let mut cy = [0.0; N];
        let mut c_center = [0.0; N];

        for (s, coeff) in coeffs.iter().enumerate().take(N) {
            cx[s] = coeff * inv_dx_sq;
            cy[s] = coeff * inv_dy_sq;
            c_center[s] = -2.0 * (cx[s] + cy[s]);
        }

        // Verify all buffers are large enough to avoid UB or panics
        for (s, buffer) in state.iter().enumerate().take(N) {
            assert!(
                buffer.len() >= n,
                "Buffer too small for diffusion (species {})",
                s
            );
        }

        iter_stencil_2d(
            self.width,
            self.height,
            |idx, idx_l, idx_r, idx_u, idx_d| {
                let mut current_vals = [0.0; N];
                let mut diff_vals = [0.0; N];

                for s in 0..N {
                    let u = state[s];
                    // SAFETY: All indices are guaranteed valid by iter_stencil_2d logic
                    // and we explicitly asserted lengths above.
                    // We transition to safe indexing to guarantee no UB.
                    let u_curr = u[idx];
                    let u_l = u[idx_l];
                    let u_r = u[idx_r];
                    let u_u = u[idx_u];
                    let u_d = u[idx_d];

                    let diff = (u_r + u_l) * cx[s] + (u_d + u_u) * cy[s] + u_curr * c_center[s];

                    current_vals[s] = u_curr;
                    diff_vals[s] = diff;
                }
                op(idx, current_vals, diff_vals);
            },
        );
    }
}

/// Applies a 2D Finite Difference stencil to a single array.
///
/// This helper implements a loop-splitting optimization to separate the hot interior path
/// from the boundary handling, eliminating conditional checks for the majority of grid points.
///
/// # Arguments
/// * `width` - Grid width.
/// * `height` - Grid height.
/// * `dx` - Grid spacing x.
/// * `dy` - Grid spacing y.
/// * `src` - Input concentration slice.
/// * `dst` - Output buffer for Laplacian.
/// * `coeff` - Diffusion coefficient.
fn apply_2d_stencil_optimized(
    width: usize,
    height: usize,
    dx: f64,
    dy: f64,
    src: &[f64],
    dst: &mut [f64],
    coeff: f64,
) {
    // Security Check: Use checked multiplication to avoid integer overflow
    let n = width
        .checked_mul(height)
        .expect("Grid dimensions overflow usize");

    if src.len() < n || dst.len() < n {
        // In a real system this might panic, but for a helper we just return
        return;
    }

    let inv_dx_sq = 1.0 / (dx * dx);
    let inv_dy_sq = 1.0 / (dy * dy);
    let cx = coeff * inv_dx_sq;
    let cy = coeff * inv_dy_sq;
    let c_center = -2.0 * (cx + cy);

    iter_stencil_2d(width, height, |idx, idx_l, idx_r, idx_u, idx_d| {
        // SAFETY: iter_stencil_2d guarantees indices are within 0..width*height
        // and we checked src/dst lengths >= n.
        // We transition to safe indexing to guarantee no UB.
        let u_curr = src[idx];
        let u_l = src[idx_l];
        let u_r = src[idx_r];
        let u_u = src[idx_u];
        let u_d = src[idx_d];

        let diff = (u_r + u_l) * cx + (u_d + u_u) * cy + u_curr * c_center;
        dst[idx] = diff;
    });
}

/// Iterates over a 2D grid, providing indices for the center and its 4 neighbors (Neumann BC).
///
/// This helper implements loop splitting to optimize interior access.
///
/// # Arguments
/// * `width` - Grid width.
/// * `height` - Grid height.
/// * `op` - Closure called with (center_idx, left_idx, right_idx, up_idx, down_idx).
#[inline(always)]
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn iter_stencil_2d<F>(width: usize, height: usize, mut op: F)
where
    F: FnMut(usize, usize, usize, usize, usize),
{
    // Fallback for small grids
    if width < 3 || height < 3 {
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let x_prev = if x > 0 { x - 1 } else { x };
                let x_next = if x < width - 1 { x + 1 } else { x };
                let y_prev = if y > 0 { y - 1 } else { y };
                let y_next = if y < height - 1 { y + 1 } else { y };

                let idx_l = y * width + x_prev;
                let idx_r = y * width + x_next;
                let idx_u = y_prev * width + x;
                let idx_d = y_next * width + x;

                op(idx, idx_l, idx_r, idx_u, idx_d);
            }
        }
        return;
    }

    // 1. Top Row (y=0)
    {
        let y = 0;
        let y_prev = 0;
        let y_next = 1;
        for x in 0..width {
            let idx = x;
            let x_prev = if x > 0 { x - 1 } else { x };
            let x_next = if x < width - 1 { x + 1 } else { x };

            let idx_l = y * width + x_prev;
            let idx_r = y * width + x_next;
            let idx_u = y_prev * width + x;
            let idx_d = y_next * width + x;
            op(idx, idx_l, idx_r, idx_u, idx_d);
        }
    }

    // 2. Interior Rows
    for y in 1..height - 1 {
        let row_offset = y * width;

        // Left Col
        {
            let x = 0;
            let idx = row_offset;
            let x_prev = 0;
            let x_next = 1;

            let idx_l = row_offset + x_prev;
            let idx_r = row_offset + x_next;
            let idx_u = (y - 1) * width + x;
            let idx_d = (y + 1) * width + x;
            op(idx, idx_l, idx_r, idx_u, idx_d);
        }

        // Interior
        for x in 1..width - 1 {
            let idx = row_offset + x;
            let idx_l = idx - 1;
            let idx_r = idx + 1;
            let idx_u = idx - width;
            let idx_d = idx + width;
            op(idx, idx_l, idx_r, idx_u, idx_d);
        }

        // Right Col
        {
            let x = width - 1;
            let idx = row_offset + x;
            let x_prev = x - 1;
            let x_next = x;

            let idx_l = row_offset + x_prev;
            let idx_r = row_offset + x_next;
            let idx_u = (y - 1) * width + x;
            let idx_d = (y + 1) * width + x;
            op(idx, idx_l, idx_r, idx_u, idx_d);
        }
    }

    // 3. Bottom Row
    {
        let y = height - 1;
        let y_prev = y - 1;
        let y_next = y;
        for x in 0..width {
            let idx = y * width + x;
            let x_prev = if x > 0 { x - 1 } else { x };
            let x_next = if x < width - 1 { x + 1 } else { x };

            let idx_l = y * width + x_prev;
            let idx_r = y * width + x_next;
            let idx_u = y_prev * width + x;
            let idx_d = y_next * width + x;
            op(idx, idx_l, idx_r, idx_u, idx_d);
        }
    }
}

#[cfg(test)]
mod tests_2d {
    use super::*;

    #[test]
    fn test_laplacian_uniform() {
        let width = 10;
        let height = 10;
        let diff = FiniteDifference2D::new(width, height, 1.0, 1.0);

        let n = width * height;
        let u = vec![1.0; n];
        let v = vec![2.0; n];
        let mut out_u = vec![0.0; n];
        let mut out_v = vec![0.0; n];

        // Use array arguments for N=2
        diff.apply(
            [u.as_slice(), v.as_slice()],
            [out_u.as_mut_slice(), out_v.as_mut_slice()],
            [1.0, 1.0],
        );

        for val in out_u {
            assert_eq!(val, 0.0);
        }
        for val in out_v {
            assert_eq!(val, 0.0);
        }
    }

    #[test]
    fn test_laplacian_parabolic() {
        let width = 5;
        let height = 5;
        let diff = FiniteDifference2D::new(width, height, 1.0, 1.0);

        let n = width * height;
        let mut u = vec![0.0; n];
        let v = vec![0.0; n]; // unused
        let mut out_u = vec![0.0; n];
        let mut out_v = vec![0.0; n];

        // u = x^2 + y^2
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                u[idx] = (x as f64).powi(2) + (y as f64).powi(2);
            }
        }

        diff.apply(
            [u.as_slice(), v.as_slice()],
            [out_u.as_mut_slice(), out_v.as_mut_slice()],
            [1.0, 1.0],
        );

        // Interior points should be exactly 4.0
        // (1,1) is index 1*5 + 1 = 6.
        // (3,3) is index 3*5 + 3 = 18.
        // Interior range: x in 1..3, y in 1..3
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let idx = y * width + x;
                assert!(
                    (out_u[idx] - 4.0).abs() < 1e-10,
                    "Failed at ({}, {}): {}",
                    x,
                    y,
                    out_u[idx]
                );
            }
        }
    }

    #[test]
    fn test_map_diffusion_equivalence() {
        let width = 5;
        let height = 5;
        let diff = FiniteDifference2D::new(width, height, 1.0, 1.0);

        let n = width * height;
        let mut u = vec![0.0; n];
        let mut v = vec![0.0; n];

        // Randomish initialization
        for i in 0..n {
            u[i] = (i as f64) * 0.1;
            v[i] = (n - i) as f64 * 0.1;
        }

        let mut out_u_1 = vec![0.0; n];
        let mut out_v_1 = vec![0.0; n];
        let mut out_u_2 = vec![0.0; n];
        let mut out_v_2 = vec![0.0; n];

        let dt = 0.01;
        let d_u = 0.5;
        let d_v = 0.1;

        // Method 1: Manual step using apply
        diff.apply(
            [u.as_slice(), v.as_slice()],
            [out_u_1.as_mut_slice(), out_v_1.as_mut_slice()],
            [d_u, d_v],
        );
        for i in 0..n {
            out_u_1[i] = u[i] + dt * (out_u_1[i] + 1.0); // Dummy reaction +1
            out_v_1[i] = v[i] + dt * (out_v_1[i] + 2.0); // Dummy reaction +2
        }

        // Method 2: map_diffusion fused step
        diff.map_diffusion(
            [u.as_slice(), v.as_slice()],
            [d_u, d_v],
            |i, vals, diffs| {
                let u_curr = vals[0];
                let v_curr = vals[1];
                let diff_u = diffs[0];
                let diff_v = diffs[1];

                let (reac_u, reac_v) = (1.0, 2.0);
                out_u_2[i] = u_curr + dt * (diff_u + reac_u);
                out_v_2[i] = v_curr + dt * (diff_v + reac_v);
            },
        );

        for i in 0..n {
            assert!((out_u_1[i] - out_u_2[i]).abs() < 1e-10);
            assert!((out_v_1[i] - out_v_2[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_diffusion_model_2d() {
        use crate::biology::reaction_diffusion::{ChemicalState, DiffusionModel};

        let width = 5;
        let height = 5;
        let dx = 1.0;
        let dy = 1.0;
        let diff = FiniteDifference2D::new(width, height, dx, dy);

        let n = width * height;
        let mut state = ChemicalState::new(2, n);
        let mut out = ChemicalState::new(2, n);

        // Initialize with a simple pattern: center point high
        let center_idx = 2 * width + 2; // (2, 2)
        state.species_mut(0)[center_idx] = 1.0;

        // Apply diffusion
        let coeffs = [0.1, 0.2];
        DiffusionModel::apply(&diff, &state, &mut out, &coeffs);

        // Check center point diffusion
        // Laplacian at center: (0+0+0+0 - 4*1) = -4
        // D*Lap = 0.1 * -4 = -0.4
        let expected_center = -4.0 * coeffs[0];
        let val = out.species(0)[center_idx];
        assert!(
            (val - expected_center).abs() < 1e-10,
            "Expected {}, got {}",
            expected_center,
            val
        );

        // Check neighbor points (should receive flux)
        // Neighbor Laplacian: (1+0+0+0 - 4*0) = 1
        // D*Lap = 0.1 * 1 = 0.1
        let neighbor_idx = 2 * width + 3; // (3, 2)
        let expected_neighbor = 1.0 * coeffs[0];
        let val_neighbor = out.species(0)[neighbor_idx];
        assert!(
            (val_neighbor - expected_neighbor).abs() < 1e-10,
            "Expected {}, got {}",
            expected_neighbor,
            val_neighbor
        );

        // Verify 2nd species works independently
        assert_eq!(out.species(1)[center_idx], 0.0);
    }
}
