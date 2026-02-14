use super::linear_algebra::{Matrix, Scalar, Vector};
use super::probability::softmax;
use std::collections::HashMap;

/// Mean Squared Error (MSE) Loss function.
/// Used primarily for Regression.
///
/// J(\theta) = \frac{1}{n} \sum_{i=1}^{n} (y^{(i)} - \hat{y}^{(i)})^2
pub fn mean_squared_error(y_pred: &Vector, y_true: &Vector) -> Scalar {
    let diff = y_pred - y_true;
    diff.dot(&diff) / (y_pred.len() as f64)
}

/// Derivative of MSE with respect to y_pred.
/// \frac{\partial J}{\partial \hat{y}} = \frac{2}{n} (\hat{y} - y)
pub fn mse_prime(y_pred: &Vector, y_true: &Vector) -> Vector {
    let n = y_pred.len() as f64;
    (y_pred - y_true) * (2.0 / n)
}

/// Cross-Entropy Loss function.
/// Used primarily for Classification.
///
/// J(\theta) = - \sum_{i} y_i \log(\hat{y}_i)
///
/// Note: This implementation assumes y_true is a one-hot vector or probability distribution.
pub fn cross_entropy_loss(y_pred: &Vector, y_true: &Vector) -> Scalar {
    // Add a small epsilon to avoid log(0)
    let epsilon = 1e-15;
    let y_pred_safe = y_pred.map(|v| v.max(epsilon));

    let log_likelihood = y_pred_safe.map(|v| v.ln());
    -(y_true.dot(&log_likelihood))
}

/// Derivative of Cross-Entropy Loss combined with Softmax.
///
/// If output layer is Softmax and Loss is Cross-Entropy, the gradient w.r.t the logits z is:
/// \frac{\partial L}{\partial z} = \hat{y} - y
///
/// This is a very elegant result that simplifies backpropagation.
pub fn cross_entropy_softmax_prime(z_logits: &Vector, y_true: &Vector) -> Vector {
    let y_pred = softmax(z_logits);
    y_pred - y_true
}

/// Common interface for optimization algorithms.
pub trait Optimizer {
    /// Updates the weights and bias for a specific layer.
    fn update(
        &mut self,
        layer_id: usize,
        weights: &mut Matrix,
        bias: &mut Vector,
        grad_w: &Matrix,
        grad_b: &Vector,
    );
}

/// Gradient Descent Update Rule.
/// \theta = \theta - \alpha \nabla_{\theta} J
///
/// Stochastic Gradient Descent (SGD)
pub struct SGD {
    pub learning_rate: f64,
}

impl SGD {
    pub fn new(learning_rate: f64) -> Self {
        Self { learning_rate }
    }

    pub fn update_vector(&self, param: &mut Vector, grad: &Vector) {
        *param -= grad * self.learning_rate;
    }

    pub fn update_matrix(&self, param: &mut Matrix, grad: &Matrix) {
        *param -= grad * self.learning_rate;
    }
}

impl Optimizer for SGD {
    fn update(
        &mut self,
        _layer_id: usize,
        weights: &mut Matrix,
        bias: &mut Vector,
        grad_w: &Matrix,
        grad_b: &Vector,
    ) {
        self.update_matrix(weights, grad_w);
        self.update_vector(bias, grad_b);
    }
}

/// Internal state for Adam optimizer for a single layer.
struct AdamLayerState {
    m_w: Matrix,
    v_w: Matrix,
    m_b: Vector,
    v_b: Vector,
    t: i32,
}

impl AdamLayerState {
    fn new(shape_w: (usize, usize), shape_b: usize) -> Self {
        Self {
            m_w: Matrix::zeros(shape_w.0, shape_w.1),
            v_w: Matrix::zeros(shape_w.0, shape_w.1),
            m_b: Vector::zeros(shape_b),
            v_b: Vector::zeros(shape_b),
            t: 0,
        }
    }

    fn step(
        &mut self,
        weights: &mut Matrix,
        bias: &mut Vector,
        grad_w: &Matrix,
        grad_b: &Vector,
        lr: f64,
        beta1: f64,
        beta2: f64,
        epsilon: f64,
    ) {
        self.t += 1;
        let t = self.t as f64;

        // Update biased first moment estimate
        self.m_w = beta1 * &self.m_w + (1.0 - beta1) * grad_w;
        self.m_b = beta1 * &self.m_b + (1.0 - beta1) * grad_b;

        // Update biased second raw moment estimate
        let grad_w_sq = grad_w.map(|g| g * g);
        let grad_b_sq = grad_b.map(|g| g * g);

        self.v_w = beta2 * &self.v_w + (1.0 - beta2) * grad_w_sq;
        self.v_b = beta2 * &self.v_b + (1.0 - beta2) * grad_b_sq;

        // Compute bias-corrected first moment estimate
        let m_hat_w = &self.m_w / (1.0 - beta1.powf(t));
        let m_hat_b = &self.m_b / (1.0 - beta1.powf(t));

        // Compute bias-corrected second raw moment estimate
        let v_hat_w = &self.v_w / (1.0 - beta2.powf(t));
        let v_hat_b = &self.v_b / (1.0 - beta2.powf(t));

        // Update parameters
        let update_w = m_hat_w.component_div(&v_hat_w.map(|v| v.sqrt() + epsilon));
        let update_b = m_hat_b.component_div(&v_hat_b.map(|v| v.sqrt() + epsilon));

        *weights -= update_w * lr;
        *bias -= update_b * lr;
    }
}

/// Adam Optimizer.
/// Adaptive Moment Estimation.
pub struct Adam {
    learning_rate: f64,
    beta1: f64,
    beta2: f64,
    epsilon: f64,
    states: HashMap<usize, AdamLayerState>,
}

impl Adam {
    pub fn new(lr: f64) -> Self {
        Self {
            learning_rate: lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            states: HashMap::new(),
        }
    }
}

impl Optimizer for Adam {
    fn update(
        &mut self,
        layer_id: usize,
        weights: &mut Matrix,
        bias: &mut Vector,
        grad_w: &Matrix,
        grad_b: &Vector,
    ) {
        let state = self.states.entry(layer_id).or_insert_with(|| {
            let shape_w = weights.shape();
            let shape_b = bias.len();
            AdamLayerState::new(shape_w, shape_b)
        });

        state.step(
            weights,
            bias,
            grad_w,
            grad_b,
            self.learning_rate,
            self.beta1,
            self.beta2,
            self.epsilon,
        );
    }
}
