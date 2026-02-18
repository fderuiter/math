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

    /// Applies the diffusion operator and reaction kinetics in a fused loop.
    ///
    /// This method is intended for performance optimization, allowing the diffusion
    /// and reaction terms to be computed in a single pass over the data.
    ///
    /// # Arguments
    /// * `u` - Input activator concentration slice.
    /// * `v` - Input inhibitor concentration slice.
    /// * `out_u` - Output buffer for activator state (u + dt * du/dt).
    /// * `out_v` - Output buffer for inhibitor state (v + dt * dv/dt).
    /// * `d_u` - Diffusion coefficient for u.
    /// * `d_v` - Diffusion coefficient for v.
    /// * `dt` - Time step.
    /// * `reaction` - Closure that computes the reaction rates (du/dt, dv/dt) given (u, v).
    ///
    /// # Example
    ///
    /// ```
    /// use math_explorer::biology::diffusion::{FiniteDifference1D, SpatialDiffusion};
    ///
    /// let diff = FiniteDifference1D::new(0.1);
    /// let u = vec![1.0; 10];
    /// let v = vec![0.5; 10];
    /// let mut out_u = vec![0.0; 10];
    /// let mut out_v = vec![0.0; 10];
    ///
    /// // Fused update: du/dt = D*lap(u) - u, dv/dt = D*lap(v)
    /// diff.apply_step(
    ///     &u, &v,
    ///     &mut out_u, &mut out_v,
    ///     0.1, 0.1,  // d_u, d_v
    ///     0.01,      // dt
    ///     |u, _v| (-u, 0.0) // reaction closure
    /// );
    /// ```
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
        // Default implementation: calculate diffusion then apply reaction
        self.apply(u, v, out_u, out_v, d_u, d_v);

        let n = u.len();
        // Safety: we assume out buffers are sized correctly if apply succeeded,
        // but let's be safe with bounds checks in default impl or use unsafe if we trust apply.
        // Let's use safe indexing for default impl as it is the fallback.
        for i in 0..n {
            let diff_u = out_u[i];
            let diff_v = out_v[i];
            let (reac_u, reac_v) = reaction(u[i], v[i]);
            out_u[i] = u[i] + dt * (diff_u + reac_u);
            out_v[i] = v[i] + dt * (diff_v + reac_v);
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

        apply_1d_stencil(u, out_u, d_u, inv_dx_sq);
        apply_1d_stencil(v, out_v, d_v, inv_dx_sq);
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

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            let (reac_u, reac_v) = reaction(u_curr, v_curr);

            out_u[0] = u_curr + dt * (d_u * lap_u + reac_u);
            out_v[0] = v_curr + dt * (d_v * lap_v + reac_v);
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

                let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                let (reac_u, reac_v) = reaction(u_curr, v_curr);

                *o_u = u_curr + dt * (d_u * lap_u + reac_u);
                *o_v = v_curr + dt * (d_v * lap_v + reac_v);
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

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            let (reac_u, reac_v) = reaction(u_curr, v_curr);

            out_u[i] = u_curr + dt * (d_u * lap_u + reac_u);
            out_v[i] = v_curr + dt * (d_v * lap_v + reac_v);
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
    let n = src.len();
    if n == 0 {
        return;
    }

    // 1. Left Boundary (i=0)
    {
        let u_curr = src[0];
        let u_prev = u_curr; // Neumann: u_{-1} = u_0
        let u_next = if n > 1 { src[1] } else { u_curr };
        dst[0] = d * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
    }

    // 2. Interior (Optimized with windows iterator)
    if n > 2 {
        for (win, out) in src.windows(3).zip(dst.iter_mut().skip(1)) {
            let u_prev = win[0];
            let u_curr = win[1];
            let u_next = win[2];
            *out = d * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
        }
    }

    // 3. Right Boundary (i=n-1)
    if n > 1 {
        let i = n - 1;
        let u_curr = src[i];
        let u_prev = src[i - 1];
        let u_next = u_curr; // Neumann: u_{N} = u_{N-1}
        dst[i] = d * (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
    }
}
