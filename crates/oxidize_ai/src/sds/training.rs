use crate::AIError;
use crate::sds::rendering::{NeRFModel, RayBundle};
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

/// Module 5.2: Optimizer Step
/// Input: Weights theta, Gradients nabla_theta, Learning Rate eta.
/// Operation: Update weights (e.g., using Adam optimizer).
/// Output: Updated NeRF Model.
pub trait Optimizer {
    /// Performs a single optimization step.
    ///
    /// # Arguments
    /// * `params` - The current parameters.
    /// * `grads` - The gradients with respect to the parameters.
    ///
    /// # Returns
    /// * `Ok(DMatrix<f64>)` - The updated parameters.
    /// * `Err(AIError)` - If dimensions mismatch.
    fn step(
        &mut self,
        params: &DMatrix<f64>,
        grads: &DMatrix<f64>,
    ) -> Result<DMatrix<f64>, AIError>;
}

/// Simplified Adam implementation for a single parameter tensor (e.g., NeRF weights).
/// theta_{t+1} = theta_t - eta * m_t / (sqrt(v_t) + epsilon)
pub struct AdamOptimizer {
    pub learning_rate: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub epsilon: f64,
    pub m: Option<DMatrix<f64>>,
    pub v: Option<DMatrix<f64>>,
    pub t: usize,
}

impl AdamOptimizer {
    pub fn new(learning_rate: f64) -> Self {
        Self {
            learning_rate,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            m: None,
            v: None,
            t: 0,
        }
    }
}

impl Optimizer for AdamOptimizer {
    fn step(
        &mut self,
        params: &DMatrix<f64>,
        grads: &DMatrix<f64>,
    ) -> Result<DMatrix<f64>, AIError> {
        if params.shape() != grads.shape() {
            return Err(AIError::DimensionMismatch {
                expected: format!("{:?}", params.shape()),
                got: format!("{:?}", grads.shape()),
            });
        }

        self.t += 1;

        // Initialize state if needed
        if self.m.is_none() {
            self.m = Some(DMatrix::zeros(params.nrows(), params.ncols()));
            self.v = Some(DMatrix::zeros(params.nrows(), params.ncols()));
        }

        let m = self.m.as_mut().ok_or(AIError::UninitializedState {
            name: "m".to_string(),
        })?;
        let v = self.v.as_mut().ok_or(AIError::UninitializedState {
            name: "v".to_string(),
        })?;

        // Update biased first moment estimate: m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
        *m = &*m * self.beta1 + grads * (1.0 - self.beta1);

        // Update biased second raw moment estimate: v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
        // Element-wise square of gradients
        let grads_sq = grads.map(|x| x * x);
        *v = &*v * self.beta2 + grads_sq * (1.0 - self.beta2);

        // Compute bias-corrected first moment estimate
        let m_hat = &*m / (1.0 - self.beta1.powi(self.t as i32));

        // Compute bias-corrected second raw moment estimate
        let v_hat = &*v / (1.0 - self.beta2.powi(self.t as i32));

        // Update parameters: theta = theta - lr * m_hat / (sqrt(v_hat) + epsilon)
        let update_term = m_hat.component_div(&v_hat.map(|x| x.sqrt() + self.epsilon));

        Ok(params - update_term * self.learning_rate)
    }
}

/// Stochastic Gradient Descent with Momentum.
/// v_{t+1} = momentum * v_t + g_t
/// theta_{t+1} = theta_t - lr * v_{t+1}
pub struct SgdOptimizer {
    pub learning_rate: f64,
    pub momentum: f64,
    pub velocity: Option<DMatrix<f64>>,
}

impl SgdOptimizer {
    pub fn new(learning_rate: f64, momentum: f64) -> Self {
        Self {
            learning_rate,
            momentum,
            velocity: None,
        }
    }
}

impl Optimizer for SgdOptimizer {
    fn step(
        &mut self,
        params: &DMatrix<f64>,
        grads: &DMatrix<f64>,
    ) -> Result<DMatrix<f64>, AIError> {
        if params.shape() != grads.shape() {
            return Err(AIError::DimensionMismatch {
                expected: format!("{:?}", params.shape()),
                got: format!("{:?}", grads.shape()),
            });
        }

        // Initialize state if needed
        if self.velocity.is_none() {
            self.velocity = Some(DMatrix::zeros(params.nrows(), params.ncols()));
        }

        let v = self.velocity.as_mut().ok_or(AIError::UninitializedState {
            name: "velocity".to_string(),
        })?;

        // Update velocity: v_{t+1} = momentum * v_t + g_t
        // Note: Some implementations use v_{t+1} = momentum * v_t - lr * g_t.
        // We stick to the additive accumulation of gradients, then subtract.
        // v = mu * v + g
        *v = &*v * self.momentum + grads;

        // Update parameters: theta = theta - lr * v
        Ok(params - &*v * self.learning_rate)
    }
}

#[cfg(test)]
#[path = "tests_training.rs"]
mod tests;
