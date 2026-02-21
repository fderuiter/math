//! Spatial Diffusion Strategies
//!
//! This module provides strategies for computing the spatial diffusion term $D \nabla^2 u$
//! in reaction-diffusion systems.

/// Defines a strategy for computing spatial diffusion.
pub trait SpatialDiffusion<const N: usize> {
    /// Computes diffusion terms for each point and calls the closure.
    /// Internal iteration allows for optimization (loop fusion, SIMD).
    ///
    /// The closure `op` is called with `(index, vals, diffs)` where:
    /// * `index`: The linear index of the point.
    /// * `vals`: Current values of components at index.
    /// * `diffs`: The diffusion terms for components ($D \nabla^2 u$).
    fn map_diffusion<F>(&self, components: [&[f64]; N], coeffs: [f64; N], op: F)
    where
        F: FnMut(usize, [f64; N], [f64; N]);

    /// Applies the diffusion operator to the state vectors.
    ///
    /// Computes $D \nabla^2 u$ and stores the result in `out`.
    fn apply(
        &self,
        components: [&[f64]; N],
        out: [&mut [f64]; N],
        coeffs: [f64; N],
    ) {
        // Default implementation: calculate diffusion and write to buffer
        self.map_diffusion(components, coeffs, |i, _vals, diffs| {
            for s in 0..N {
                if i < out[s].len() {
                    out[s][i] = diffs[s];
                }
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

impl<const N: usize> SpatialDiffusion<N> for FiniteDifference1D {
    fn map_diffusion<F>(&self, components: [&[f64]; N], coeffs: [f64; N], mut op: F)
    where
        F: FnMut(usize, [f64; N], [f64; N]),
    {
        let n = components[0].len();
        if n == 0 {
            return;
        }

        for c in components.iter() {
            assert_eq!(c.len(), n, "Component buffer size mismatch");
        }

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        // 1. Left Boundary (i=0)
        {
            let mut vals = [0.0; N];
            let mut diffs = [0.0; N];

            for s in 0..N {
                let u_curr = components[s][0];
                vals[s] = u_curr;
                let u_prev = u_curr; // Neumann: u_{-1} = u_0
                let u_next = if n > 1 {
                    components[s][1]
                } else {
                    u_curr
                };
                diffs[s] = coeffs[s] * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            }
            op(0, vals, diffs);
        }

        // 2. Interior (Safe Windows)
        if n > 2 {
            // We iterate manually to handle multiple windows in parallel
            // Or we just loop i and index.
            // Using windows on array of slices is tricky.
            // Simple indexing is efficient enough here due to bounds check elimination in simple loops,
            // but let's try to be efficient.
            // However, iterating N components inside the loop is the key.

            for i in 1..n - 1 {
                let mut vals = [0.0; N];
                let mut diffs = [0.0; N];
                for s in 0..N {
                    let u_prev = components[s][i - 1];
                    let u_curr = components[s][i];
                    let u_next = components[s][i + 1];
                    vals[s] = u_curr;
                    diffs[s] = coeffs[s] * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                }
                op(i, vals, diffs);
            }
        }

        // 3. Handle i = n-1 (Right Boundary)
        if n > 1 {
            let i = n - 1;
            let mut vals = [0.0; N];
            let mut diffs = [0.0; N];

            for s in 0..N {
                let u_curr = components[s][i];
                vals[s] = u_curr;
                let u_prev = components[s][i - 1];
                let u_next = u_curr; // Neumann: u_{N} = u_{N-1}
                diffs[s] = coeffs[s] * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            }
            op(i, vals, diffs);
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

impl<const N: usize> SpatialDiffusion<N> for FiniteDifference2D {
    fn map_diffusion<F>(&self, components: [&[f64]; N], coeffs: [f64; N], mut op: F)
    where
        F: FnMut(usize, [f64; N], [f64; N]),
    {
        let n = self.width * self.height;
        if n == 0 {
            return;
        }
        for c in components.iter() {
            if c.len() != n {
                panic!("Buffer size mismatch in FiniteDifference2D");
            }
        }

        let inv_dx_sq = 1.0 / (self.dx * self.dx);
        let inv_dy_sq = 1.0 / (self.dy * self.dy);

        // Precompute weights
        let mut cx = [0.0; N];
        let mut cy = [0.0; N];
        let mut c_center = [0.0; N];

        for s in 0..N {
            cx[s] = coeffs[s] * inv_dx_sq;
            cy[s] = coeffs[s] * inv_dy_sq;
            c_center[s] = -2.0 * (cx[s] + cy[s]);
        }

        for y in 0..self.height {
            for x in 0..self.width {
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

                let mut vals = [0.0; N];
                let mut diffs = [0.0; N];

                for s in 0..N {
                    let u = components[s];
                    let u_curr = u[idx];
                    vals[s] = u_curr;

                    diffs[s] = (u[idx_r] + u[idx_l]) * cx[s]
                        + (u[idx_d] + u[idx_u]) * cy[s]
                        + u_curr * c_center[s];
                }

                op(idx, vals, diffs);
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

        diff.apply(
            [&u, &v],
            [&mut out_u, &mut out_v],
            [1.0, 1.0]
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
            [&u, &v],
            [&mut out_u, &mut out_v],
            [1.0, 1.0]
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
            [&u, &v],
            [&mut out_u_1, &mut out_v_1],
            [d_u, d_v]
        );
        for i in 0..n {
            out_u_1[i] = u[i] + dt * (out_u_1[i] + 1.0); // Dummy reaction +1
            out_v_1[i] = v[i] + dt * (out_v_1[i] + 2.0); // Dummy reaction +2
        }

        // Method 2: map_diffusion fused step
        diff.map_diffusion(
            [&u, &v],
            [d_u, d_v],
            |i, vals, diffs| {
            let u_curr = vals[0];
            let v_curr = vals[1];
            let diff_u = diffs[0];
            let diff_v = diffs[1];

            let (reac_u, reac_v) = (1.0, 2.0);
            out_u_2[i] = u_curr + dt * (diff_u + reac_u);
            out_v_2[i] = v_curr + dt * (diff_v + reac_v);
        });

        for i in 0..n {
            assert!((out_u_1[i] - out_u_2[i]).abs() < 1e-10);
            assert!((out_v_1[i] - out_v_2[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn test_diffusion_generic_3d() {
        let width = 5;
        let height = 5;
        let diff = FiniteDifference2D::new(width, height, 1.0, 1.0);
        let n = width * height;

        // 3 components
        let u = vec![1.0; n];
        let v = vec![2.0; n];
        let w = vec![3.0; n];

        let mut out_u = vec![0.0; n];
        let mut out_v = vec![0.0; n];
        let mut out_w = vec![0.0; n];

        diff.apply(
            [&u, &v, &w],
            [&mut out_u, &mut out_v, &mut out_w],
            [1.0, 1.0, 1.0],
        );

        for val in out_u {
            assert_eq!(val, 0.0);
        }
        for val in out_v {
            assert_eq!(val, 0.0);
        }
        for val in out_w {
            assert_eq!(val, 0.0);
        }
    }
}
