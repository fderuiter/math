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

    /// Applies a fused diffusion + reaction + time integration step.
    ///
    /// This allows strategies to optimize the memory access pattern by fusing the loops.
    /// The default implementation calls `apply` and then performs the reaction/integration in a separate pass.
    ///
    /// # Arguments
    /// * `u` - Input activator concentration slice.
    /// * `v` - Input inhibitor concentration slice.
    /// * `out_u` - Output buffer (acts as both diffusion scratchpad and final state destination).
    /// * `out_v` - Output buffer.
    /// * `d_u` - Diffusion coefficient for u.
    /// * `d_v` - Diffusion coefficient for v.
    /// * `dt` - Time step.
    /// * `reaction` - Closure computing reaction rates (du/dt, dv/dt) given (u, v).
    #[allow(clippy::too_many_arguments)]
    fn apply_step<F>(
        &self,
        u: &[f64],
        v: &[f64],
        out_u: &mut [f64],
        out_v: &mut [f64],
        d_u: f64,
        d_v: f64,
        dt: f64,
        reaction: F,
    ) where
        F: Fn(f64, f64) -> (f64, f64),
    {
        // 1. Compute Diffusion (writes D*Lap into out buffers)
        self.apply(u, v, out_u, out_v, d_u, d_v);

        // 2. Compute Reaction and Integrate
        // Note: out_u currently holds D*Lap(u)
        // Using iterators to avoid bounds checking in default impl (though compiler handles it well usually)
        // But for consistency with typical safe code:
        let len = u.len();
        for i in 0..len {
            // Safety: We assume apply() verified lengths or they match u.len()
            if i >= out_u.len() || i >= out_v.len() || i >= v.len() {
                break;
            }
            let u_curr = u[i];
            let v_curr = v[i];
            let diff_u = out_u[i];
            let diff_v = out_v[i];

            let (reac_u, reac_v) = reaction(u_curr, v_curr);

            out_u[i] = u_curr + dt * (diff_u + reac_u);
            out_v[i] = v_curr + dt * (diff_v + reac_v);
        }
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

        // Validate slice lengths to prevent Undefined Behavior in unsafe blocks
        assert!(v.len() >= n, "v buffer too small");
        assert!(out_u.len() >= n, "out_u buffer too small");
        assert!(out_v.len() >= n, "out_v buffer too small");

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

    fn apply_step<F>(
        &self,
        u: &[f64],
        v: &[f64],
        out_u: &mut [f64],
        out_v: &mut [f64],
        d_u: f64,
        d_v: f64,
        dt: f64,
        reaction: F,
    ) where
        F: Fn(f64, f64) -> (f64, f64),
    {
        let n = u.len();
        if n == 0 {
            return;
        }

        assert!(v.len() >= n, "v buffer too small");
        assert!(out_u.len() >= n, "out_u buffer too small");
        assert!(out_v.len() >= n, "out_v buffer too small");

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        // 1. Handle i = 0
        {
            let i = 0;
            // Safety: n > 0 checked above
            let u_curr = unsafe { *u.get_unchecked(i) };
            let v_curr = unsafe { *v.get_unchecked(i) };

            let u_prev = u_curr; // Neumann BC
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                unsafe { (*u.get_unchecked(1), *v.get_unchecked(1)) }
            } else {
                (u_curr, v_curr)
            };

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            let (reac_u, reac_v) = reaction(u_curr, v_curr);

            unsafe {
                *out_u.get_unchecked_mut(i) = u_curr + dt * (d_u * lap_u + reac_u);
                *out_v.get_unchecked_mut(i) = v_curr + dt * (d_v * lap_v + reac_v);
            }
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
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

                    let (reac_u, reac_v) = reaction(u_curr, v_curr);

                    *out_u.get_unchecked_mut(i) = u_curr + dt * (d_u * lap_u + reac_u);
                    *out_v.get_unchecked_mut(i) = v_curr + dt * (d_v * lap_v + reac_v);

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

                let u_next = u_curr; // Neumann BC
                let v_next = v_curr;

                let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                let (reac_u, reac_v) = reaction(u_curr, v_curr);

                *out_u.get_unchecked_mut(i) = u_curr + dt * (d_u * lap_u + reac_u);
                *out_v.get_unchecked_mut(i) = v_curr + dt * (d_v * lap_v + reac_v);
            }
        }
    }
}
