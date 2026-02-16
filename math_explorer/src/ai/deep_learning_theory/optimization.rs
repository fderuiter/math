use super::linear_algebra::{Matrix, Scalar, Vector};
use super::probability::softmax;
use nalgebra::DMatrix;
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

/// Identifies the type of parameter being updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamType {
    Weight,
    Bias,
}

/// Strategy for updating model parameters.
pub trait Optimizer {
    fn update_matrix(&mut self, layer_idx: usize, param: &mut Matrix, grad: &Matrix);
    fn update_vector(&mut self, layer_idx: usize, param: &mut Vector, grad: &Vector);
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
}

impl Optimizer for SGD {
    fn update_vector(&mut self, _layer_idx: usize, param: &mut Vector, grad: &Vector) {
        *param -= grad * self.learning_rate;
    }

    fn update_matrix(&mut self, _layer_idx: usize, param: &mut Matrix, grad: &Matrix) {
        *param -= grad * self.learning_rate;
    }
}

/// Internal state for Adam optimizer (per parameter tensor).
struct AdamState {
    m: Matrix, // First moment estimate (biased)
    v: Matrix, // Second raw moment estimate (biased)
    t: i32,    // Time step
}

/// Adam Optimizer.
/// Adaptive Moment Estimation.
///
/// This implementation uses lazy state initialization, allowing it to adapt to any
/// network architecture dynamically. State is stored per (layer_index, parameter_type).
pub struct Adam {
    learning_rate: f64,
    beta1: f64,
    beta2: f64,
    epsilon: f64,
    /// State keyed by (Layer Index, Parameter Type)
    states: HashMap<(usize, ParamType), AdamState>,
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

    fn get_state(&mut self, key: (usize, ParamType), shape: (usize, usize)) -> &mut AdamState {
        self.states.entry(key).or_insert_with(|| AdamState {
            m: Matrix::zeros(shape.0, shape.1),
            v: Matrix::zeros(shape.0, shape.1),
            t: 0,
        })
    }
}

impl Optimizer for Adam {
    fn update_matrix(&mut self, layer_idx: usize, param: &mut Matrix, grad: &Matrix) {
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;

        let state = self.get_state((layer_idx, ParamType::Weight), (param.nrows(), param.ncols()));

        state.t += 1;
        let t = state.t as f64;

        // Update biased first moment estimate
        state.m = beta1 * &state.m + (1.0 - beta1) * grad;

        // Update biased second raw moment estimate
        let grad_sq = grad.map(|g| g * g);
        state.v = beta2 * &state.v + (1.0 - beta2) * grad_sq;

        // Compute bias-corrected first moment estimate
        let m_hat = &state.m / (1.0 - beta1.powf(t));

        // Compute bias-corrected second raw moment estimate
        let v_hat = &state.v / (1.0 - beta2.powf(t));

        // Update parameters
        let update = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));
        *param -= update * lr;
    }

    fn update_vector(&mut self, layer_idx: usize, param: &mut Vector, grad: &Vector) {
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;

        let rows = param.len();
        let cols = 1;
        let state = self.get_state((layer_idx, ParamType::Bias), (rows, cols));

        state.t += 1;
        let t = state.t as f64;

        // Convert grad to DMatrix for consistent operations with state
        let grad_mat = DMatrix::from_column_slice(rows, cols, grad.as_slice());

        state.m = beta1 * &state.m + (1.0 - beta1) * &grad_mat;

        let grad_sq = grad_mat.map(|g| g * g);
        state.v = beta2 * &state.v + (1.0 - beta2) * grad_sq;

        let m_hat = &state.m / (1.0 - beta1.powf(t));
        let v_hat = &state.v / (1.0 - beta2.powf(t));

        let update_mat = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));

        // Convert update back to Vector
        let update_vec = Vector::from_column_slice(update_mat.as_slice());
        *param -= update_vec * lr;
    }
}
