//! Computer Vision and Image Analysis.

use nalgebra::{DMatrix, DVector};

/// Optical Flow Intensity Conservation (Horn-Schunck constraint).
///
/// $$ v = -\nabla I \frac{\partial_V I}{|\nabla I|^2} $$
///
/// This formula computes the normal optical flow velocity component.
///
/// # Arguments
/// * `grad_x` - Gradient in x ($\partial I / \partial x$).
/// * `grad_y` - Gradient in y ($\partial I / \partial y$).
/// * `grad_t` - Temporal gradient ($\partial I / \partial t$).
///
/// # Returns
/// * `(vx, vy)` - The velocity vector component normal to the edge.
pub fn optical_flow_normal_velocity(grad_x: f64, grad_y: f64, grad_t: f64) -> (f64, f64) {
    let norm_sq = grad_x * grad_x + grad_y * grad_y;
    if norm_sq < 1e-9 {
        return (0.0, 0.0);
    }

    let factor = -grad_t / norm_sq;
    (grad_x * factor, grad_y * factor)
}
