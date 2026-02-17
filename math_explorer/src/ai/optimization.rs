use nalgebra::{DMatrix, DVector, RealField};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

/// Mean Squared Error (MSE) Loss function.
/// Used primarily for Regression.
///
/// J(\theta) = \frac{1}{n} \sum_{i=1}^{n} (y^{(i)} - \hat{y}^{(i)})^2
pub fn mean_squared_error<T: RealField + Copy>(y_pred: &DVector<T>, y_true: &DVector<T>) -> T {
    let diff = y_pred - y_true;
    let n = T::from_usize(y_pred.len()).unwrap();
    diff.dot(&diff) / n
}

/// Derivative of MSE with respect to y_pred.
/// \frac{\partial J}{\partial \hat{y}} = \frac{2}{n} (\hat{y} - y)
pub fn mse_prime<T: RealField + Copy>(y_pred: &DVector<T>, y_true: &DVector<T>) -> DVector<T> {
    let n = T::from_usize(y_pred.len()).unwrap();
    let two = T::from_f64(2.0).unwrap();
    (y_pred - y_true) * (two / n)
}

/// Cross-Entropy Loss function.
/// Used primarily for Classification.
///
/// J(\theta) = - \sum_{i} y_i \log(\hat{y}_i)
///
/// Note: This implementation assumes y_true is a one-hot vector or probability distribution.
pub fn cross_entropy_loss<T: RealField + Copy>(y_pred: &DVector<T>, y_true: &DVector<T>) -> T {
    // Add a small epsilon to avoid log(0)
    let epsilon = T::from_f64(1e-15).unwrap();
    let y_pred_safe = y_pred.map(|v| if v > epsilon { v } else { epsilon });

    let log_likelihood = y_pred_safe.map(|v| v.ln());
    -(y_true.dot(&log_likelihood))
}

/// Derivative of Cross-Entropy Loss combined with Softmax.
///
/// If output layer is Softmax and Loss is Cross-Entropy, the gradient w.r.t the logits z is:
/// \frac{\partial L}{\partial z} = \hat{y} - y
pub fn cross_entropy_softmax_prime<T: RealField + Copy>(
    z_logits: &DVector<T>,
    y_true: &DVector<T>,
) -> DVector<T> {
    let y_pred = softmax(z_logits);
    y_pred - y_true
}

/// Helper Softmax function for generic types.
fn softmax<T: RealField + Copy>(z: &DVector<T>) -> DVector<T> {
    let max_z = z.max();
    let exps = z.map(|v| (v - max_z).exp());
    let sum_exps = exps.sum();
    exps / sum_exps
}

/// Identifies the type of parameter being updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParamType {
    Weight,
    Bias,
}

/// Strategy for updating model parameters.
pub trait Optimizer<T: RealField + Copy> {
    fn update_matrix(&mut self, layer_idx: usize, param: &mut DMatrix<T>, grad: &DMatrix<T>);
    fn update_vector(&mut self, layer_idx: usize, param: &mut DVector<T>, grad: &DVector<T>);
}

/// Gradient Descent Update Rule.
/// \theta = \theta - \alpha \nabla_{\theta} J
///
/// Stochastic Gradient Descent (SGD)
pub struct SGD<T> {
    pub learning_rate: T,
}

impl<T: RealField + Copy> SGD<T> {
    pub fn new(learning_rate: T) -> Self {
        Self { learning_rate }
    }
}

impl<T: RealField + Copy> Optimizer<T> for SGD<T> {
    fn update_vector(&mut self, _layer_idx: usize, param: &mut DVector<T>, grad: &DVector<T>) {
        *param -= grad.clone() * self.learning_rate;
    }

    fn update_matrix(&mut self, _layer_idx: usize, param: &mut DMatrix<T>, grad: &DMatrix<T>) {
        *param -= grad.clone() * self.learning_rate;
    }
}

/// Internal state for Adam optimizer (per parameter tensor).
struct AdamState<T> {
    m: DMatrix<T>, // First moment estimate (biased)
    v: DMatrix<T>, // Second raw moment estimate (biased)
    t: i32,        // Time step
}

/// Adam Optimizer.
/// Adaptive Moment Estimation.
pub struct Adam<T> {
    learning_rate: T,
    beta1: T,
    beta2: T,
    epsilon: T,
    /// State keyed by (Layer Index, Parameter Type)
    states: HashMap<(usize, ParamType), AdamState<T>>,
}

impl<T: RealField + Copy> Adam<T> {
    pub fn new(lr: T) -> Self {
        Self {
            learning_rate: lr,
            beta1: T::from_f64(0.9).unwrap(),
            beta2: T::from_f64(0.999).unwrap(),
            epsilon: T::from_f64(1e-8).unwrap(),
            states: HashMap::new(),
        }
    }

    fn get_state(&mut self, key: (usize, ParamType), shape: (usize, usize)) -> &mut AdamState<T> {
        self.states.entry(key).or_insert_with(|| AdamState {
            m: DMatrix::zeros(shape.0, shape.1),
            v: DMatrix::zeros(shape.0, shape.1),
            t: 0,
        })
    }
}

impl<T: RealField + Copy> Optimizer<T> for Adam<T> {
    fn update_matrix(&mut self, layer_idx: usize, param: &mut DMatrix<T>, grad: &DMatrix<T>) {
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;
        let one = T::one();

        let state = self.get_state(
            (layer_idx, ParamType::Weight),
            (param.nrows(), param.ncols()),
        );

        state.t += 1;
        let t_val = T::from_i32(state.t).unwrap();

        // Update biased first moment estimate
        // m = beta1 * m + (1 - beta1) * grad
        state.m = state.m.clone() * beta1 + grad.clone() * (one - beta1);

        // Update biased second raw moment estimate
        // v = beta2 * v + (1 - beta2) * grad^2
        let grad_sq = grad.map(|g| g * g);
        state.v = state.v.clone() * beta2 + grad_sq * (one - beta2);

        // Compute bias-corrected first moment estimate
        // m_hat = m / (1 - beta1^t)
        let m_hat = &state.m / (one - beta1.powf(t_val));

        // Compute bias-corrected second raw moment estimate
        // v_hat = v / (1 - beta2^t)
        let v_hat = &state.v / (one - beta2.powf(t_val));

        // Update parameters
        // param -= lr * m_hat / (sqrt(v_hat) + epsilon)
        let update = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));
        *param -= update * lr;
    }

    fn update_vector(&mut self, layer_idx: usize, param: &mut DVector<T>, grad: &DVector<T>) {
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;
        let one = T::one();

        let rows = param.len();
        let cols = 1;
        let state = self.get_state((layer_idx, ParamType::Bias), (rows, cols));

        state.t += 1;
        let t_val = T::from_i32(state.t).unwrap();

        // Convert grad to DMatrix for consistent operations with state
        let grad_mat = DMatrix::from_column_slice(rows, cols, grad.as_slice());

        state.m = state.m.clone() * beta1 + grad_mat.clone() * (one - beta1);

        let grad_sq = grad_mat.map(|g| g * g);
        state.v = state.v.clone() * beta2 + grad_sq * (one - beta2);

        let m_hat = &state.m / (one - beta1.powf(t_val));
        let v_hat = &state.v / (one - beta2.powf(t_val));

        let update_mat = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));

        // Convert update back to Vector
        let update_vec = DVector::from_column_slice(update_mat.as_slice());
        *param -= update_vec * lr;
    }
}
