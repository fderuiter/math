//! Optimization Algorithms.
//!
//! Provides structures and traits for mathematical optimization problems.

use nalgebra::{DMatrix, DVector, RealField};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during optimization.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum OptimizationError {
    #[error("Failed to convert numerical value")]
    ConversionError,
}

/// Represents an L1 Norm-Regularized Least Squares problem.
///
/// $$ J(x) = \frac{1}{2} \| y - z(Wx) \|^2_2 + \lambda \| x \|_1 $$
///
/// Note: This struct is a placeholder for the objective function definition.
/// Solving L1 regularized problems (Lasso) typically requires iterative solvers like ISTA or FISTA,
/// which are beyond the scope of a simple formula function.
/// We provide the cost function evaluation.
pub struct L1RegularizedLeastSquares {
    lambda: f64,
}

impl L1RegularizedLeastSquares {
    #[verified_engine::verified]
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }

    /// Evaluates the cost function $J(x)$.
    ///
    /// Assuming simplified linear model $z(Wx) \approx Ax$.
    #[verified_engine::verified]
    pub fn cost(&self, a: &DMatrix<f64>, x: &DVector<f64>, y: &DVector<f64>) -> f64 {
        let residual = y - (a * x);
        let l2_term = 0.5 * residual.norm_squared();
        let l1_term = x.iter().map(|v| v.abs()).sum::<f64>();

        l2_term + self.lambda * l1_term
    }
}

/// Strategy for updating parameters.
///
/// This trait is generic over the field type `T` (e.g., `f32`, `f64`) and uses a generic `Key`
/// to identify parameters, allowing it to be used for neural networks, regression, or other optimization tasks.
///
/// # Generics
/// * `T`: The numeric field (must be `RealField` + `Copy`).
/// * `Key`: A unique identifier for the parameter being updated (e.g., `u64`, `String`, or a specialized Enum).
pub trait Optimizer<T: RealField + Copy, Key = u64> {
    /// Updates a matrix parameter.
    #[verified_engine::verified]
    fn update_matrix(
        &mut self,
        key: Key,
        param: &mut DMatrix<T>,
        grad: &DMatrix<T>,
    ) -> Result<(), OptimizationError>;
    /// Updates a vector parameter.
    #[verified_engine::verified]
    fn update_vector(
        &mut self,
        key: Key,
        param: &mut DVector<T>,
        grad: &DVector<T>,
    ) -> Result<(), OptimizationError>;
}

/// Gradient Descent Update Rule.
/// \theta = \theta - \alpha \nabla_{\theta} J
///
/// Stochastic Gradient Descent (SGD)
pub struct SGD<T> {
    pub learning_rate: T,
}

impl<T: RealField + Copy> SGD<T> {
    #[verified_engine::verified]
    pub fn new(learning_rate: T) -> Self {
        Self { learning_rate }
    }
}

impl<T: RealField + Copy, Key> Optimizer<T, Key> for SGD<T> {
    #[verified_engine::verified]
    fn update_vector(
        &mut self,
        _key: Key,
        param: &mut DVector<T>,
        grad: &DVector<T>,
    ) -> Result<(), OptimizationError> {
        *param -= grad * self.learning_rate;
        Ok(())
    }

    #[verified_engine::verified]
    fn update_matrix(
        &mut self,
        _key: Key,
        param: &mut DMatrix<T>,
        grad: &DMatrix<T>,
    ) -> Result<(), OptimizationError> {
        *param -= grad * self.learning_rate;
        Ok(())
    }
}

/// Internal state for Adam optimizer.
/// Stores the first (m) and second (v) moment estimates.
struct AdamState<T> {
    m: DMatrix<T>, // First moment estimate (biased)
    v: DMatrix<T>, // Second raw moment estimate (biased)
    t: i32,        // Time step
}

/// Adam Optimizer.
/// Adaptive Moment Estimation.
pub struct Adam<T, Key>
where
    Key: Eq + std::hash::Hash,
{
    learning_rate: T,
    beta1: T,
    beta2: T,
    epsilon: T,
    /// State keyed by the unique parameter identifier.
    states: HashMap<Key, AdamState<T>>,
}

impl<T: RealField + Copy, Key> Adam<T, Key>
where
    Key: Eq + std::hash::Hash + Clone,
{
    #[verified_engine::verified]
    pub fn new(lr: T) -> Result<Self, OptimizationError> {
        Ok(Self {
            learning_rate: lr,
            beta1: T::from_f64(0.9).ok_or(OptimizationError::ConversionError)?,
            beta2: T::from_f64(0.999).ok_or(OptimizationError::ConversionError)?,
            epsilon: T::from_f64(1e-8).ok_or(OptimizationError::ConversionError)?,
            states: HashMap::new(),
        })
    }

    #[verified_engine::verified]
    fn get_state(&mut self, key: Key, shape: (usize, usize)) -> &mut AdamState<T> {
        self.states.entry(key).or_insert_with(|| AdamState {
            m: DMatrix::zeros(shape.0, shape.1),
            v: DMatrix::zeros(shape.0, shape.1),
            t: 0,
        })
    }
}

impl<T: RealField + Copy, Key> Optimizer<T, Key> for Adam<T, Key>
where
    Key: Eq + std::hash::Hash + Clone,
{
    #[verified_engine::verified]
    fn update_matrix(
        &mut self,
        key: Key,
        param: &mut DMatrix<T>,
        grad: &DMatrix<T>,
    ) -> Result<(), OptimizationError> {
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;
        let one = T::one();

        let state = self.get_state(key, (param.nrows(), param.ncols()));

        state.t += 1;
        let t_val = T::from_i32(state.t).ok_or(OptimizationError::ConversionError)?;

        // Update biased first moment estimate
        // m = beta1 * m + (1 - beta1) * grad
        state.m = &state.m * beta1 + grad * (one - beta1);

        // Update biased second raw moment estimate
        // v = beta2 * v + (1 - beta2) * grad^2
        let grad_sq = grad.map(|g| g * g);
        state.v = &state.v * beta2 + grad_sq * (one - beta2);

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
        Ok(())
    }

    #[verified_engine::verified]
    fn update_vector(
        &mut self,
        key: Key,
        param: &mut DVector<T>,
        grad: &DVector<T>,
    ) -> Result<(), OptimizationError> {
        let beta1 = self.beta1;
        let beta2 = self.beta2;
        let epsilon = self.epsilon;
        let lr = self.learning_rate;
        let one = T::one();

        let rows = param.len();
        let cols = 1;
        // Map vectors to matrices (Mx1) internally for state storage
        let state = self.get_state(key, (rows, cols));

        state.t += 1;
        let t_val = T::from_i32(state.t).ok_or(OptimizationError::ConversionError)?;

        // Convert grad to DMatrix for consistent operations with state
        let grad_mat = DMatrix::from_column_slice(rows, cols, grad.as_slice());

        state.m = &state.m * beta1 + &grad_mat * (one - beta1);

        let grad_sq = grad_mat.map(|g| g * g);
        state.v = &state.v * beta2 + grad_sq * (one - beta2);

        let m_hat = &state.m / (one - beta1.powf(t_val));
        let v_hat = &state.v / (one - beta2.powf(t_val));

        let update_mat = m_hat.component_div(&v_hat.map(|v| v.sqrt() + epsilon));

        // Convert update back to Vector
        let update_vec = DVector::from_column_slice(update_mat.as_slice());
        *param -= update_vec * lr;
        Ok(())
    }
}
