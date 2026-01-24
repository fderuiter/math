use crate::ai::sds::rendering::{NeRFModel, RayBundle};
use nalgebra::DMatrix;

/// Module 5.1: Jacobian-Vector Product
/// Input: SDS Gradient delta_SDS, Rendered Image x_render, NeRF Weights theta.
/// Operation: We need dL/dtheta. By chain rule, this is delta_SDS * dx_render/dtheta.
/// Modern autodiff engines (PyTorch) handle this by calling .backward() on the rendered image with delta_SDS as the incoming gradient.
/// Output: Gradient accumulator for theta.
pub trait DifferentiableNeRF: NeRFModel {
    /// Computes the gradient of the loss with respect to the model parameters,
    /// given the gradient of the loss with respect to the output image.
    /// This corresponds to the `backward` pass in an autodiff framework.
    ///
    /// # Arguments
    /// * `bundle` - The ray bundle used for the forward pass.
    /// * `image_grad` - The gradient of the loss w.r.t the rendered image (delta_SDS).
    ///
    /// # Returns
    /// * `DMatrix<f64>` - The gradient vector for the model parameters (theta).
    fn backward(&self, bundle: &RayBundle, image_grad: &DMatrix<f64>) -> DMatrix<f64>;
}

// Re-export Optimizer trait and AdamOptimizer for backward compatibility.
pub use crate::ai::optimizers::{AdamOptimizer, Optimizer};

#[cfg(test)]
#[path = "tests_training.rs"]
mod tests;
