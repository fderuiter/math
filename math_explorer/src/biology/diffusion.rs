//! Spatial Diffusion Strategies
//!
//! This module provides strategies for computing the spatial diffusion term $D \nabla^2 u$
//! in reaction-diffusion systems.

/// Defines a strategy for computing spatial diffusion.
pub trait SpatialDiffusion {
    /// Computes diffusion terms for each point and calls the closure.
    /// Internal iteration allows for optimization (loop fusion, SIMD).
    ///
    /// The closure `op` is called with `(index, u_curr, v_curr, diff_u, diff_v)` where:
    /// * `index`: The linear index of the point.
    /// * `u_curr`: Current value of u at index.
    /// * `v_curr`: Current value of v at index.
    /// * `diff_u`: The diffusion term for u ($D_u \nabla^2 u$).
    /// * `diff_v`: The diffusion term for v ($D_v \nabla^2 v$).
    fn map_diffusion<F>(&self, u: &[f64], v: &[f64], d_u: f64, d_v: f64, op: F)
    where
        F: FnMut(usize, f64, f64, f64, f64);

    /// Applies the diffusion operator to the state vectors.
    ///
    /// Computes $D_u \nabla^2 u$ and $D_v \nabla^2 v$ and stores the result in `out_u` and `out_v`.
    ///
    /// # Arguments
    /// * `u` - Input activator concentration slice.
    /// * `v` - Input inhibitor concentration slice.
    /// * `out_u` - Output buffer for activator diffusion term.
    /// * `out_v` - Output buffer for inhibitor diffusion term.
    /// * `d_u` - Diffusion coefficient for u.
    /// * `d_v` - Diffusion coefficient for v.
    fn apply(
        &self,
        u: &[f64],
        v: &[f64],
        out_u: &mut [f64],
        out_v: &mut [f64],
        d_u: f64,
        d_v: f64,
    ) {
        // Default implementation: calculate diffusion and write to buffer
        self.map_diffusion(u, v, d_u, d_v, |i, _u_curr, _v_curr, diff_u, diff_v| {
            if i < out_u.len() {
                out_u[i] = diff_u;
            }
            if i < out_v.len() {
                out_v[i] = diff_v;
            }
        });
    }
}

/// A 1D Finite Difference implementation using a 3-point stencil.
///
/// Handles boundaries with Neumann conditions (zero flux).
#[derive(Debug, Clone, Copy)]
pub struct FiniteDifference1D {
    /// Grid spacing.
    pub dx: f64,
}

impl FiniteDifference1D {
    /// Creates a new 1D finite difference strategy.
    pub fn new(dx: f64) -> Self {
        Self { dx }
    }
}

impl crate::biology::reaction_diffusion::DiffusionModel for FiniteDifference1D {
    fn apply(
        &self,
        state: &crate::biology::reaction_diffusion::ChemicalState,
        out: &mut crate::biology::reaction_diffusion::ChemicalState,
        coeffs: &[f64],
    ) {
        let n_species = state.num_species();
        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        for (s, d) in coeffs.iter().enumerate().take(n_species) {
            let u = &state.concentrations[s];
            let out_u = &mut out.concentrations[s];
            let d = *d;
            apply_1d_stencil(u, out_u, d, inv_dx_sq);
        }
    }
}

impl SpatialDiffusion for FiniteDifference1D {
    fn map_diffusion<F>(&self, u: &[f64], v: &[f64], d_u: f64, d_v: f64, mut op: F)
    where
        F: FnMut(usize, f64, f64, f64, f64),
    {
        let n = u.len();
        if n == 0 {
            return;
        }

        assert_eq!(v.len(), n, "v buffer size mismatch");

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        // 1. Left Boundary (i=0)
        {
            let u_curr = u[0];
            let v_curr = v[0];
            let u_prev = u_curr; // Neumann: u_{-1} = u_0
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                (u[1], v[1])
            } else {
                (u_curr, v_curr)
            };

            let lap_u = d_u * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = d_v * (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            op(0, u_curr, v_curr, lap_u, lap_v);
        }

        // 2. Interior (Safe Windows)
        if n > 2 {
            // Iterate over windows of 3 elements: [prev, curr, next]
            for (i, (win_u, win_v)) in u.windows(3).zip(v.windows(3)).enumerate() {
                let u_prev = win_u[0];
                let u_curr = win_u[1];
                let u_next = win_u[2];

                let v_prev = win_v[0];
                let v_curr = win_v[1];
                let v_next = win_v[2];

                let lap_u = d_u * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                let lap_v = d_v * (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                // Enumerate starts at 0, but window corresponds to index 1
                op(i + 1, u_curr, v_curr, lap_u, lap_v);
            }
        }

        // 3. Handle i = n-1 (Right Boundary)
        if n > 1 {
            let i = n - 1;
            let u_curr = u[i];
            let v_curr = v[i];
            let u_prev = u[i - 1];
            let v_prev = v[i - 1];
            // Neumann BC: u_{N} = u_{N-1}
            let u_next = u_curr;
            let v_next = v_curr;

            let lap_u = d_u * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = d_v * (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            op(i, u_curr, v_curr, lap_u, lap_v);
        }
    }
}

/// Applies a 1D Finite Difference stencil (Neumann BC) to a single array.
///
/// This is a private helper to ensure DRY compliance between DiffusionModel and SpatialDiffusion implementations.
///
/// # Arguments
/// * `src`: Input concentration slice.
/// * `dst`: Output buffer for the Laplacian term (D * d2u/dx2).
/// * `d`: Diffusion coefficient.
/// * `inv_dx_sq`: Inverse square of grid spacing (1/dx^2).
fn apply_1d_stencil(src: &[f64], dst: &mut [f64], d: f64, inv_dx_sq: f64) {
    scan_1d_stencil(src, d, inv_dx_sq, |i, val| {
        // Safety: We rely on the caller to ensure dst is sized correctly.
        // scan_1d_stencil guarantees i < src.len(), which should match dst.len().
        if i < dst.len() {
            dst[i] = val;
        }
    });
}

/// Helper to apply 1D Finite Difference stencil.
/// Calls `op` with (index, laplacian_value) for each point.
fn scan_1d_stencil<F>(src: &[f64], d: f64, inv_dx_sq: f64, mut op: F)
where
    F: FnMut(usize, f64),
{
    let n = src.len();
    if n == 0 {
        return;
    }

    // 1. Left Boundary (i=0)
    {
        let u_curr = src[0];
        let u_prev = u_curr; // Neumann: u_{-1} = u_0
        let u_next = if n > 1 { src[1] } else { u_curr };
        let lap = d * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
        op(0, lap);
    }

    // 2. Interior (Optimized with windows iterator)
    if n > 2 {
        // Iterate over windows of 3 elements: [prev, curr, next]
        // Window index 0 corresponds to center index 1.
        for (i, win) in src.windows(3).enumerate() {
            let u_prev = win[0];
            let u_curr = win[1];
            let u_next = win[2];
            let lap = d * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            op(i + 1, lap);
        }
    }

    // 3. Right Boundary (i=n-1)
    if n > 1 {
        let i = n - 1;
        let u_curr = src[i];
        let u_prev = src[i - 1];
        let u_next = u_curr; // Neumann: u_{N} = u_{N-1}
        let lap = d * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
        op(i, lap);
    }
}

/// A 2D Finite Difference implementation using a 5-point stencil.
///
/// Handles boundaries with Neumann conditions (zero flux).
#[derive(Debug, Clone, Copy)]
pub struct FiniteDifference2D {
    pub width: usize,
    pub height: usize,
    pub dx: f64,
    pub dy: f64,
}

impl FiniteDifference2D {
    /// Creates a new 2D finite difference strategy.
    pub fn new(width: usize, height: usize, dx: f64, dy: f64) -> Self {
        Self {
            width,
            height,
            dx,
            dy,
        }
    }
}

impl SpatialDiffusion for FiniteDifference2D {
    fn map_diffusion<F>(&self, u: &[f64], v: &[f64], d_u: f64, d_v: f64, mut op: F)
    where
        F: FnMut(usize, f64, f64, f64, f64),
    {
        let n = self.width * self.height;
        // Reviewer feedback: Ensure buffer is at least n to guarantee safety of unchecked access.
        assert!(u.len() >= n, "u buffer too small");
        assert!(v.len() >= n, "v buffer too small");

        if n == 0 {
            return;
        }

        let inv_dx_sq = 1.0 / (self.dx * self.dx);
        let inv_dy_sq = 1.0 / (self.dy * self.dy);

        // Precompute weights for U
        let cx_u = d_u * inv_dx_sq;
        let cy_u = d_u * inv_dy_sq;
        let c_center_u = -2.0 * (cx_u + cy_u);

        // Precompute weights for V
        let cx_v = d_v * inv_dx_sq;
        let cy_v = d_v * inv_dy_sq;
        let c_center_v = -2.0 * (cx_v + cy_v);

        // Helper closure for stencil calculation to avoid code duplication
        let calc_stencil = |idx: usize,
                            idx_l: usize,
                            idx_r: usize,
                            idx_u: usize,
                            idx_d: usize|
         -> (f64, f64, f64, f64) {
            // Safety: Caller must ensure indices are valid.
            // Indices are guaranteed to be < n by the logic in process_safe (clamping) and the interior loop (range bounds).
            unsafe {
                let u_curr = *u.get_unchecked(idx);
                let diff_u = (*u.get_unchecked(idx_r) + *u.get_unchecked(idx_l)) * cx_u
                    + (*u.get_unchecked(idx_d) + *u.get_unchecked(idx_u)) * cy_u
                    + u_curr * c_center_u;

                let v_curr = *v.get_unchecked(idx);
                let diff_v = (*v.get_unchecked(idx_r) + *v.get_unchecked(idx_l)) * cx_v
                    + (*v.get_unchecked(idx_d) + *v.get_unchecked(idx_u)) * cy_v
                    + v_curr * c_center_v;
                (u_curr, v_curr, diff_u, diff_v)
            }
        };

        // Helper closure to process a single point safely (with boundary checks)
        // We define this to avoid code duplication for the fallback and boundary processing.
        let process_safe = |x: usize, y: usize, op: &mut F| {
            let idx = y * self.width + x;

            // Neighbor indices with Neumann BC clamping
            let x_prev = if x > 0 { x - 1 } else { x };
            let x_next = if x < self.width - 1 { x + 1 } else { x };
            let y_prev = if y > 0 { y - 1 } else { y };
            let y_next = if y < self.height - 1 { y + 1 } else { y };

            let idx_l = y * self.width + x_prev;
            let idx_r = y * self.width + x_next;
            let idx_u = y_prev * self.width + x;
            let idx_d = y_next * self.width + x;

            let (u_curr, v_curr, diff_u, diff_v) = calc_stencil(idx, idx_l, idx_r, idx_u, idx_d);
            op(idx, u_curr, v_curr, diff_u, diff_v);
        };

        // Fallback for small grids where interior optimization isn't possible
        if self.width < 3 || self.height < 3 {
            for y in 0..self.height {
                for x in 0..self.width {
                    process_safe(x, y, &mut op);
                }
            }
            return;
        }

        // Optimized Loop Splitting Strategy
        // We iterate y from 0 to height-1.
        // For interior rows (1..height-1), we process the interior (1..width-1) using unsafe unchecked access,
        // which avoids the boundary checks and branches in the hot path.

        for y in 0..self.height {
            let is_boundary_row = y == 0 || y == self.height - 1;

            if is_boundary_row {
                // Process the entire row safely
                for x in 0..self.width {
                    process_safe(x, y, &mut op);
                }
            } else {
                // 1. Left Boundary (x=0)
                process_safe(0, y, &mut op);

                // 2. Interior (Hot Path)
                // x ranges from 1 to width-2 (inclusive)
                let row_offset = y * self.width;
                for x in 1..self.width - 1 {
                    let idx = row_offset + x;

                    // Indices for calc_stencil
                    let idx_l = idx - 1;
                    let idx_r = idx + 1;
                    let idx_u = idx - self.width;
                    let idx_d = idx + self.width;

                    // Safety:
                    // x is in [1, width-2], y is in [1, height-2].
                    // All indices are strictly within bounds [0, n-1].
                    let (u_curr, v_curr, diff_u, diff_v) =
                        calc_stencil(idx, idx_l, idx_r, idx_u, idx_d);
                    op(idx, u_curr, v_curr, diff_u, diff_v);
                }

                // 3. Right Boundary (x=width-1)
                process_safe(self.width - 1, y, &mut op);
            }
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

        diff.apply(&u, &v, &mut out_u, &mut out_v, 1.0, 1.0);

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

        diff.apply(&u, &v, &mut out_u, &mut out_v, 1.0, 1.0);

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
        diff.apply(&u, &v, &mut out_u_1, &mut out_v_1, d_u, d_v);
        for i in 0..n {
            out_u_1[i] = u[i] + dt * (out_u_1[i] + 1.0); // Dummy reaction +1
            out_v_1[i] = v[i] + dt * (out_v_1[i] + 2.0); // Dummy reaction +2
        }

        // Method 2: map_diffusion fused step
        diff.map_diffusion(&u, &v, d_u, d_v, |i, u_curr, v_curr, diff_u, diff_v| {
            let (reac_u, reac_v) = (1.0, 2.0);
            out_u_2[i] = u_curr + dt * (diff_u + reac_u);
            out_v_2[i] = v_curr + dt * (diff_v + reac_v);
        });

        for i in 0..n {
            assert!((out_u_1[i] - out_u_2[i]).abs() < 1e-10);
            assert!((out_v_1[i] - out_v_2[i]).abs() < 1e-10);
        }
    }
}
