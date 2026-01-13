//! Regression Analysis.

use nalgebra::{DMatrix, DVector};

/// Multivariate Linear Regression.
///
/// $$ Y = \tilde{X}B + E $$
///
/// Solves for $B$ using Ordinary Least Squares: $B = (X^T X)^{-1} X^T Y$.
///
/// # Arguments
/// * `x` - Predictor matrix ($\tilde{X}$).
/// * `y` - Response matrix/vector ($Y$).
///
/// # Returns
/// * `Some(B)` - Coefficient matrix.
pub fn multivariate_linear_regression(x: &DMatrix<f64>, y: &DMatrix<f64>) -> Option<DMatrix<f64>> {
    let x_t = x.transpose();
    let x_t_x = &x_t * x;
    let x_t_y = &x_t * y;

    match x_t_x.try_inverse() {
        Some(inv) => Some(&inv * x_t_y),
        None => None,
    }
}
