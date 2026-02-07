//! Spatial Diffusion Strategies
//!
//! This module provides strategies for computing the spatial diffusion term $D \nabla^2 u$
//! in reaction-diffusion systems.

/// Defines a strategy for computing spatial diffusion.
pub trait SpatialDiffusion {
    /// Applies the diffusion operator to the state vector.
    ///
    /// Computes $D \nabla^2 u$ and stores the result in `out`.
    ///
    /// # Arguments
    /// * `field` - Input concentration slice (u).
    /// * `out` - Output buffer for diffusion term.
    /// * `d` - Diffusion coefficient.
    fn apply(&self, field: &[f64], out: &mut [f64], d: f64);
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
    fn apply(&self, field: &[f64], out: &mut [f64], d: f64) {
        let n = field.len();
        if n == 0 {
            return;
        }

        assert!(out.len() >= n, "Output buffer too small");

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;
        let coeff = d * inv_dx_sq;

        // 1. Handle i = 0 (Left Boundary)
        {
            let u_curr = field[0];
            let u_prev = u_curr; // Neumann BC: u_{-1} = u_0 (zero flux)

            let u_next = if n > 1 { field[1] } else { u_curr };

            // Laplacian: (u_{i+1} - 2u_i + u_{i-1}) / dx^2
            let lap = u_next - 2.0 * u_curr + u_prev;
            out[0] = coeff * lap;
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            // Using windows(3) allows the compiler to optimize the loop efficiently.
            // Iterates over [u_{i-1}, u_i, u_{i+1}]
            // zip with out[1..n-1]
            for (window, out_val) in field.windows(3).zip(out.iter_mut().skip(1)) {
                let u_prev = window[0];
                let u_curr = window[1];
                let u_next = window[2];

                let lap = u_next - 2.0 * u_curr + u_prev;
                *out_val = coeff * lap;
            }
        }

        // 3. Handle i = n-1 (Right Boundary)
        if n > 1 {
            let i = n - 1;
            let u_curr = field[i];
            let u_prev = field[i - 1];

            let u_next = u_curr; // Neumann BC: u_{N} = u_{N-1} (zero flux)

            let lap = u_next - 2.0 * u_curr + u_prev;
            out[i] = coeff * lap;
        }
    }
}
