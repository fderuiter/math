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

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        // 1. Handle i = 0
        {
            let i = 0;
            // Safety: n > 0 checked above
            let u_curr = unsafe { *u.get_unchecked(i) };
            let v_curr = unsafe { *v.get_unchecked(i) };

            let u_prev = u_curr; // Neumann BC: u_{-1} = u_0
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                unsafe { (*u.get_unchecked(1), *v.get_unchecked(1)) }
            } else {
                (u_curr, v_curr)
            };

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            unsafe {
                *out_u.get_unchecked_mut(i) = d_u * lap_u;
                *out_v.get_unchecked_mut(i) = d_v * lap_v;
            }
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            // Optimization: Sliding Window / Register Rotation
            unsafe {
                let mut u_prev = *u.get_unchecked(0);
                let mut u_curr = *u.get_unchecked(1);
                let mut v_prev = *v.get_unchecked(0);
                let mut v_curr = *v.get_unchecked(1);

                for i in 1..n - 1 {
                    let u_next = *u.get_unchecked(i + 1);
                    let v_next = *v.get_unchecked(i + 1);

                    let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                    let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                    *out_u.get_unchecked_mut(i) = d_u * lap_u;
                    *out_v.get_unchecked_mut(i) = d_v * lap_v;

                    // Shift window
                    u_prev = u_curr;
                    u_curr = u_next;
                    v_prev = v_curr;
                    v_curr = v_next;
                }
            }
        }

        // 3. Handle i = n-1
        if n > 1 {
            let i = n - 1;
            unsafe {
                let u_curr = *u.get_unchecked(i);
                let v_curr = *v.get_unchecked(i);
                let u_prev = *u.get_unchecked(i - 1);
                let v_prev = *v.get_unchecked(i - 1);

                let u_next = u_curr; // Neumann BC: u_{N} = u_{N-1}
                let v_next = v_curr;

                let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                *out_u.get_unchecked_mut(i) = d_u * lap_u;
                *out_v.get_unchecked_mut(i) = d_v * lap_v;
            }
        }
    }
}
