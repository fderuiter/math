//! Optimization Algorithms.
//!
//! Provides structures and traits for mathematical optimization problems.

use nalgebra::{DMatrix, DVector, RealField};

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
    pub fn new(lambda: f64) -> Self {
        Self { lambda }
    }

    /// Evaluates the cost function $J(x)$.
    ///
    /// Assuming simplified linear model $z(Wx) \approx Ax$.
    pub fn cost(&self, a: &DMatrix<f64>, x: &DVector<f64>, y: &DVector<f64>) -> f64 {
        let residual = y - (a * x);
        let l2_term = 0.5 * residual.norm_squared();
        let l1_term = x.iter().map(|v| v.abs()).sum::<f64>();

        l2_term + self.lambda * l1_term
    }
}

/// Error types for optimization operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OptimizationError {
    /// Indicates that the parameter and gradient shapes do not match.
    DimensionMismatch { expected: String, got: String },
    /// General error message.
    Custom(String),
}

impl std::fmt::Display for OptimizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizationError::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, got)
            }
            OptimizationError::Custom(msg) => write!(f, "Optimization error: {}", msg),
        }
    }
}

impl std::error::Error for OptimizationError {}

/// A generic interface for updating parameters based on gradients.
///
/// This trait abstracts the optimization step, allowing algorithms like SGD, Adam,
/// or RMSProp to be used interchangeably.
///
/// # Type Parameters
/// * `P`: The type of the parameter being optimized (e.g., `DMatrix<f64>`, `f64`).
pub trait Optimizer<P> {
    /// Updates the parameter in-place using the provided gradient.
    ///
    /// # Arguments
    /// * `param` - Mutable reference to the parameter to update.
    /// * `grad` - The gradient with respect to the parameter.
    fn update(&mut self, param: &mut P, grad: &P) -> Result<(), OptimizationError>;
}

/// Stochastic Gradient Descent (SGD) optimizer.
///
/// Update rule: $\theta = \theta - \eta \cdot \nabla J(\theta)$
pub struct SGD<T> {
    pub learning_rate: T,
}

impl<T: Copy> SGD<T> {
    pub fn new(learning_rate: T) -> Self {
        Self { learning_rate }
    }
}

// Implement for DMatrix
impl<T: RealField + Copy> Optimizer<DMatrix<T>> for SGD<T> {
    fn update(&mut self, param: &mut DMatrix<T>, grad: &DMatrix<T>) -> Result<(), OptimizationError> {
        if param.shape() != grad.shape() {
             return Err(OptimizationError::DimensionMismatch {
                expected: format!("{:?}", param.shape()),
                got: format!("{:?}", grad.shape()),
            });
        }
        *param -= grad.clone() * self.learning_rate;
        Ok(())
    }
}

// Implement for DVector
impl<T: RealField + Copy> Optimizer<DVector<T>> for SGD<T> {
    fn update(&mut self, param: &mut DVector<T>, grad: &DVector<T>) -> Result<(), OptimizationError> {
        if param.len() != grad.len() {
             return Err(OptimizationError::DimensionMismatch {
                expected: format!("{}", param.len()),
                got: format!("{}", grad.len()),
            });
        }
        *param -= grad.clone() * self.learning_rate;
        Ok(())
    }
}

/// Adam Optimizer (Adaptive Moment Estimation).
///
/// Maintains per-parameter state (momentum $m$ and velocity $v$).
/// This struct optimizes a *single* parameter tensor. For multi-parameter models,
/// use a collection of `Adam` instances.
pub struct Adam<T: RealField + Copy> {
    pub learning_rate: T,
    pub beta1: T,
    pub beta2: T,
    pub epsilon: T,
    // We use DMatrix to store state. It can represent vectors as Nx1 matrices.
    m: Option<DMatrix<T>>,
    v: Option<DMatrix<T>>,
    t: i32,
}

impl<T: RealField + Copy> Adam<T> {
    pub fn new(learning_rate: T) -> Self {
        Self {
            learning_rate,
            beta1: T::from_f64(0.9).unwrap(),
            beta2: T::from_f64(0.999).unwrap(),
            epsilon: T::from_f64(1e-8).unwrap(),
            m: None,
            v: None,
            t: 0,
        }
    }
}

impl<T: RealField + Copy> Optimizer<DMatrix<T>> for Adam<T> {
    fn update(&mut self, param: &mut DMatrix<T>, grad: &DMatrix<T>) -> Result<(), OptimizationError> {
        if param.shape() != grad.shape() {
             return Err(OptimizationError::DimensionMismatch {
                expected: format!("{:?}", param.shape()),
                got: format!("{:?}", grad.shape()),
            });
        }

        // Initialize state if first step
        if self.m.is_none() {
            self.m = Some(DMatrix::zeros(param.nrows(), param.ncols()));
            self.v = Some(DMatrix::zeros(param.nrows(), param.ncols()));
        }

        self.t += 1;
        let t_val = T::from_i32(self.t).unwrap();
        let one = T::one();

        let m = self.m.as_mut().unwrap();
        let v = self.v.as_mut().unwrap();

        // Check state dimension matches param (sanity check against reuse for different shaped params)
        if m.shape() != param.shape() {
             return Err(OptimizationError::DimensionMismatch {
                expected: format!("{:?}", m.shape()),
                got: format!("{:?}", param.shape()),
            });
        }

        // Update biased first moment estimate
        *m = m.clone() * self.beta1 + grad.clone() * (one - self.beta1);

        // Update biased second raw moment estimate
        let grad_sq = grad.map(|g| g * g);
        *v = v.clone() * self.beta2 + grad_sq * (one - self.beta2);

        // Compute bias-corrected first moment estimate
        let m_hat = m.clone() / (one - self.beta1.powf(t_val));

        // Compute bias-corrected second raw moment estimate
        let v_hat = v.clone() / (one - self.beta2.powf(t_val));

        // Update parameters
        let update = m_hat.component_div(&v_hat.map(|val| val.sqrt() + self.epsilon));
        *param -= update * self.learning_rate;

        Ok(())
    }
}

// Implement for DVector by treating it as Nx1 Matrix
impl<T: RealField + Copy> Optimizer<DVector<T>> for Adam<T> {
    fn update(&mut self, param: &mut DVector<T>, grad: &DVector<T>) -> Result<(), OptimizationError> {
        if param.len() != grad.len() {
             return Err(OptimizationError::DimensionMismatch {
                expected: format!("{}", param.len()),
                got: format!("{}", grad.len()),
            });
        }

        // Initialize state if first step (Nx1)
        if self.m.is_none() {
            self.m = Some(DMatrix::zeros(param.len(), 1));
            self.v = Some(DMatrix::zeros(param.len(), 1));
        }

        self.t += 1;
        let t_val = T::from_i32(self.t).unwrap();
        let one = T::one();

        let m = self.m.as_mut().unwrap();
        let v = self.v.as_mut().unwrap();

        if m.len() != param.len() {
             return Err(OptimizationError::DimensionMismatch {
                expected: format!("{}", m.len()),
                got: format!("{}", param.len()),
            });
        }

        // Convert grad to matrix (Nx1)
        let grad_mat = DMatrix::from_column_slice(grad.len(), 1, grad.as_slice());

        // Update biased first moment estimate
        *m = m.clone() * self.beta1 + grad_mat.clone() * (one - self.beta1);

        // Update biased second raw moment estimate
        let grad_sq = grad_mat.map(|g| g * g);
        *v = v.clone() * self.beta2 + grad_sq * (one - self.beta2);

        // Compute bias-corrected first moment estimate
        let m_hat = m.clone() / (one - self.beta1.powf(t_val));

        // Compute bias-corrected second raw moment estimate
        let v_hat = v.clone() / (one - self.beta2.powf(t_val));

        // Update parameters
        let update_mat = m_hat.component_div(&v_hat.map(|val| val.sqrt() + self.epsilon));

        // Convert update back to vector
        let update_vec = DVector::from_column_slice(update_mat.as_slice());
        *param -= update_vec * self.learning_rate;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgd_matrix() {
        let mut sgd: SGD<f64> = SGD::new(0.1);
        let mut param: DMatrix<f64> = DMatrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let grad: DMatrix<f64> = DMatrix::from_vec(2, 2, vec![0.1, 0.1, 0.1, 0.1]);

        sgd.update(&mut param, &grad).unwrap();

        assert!((param[(0, 0)] - 0.99).abs() < 1e-6);
    }

    #[test]
    fn test_adam_vector() {
        let mut adam: Adam<f64> = Adam::new(0.1);
        let mut param: DVector<f64> = DVector::from_vec(vec![1.0, 2.0]);
        let grad: DVector<f64> = DVector::from_vec(vec![0.1, 0.1]);

        // First step
        adam.update(&mut param, &grad).unwrap();

        // Just check it changed
        assert!(param[0] != 1.0);
    }
}
