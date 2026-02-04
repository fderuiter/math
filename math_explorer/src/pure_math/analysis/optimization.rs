//! Optimization Algorithms.
//!
//! Provides structures and traits for mathematical optimization problems.

use nalgebra::{DMatrix, DVector};

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
