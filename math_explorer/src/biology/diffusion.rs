//! Spatial Diffusion Strategies
//!
//! This module provides strategies for computing the spatial diffusion term $D \nabla^2 u$
//! in reaction-diffusion systems.

/// Defines a strategy for computing spatial diffusion.
pub trait SpatialDiffusion {
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
    fn apply(&self, u: &[f64], v: &[f64], out_u: &mut [f64], out_v: &mut [f64], d_u: f64, d_v: f64);
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

impl SpatialDiffusion for FiniteDifference1D {
    fn apply(
        &self,
        u: &[f64],
        v: &[f64],
        out_u: &mut [f64],
        out_v: &mut [f64],
        d_u: f64,
        d_v: f64,
    ) {
        let n = u.len();
        if n == 0 {
            return;
        }

        // Validate slice lengths
        assert!(v.len() >= n, "v buffer too small");
        assert!(out_u.len() >= n, "out_u buffer too small");
        assert!(out_v.len() >= n, "out_v buffer too small");

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        // 1. Handle i = 0 (Left Boundary)
        {
            let u_curr = u[0];
            let v_curr = v[0];
            // Neumann BC: u_{-1} = u_0
            let u_prev = u_curr;
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                (u[1], v[1])
            } else {
                (u_curr, v_curr)
            };

            out_u[0] = d_u * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            out_v[0] = d_v * (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;
        }

        // 2. Interior (Safe Windows)
        if n > 2 {
            // Iterate over windows of 3 elements: [prev, curr, next]
            // We write to out starting at index 1
            for (((win_u, win_v), o_u), o_v) in u
                .windows(3)
                .zip(v.windows(3))
                .zip(out_u.iter_mut().skip(1))
                .zip(out_v.iter_mut().skip(1))
            {
                let u_prev = win_u[0];
                let u_curr = win_u[1];
                let u_next = win_u[2];

                let v_prev = win_v[0];
                let v_curr = win_v[1];
                let v_next = win_v[2];

                *o_u = d_u * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                *o_v = d_v * (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;
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

            out_u[i] = d_u * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            out_v[i] = d_v * (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;
        }
    }
}
