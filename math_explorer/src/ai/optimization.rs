use crate::pure_math::optimization::{self, Optimizer as CoreOptimizer};
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
///
/// This trait is specialized for Layer-based Neural Networks.
/// It delegates the actual math to `pure_math::optimization`.
pub trait Optimizer<T: RealField + Copy> {
    fn update_matrix(&mut self, layer_idx: usize, param: &mut DMatrix<T>, grad: &DMatrix<T>);
    fn update_vector(&mut self, layer_idx: usize, param: &mut DVector<T>, grad: &DVector<T>);
}

/// Gradient Descent Update Rule.
/// \theta = \theta - \alpha \nabla_{\theta} J
///
/// Stochastic Gradient Descent (SGD)
pub struct SGD<T> {
    inner: optimization::SGD<T>,
}

impl<T: Copy> SGD<T> {
    pub fn new(learning_rate: T) -> Self {
        Self {
            inner: optimization::SGD::new(learning_rate),
        }
    }
}

impl<T: RealField + Copy> Optimizer<T> for SGD<T> {
    fn update_vector(&mut self, _layer_idx: usize, param: &mut DVector<T>, grad: &DVector<T>) {
        self.inner
            .update(param, grad)
            .expect("SGD Vector Update Failed");
    }

    fn update_matrix(&mut self, _layer_idx: usize, param: &mut DMatrix<T>, grad: &DMatrix<T>) {
        self.inner
            .update(param, grad)
            .expect("SGD Matrix Update Failed");
    }
}

/// Adam Optimizer.
/// Adaptive Moment Estimation.
///
/// Manages a collection of `pure_math::optimization::Adam` instances, one for each
/// parameter tensor (Weights/Biases) in each layer.
pub struct Adam<T: RealField + Copy> {
    learning_rate: T,
    /// State keyed by (Layer Index, Parameter Type)
    optimizers: HashMap<(usize, ParamType), optimization::Adam<T>>,
}

impl<T: RealField + Copy> Adam<T> {
    pub fn new(lr: T) -> Self {
        Self {
            learning_rate: lr,
            optimizers: HashMap::new(),
        }
    }

    fn get_optimizer(&mut self, key: (usize, ParamType)) -> &mut optimization::Adam<T> {
        self.optimizers.entry(key).or_insert_with(|| {
            optimization::Adam::new(self.learning_rate)
        })
    }
}

impl<T: RealField + Copy> Optimizer<T> for Adam<T> {
    fn update_matrix(&mut self, layer_idx: usize, param: &mut DMatrix<T>, grad: &DMatrix<T>) {
        let opt = self.get_optimizer((layer_idx, ParamType::Weight));
        opt.update(param, grad).expect("Adam Matrix Update Failed");
    }

    fn update_vector(&mut self, layer_idx: usize, param: &mut DVector<T>, grad: &DVector<T>) {
        let opt = self.get_optimizer((layer_idx, ParamType::Bias));
        opt.update(param, grad).expect("Adam Vector Update Failed");
    }
}
