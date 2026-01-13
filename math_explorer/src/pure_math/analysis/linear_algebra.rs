//! Linear Algebra Utilities.
//!
//! Provides implementations for solving linear systems, least squares, and matrix operations
//! commonly used in engineering and physics problems.

use nalgebra::{DMatrix, DVector};

/// Solves a linear system $Ax = b$ using LU decomposition (or best available method).
///
/// # Arguments
///
/// * `a` - The matrix $A$.
/// * `b` - The vector $b$.
///
/// # Returns
///
/// * `Option<DVector<f64>>` - The solution vector $x$, if one exists.
pub fn solve_linear_system(a: &DMatrix<f64>, b: &DVector<f64>) -> Option<DVector<f64>> {
    a.clone().lu().solve(b)
}

/// Solves the Normal Equation for Least Squares.
///
/// $$ \mathbf{c} = (A^T A)^{-1} A^T \mathbf{b} $$
///
/// Used for finding the optimal coefficients that minimize the error squared.
///
/// # Arguments
///
/// * `a` - Design matrix $A$.
/// * `b` - Observation vector $b$.
///
/// # Returns
///
/// * `Option<DVector<f64>>` - The coefficient vector $\mathbf{c}$.
pub fn solve_normal_equation(a: &DMatrix<f64>, b: &DVector<f64>) -> Option<DVector<f64>> {
    let a_t = a.transpose();
    let ata = &a_t * a;
    let atb = &a_t * b;

    // Invert (A^T A)
    // Note: Cholesky decomposition is more efficient for symmetric positive definite matrices (like A^T A)
    match ata.clone().cholesky() {
        Some(cholesky) => Some(cholesky.solve(&atb)),
        None => ata.try_inverse().map(|inv| &inv * &atb), // Fallback
    }
}

/// Calculates the Moore-Penrose Pseudoinverse ($J^{\dagger}$).
///
/// $$ \dot{\gamma} = J^{\dagger} \tau_{cmd} $$
///
/// # Arguments
///
/// * `matrix` - The matrix $J$.
/// * `epsilon` - Tolerance for singular values (to handle near-singularities).
///
/// # Returns
///
/// * `DMatrix<f64>` - The pseudoinverse matrix.
pub fn moore_penrose_pseudoinverse(matrix: &DMatrix<f64>, epsilon: f64) -> DMatrix<f64> {
    match matrix.clone().svd(true, true).pseudo_inverse(epsilon) {
        Ok(pinv) => pinv,
        Err(_) => DMatrix::zeros(matrix.ncols(), matrix.nrows()), // Should generally not happen with SVD
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_equation() {
        // Fit y = mx + c
        // Points: (1, 1), (2, 2), (3, 3)
        // A = [[1, 1], [1, 2], [1, 3]] (cols: c, m)
        // b = [1, 2, 3]
        let a = DMatrix::from_row_slice(3, 2, &[1.0, 1.0, 1.0, 2.0, 1.0, 3.0]);
        let b = DVector::from_column_slice(&[1.0, 2.0, 3.0]);

        let c_vec = solve_normal_equation(&a, &b).unwrap();
        // Expect c=0, m=1
        assert!(c_vec[0].abs() < 1e-6);
        assert!((c_vec[1] - 1.0).abs() < 1e-6);
    }
}
