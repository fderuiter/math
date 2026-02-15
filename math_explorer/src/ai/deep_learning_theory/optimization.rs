use super::linear_algebra::{Matrix, Scalar, Vector};
use super::probability::softmax;
use std::collections::HashMap;

/// Identifies the type of parameter being updated.
/// Necessary for stateful optimizers (like Adam) to track momentum for specific parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamType {
    Weight,
    Bias,
}

/// A common interface for optimization algorithms.
/// Allows swapping SGD, Adam, etc., without modifying the training loop.
pub trait Optimizer {
    fn update_vector(
        &mut self,
        layer_idx: usize,
        param_type: ParamType,
        param: &mut Vector,
        grad: &Vector,
    );
    fn update_matrix(
        &mut self,
        layer_idx: usize,
        param_type: ParamType,
        param: &mut Matrix,
        grad: &Matrix,
    );
}

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
    fn update_vector(
        &mut self,
        _layer_idx: usize,
        _param_type: ParamType,
        param: &mut Vector,
        grad: &Vector,
    ) {
        *param -= grad * self.learning_rate;
    }

    fn update_matrix(
        &mut self,
        _layer_idx: usize,
        _param_type: ParamType,
        param: &mut Matrix,
        grad: &Matrix,
    ) {
        *param -= grad * self.learning_rate;
    }
}

/// Internal state for Adam optimizer per parameter.
struct AdamState<T> {
    m: T,   // First moment
    v: T,   // Second moment
    t: i32, // Time step for this parameter
}

/// Adam Optimizer.
/// Adaptive Moment Estimation.
pub struct Adam {
    learning_rate: f64,
    beta1: f64,
    beta2: f64,
    epsilon: f64,
    // We store state for Vectors (biases) and Matrices (weights) separately
    // Key is (layer_idx, ParamType)
    vector_states: HashMap<(usize, ParamType), AdamState<Vector>>,
    matrix_states: HashMap<(usize, ParamType), AdamState<Matrix>>,
}

impl Adam {
    pub fn new(lr: f64) -> Self {
        Self {
            learning_rate: lr,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            vector_states: HashMap::new(),
            matrix_states: HashMap::new(),
        }
    }
}

impl Optimizer for Adam {
    fn update_vector(
        &mut self,
        layer_idx: usize,
        param_type: ParamType,
        param: &mut Vector,
        grad: &Vector,
    ) {
        let key = (layer_idx, param_type);
        let state = self.vector_states.entry(key).or_insert_with(|| AdamState {
            m: Vector::zeros(param.len()),
            v: Vector::zeros(param.len()),
            t: 0,
        });

        state.t += 1;
        let t = state.t as f64;

        // Update biased first moment estimate
        // m_t = beta1 * m_{t-1} + (1 - beta1) * g_t
        state.m = &state.m * self.beta1 + grad * (1.0 - self.beta1);

        // Update biased second raw moment estimate
        // v_t = beta2 * v_{t-1} + (1 - beta2) * g_t^2
        let grad_sq = grad.map(|g| g * g);
        state.v = &state.v * self.beta2 + grad_sq * (1.0 - self.beta2);

        // Compute bias-corrected estimates
        let m_hat = &state.m / (1.0 - self.beta1.powf(t));
        let v_hat = &state.v / (1.0 - self.beta2.powf(t));

        // Update parameters
        let update = m_hat.component_div(&v_hat.map(|v| v.sqrt() + self.epsilon));
        *param -= update * self.learning_rate;
    }

    fn update_matrix(
        &mut self,
        layer_idx: usize,
        param_type: ParamType,
        param: &mut Matrix,
        grad: &Matrix,
    ) {
        let key = (layer_idx, param_type);
        let state = self.matrix_states.entry(key).or_insert_with(|| AdamState {
            m: Matrix::zeros(param.nrows(), param.ncols()),
            v: Matrix::zeros(param.nrows(), param.ncols()),
            t: 0,
        });

        state.t += 1;
        let t = state.t as f64;

        // Update biased first moment estimate
        state.m = &state.m * self.beta1 + grad * (1.0 - self.beta1);

        // Update biased second raw moment estimate
        let grad_sq = grad.map(|g| g * g);
        state.v = &state.v * self.beta2 + grad_sq * (1.0 - self.beta2);

        // Compute bias-corrected estimates
        let m_hat = &state.m / (1.0 - self.beta1.powf(t));
        let v_hat = &state.v / (1.0 - self.beta2.powf(t));

        // Update parameters
        let update = m_hat.component_div(&v_hat.map(|v| v.sqrt() + self.epsilon));
        *param -= update * self.learning_rate;
    }
}
