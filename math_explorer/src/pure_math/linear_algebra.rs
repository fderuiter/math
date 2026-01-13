//! Linear Algebra Utilities and Specialized Solvers.

use nalgebra::{DMatrix, DVector};
use std::error::Error;
use std::fmt;

/// Errors related to linear algebra operations.
#[derive(Debug)]
pub enum LinearAlgebraError {
    /// Dimension mismatch for matrix operation.
    DimensionMismatch,
    /// Matrix is singular or non-invertible.
    SingularMatrix,
    /// SVD calculation failed.
    SvdFailure,
}

impl fmt::Display for LinearAlgebraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch => write!(f, "Matrix/Vector dimension mismatch"),
            Self::SingularMatrix => write!(f, "Matrix is singular"),
            Self::SvdFailure => write!(f, "SVD calculation failed"),
        }
    }
}

impl Error for LinearAlgebraError {}

/// Solves the Normal Equation for Least Squares.
///
/// $$ \mathbf{c} = (A^T A)^{-1} A^T \mathbf{b} $$
///
/// Finds the optimal coefficient vector $\mathbf{c}$ that minimizes $|A\mathbf{c} - \mathbf{b}|^2$.
///
/// # Arguments
/// * `a` - Design matrix ($A$).
/// * `b` - Observation vector ($\mathbf{b}$).
pub fn normal_equation(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>, LinearAlgebraError> {
    if a.nrows() != b.len() {
        return Err(LinearAlgebraError::DimensionMismatch);
    }

    let a_t = a.transpose();
    let a_t_a = &a_t * a;
    let a_t_b = &a_t * b;

    // Use Cholesky decomposition for A^T * A as it is symmetric positive-definite (usually)
    // Fallback to LU or SVD if Cholesky fails (e.g. not positive definite due to numerical issues).
    // nalgebra's solve() usually picks a good strategy, but for Normal Equation specifically:

    match a_t_a.try_inverse() {
        Some(inv) => Ok(&inv * &a_t_b),
        None => Err(LinearAlgebraError::SingularMatrix),
    }
}

/// Calculates the Moore-Penrose Pseudoinverse ($J^{\dagger}$).
///
/// $$ J^{\dagger} = (J^T J)^{-1} J^T \quad \text{(if rows > cols)} $$
///
/// or via SVD for general case.
///
/// Used in spacecraft attitude control: $\dot{\gamma} = J^{\dagger} \tau_{cmd}$.
///
/// # Arguments
/// * `matrix` - The matrix to invert.
pub fn moore_penrose_pseudoinverse(matrix: &DMatrix<f64>) -> Result<DMatrix<f64>, LinearAlgebraError> {
    let epsilon = 1e-9;
    match matrix.clone().pseudo_inverse(epsilon) {
        Ok(pinv) => Ok(pinv),
        Err(_) => Err(LinearAlgebraError::SvdFailure),
    }
}

/// Solves a linear system $Ax = b$.
///
/// Wrapper around nalgebra solver, useful for benchmarking (LINPACK style).
pub fn linear_system_solver(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<DVector<f64>, LinearAlgebraError> {
    if a.nrows() != a.ncols() || a.nrows() != b.len() {
        return Err(LinearAlgebraError::DimensionMismatch);
    }
    match a.clone().lu().solve(b) {
        Some(x) => Ok(x),
        None => Err(LinearAlgebraError::SingularMatrix),
    }
}
