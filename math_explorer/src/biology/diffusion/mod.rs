//! Spatial Diffusion Strategies
//!
//! This module provides strategies for computing the spatial diffusion term $D \nabla^2 u$
//! in reaction-diffusion systems.
//!
//! ## Performance Optimization
//!
//! Implementations like `FiniteDifference2D` use **Loop Splitting** to separate the "hot path" (interior points)
//! from boundary handling. This allows the compiler to:
//! 1. Vectorize the interior loop without conditional checks.
//! 2. Unroll loops for better instruction pipelining.
//! 3. Use `unsafe` indexing (with rigorous safety proofs) to eliminate bounds checks.

pub mod fd1d;
pub mod fd2d;

pub use fd1d::FiniteDifference1D;
pub use fd2d::FiniteDifference2D;

/// Defines a strategy for computing spatial diffusion.
pub trait SpatialDiffusion<const N: usize> {
    /// Computes diffusion terms for each point and calls the closure.
    /// Internal iteration allows for optimization (loop fusion, SIMD).
    ///
    /// The closure `op` is called with `(index, current_vals, diff_vals)` where:
    /// * `index`: The linear index of the point.
    /// * `current_vals`: Current values of species at index.
    /// * `diff_vals`: The diffusion terms ($D \nabla^2 u$).
    fn map_diffusion<F>(&self, state: [&[f64]; N], coeffs: [f64; N], op: F)
    where
        F: FnMut(usize, [f64; N], [f64; N]);

    /// Applies the diffusion operator to the state vectors.
    ///
    /// Computes $D \nabla^2 u$ and stores the result in `out`.
    ///
    /// # Arguments
    /// * `state` - Input concentration slices.
    /// * `out` - Output buffers for diffusion terms.
    /// * `coeffs` - Diffusion coefficients.
    fn apply(&self, state: [&[f64]; N], out: [&mut [f64]; N], coeffs: [f64; N]) {
        // Default implementation: calculate diffusion and write to buffer
        self.map_diffusion(state, coeffs, |i, _, diffs| {
            for s in 0..N {
                if i < out[s].len() {
                    out[s][i] = diffs[s];
                }
            }
        });
    }
}
