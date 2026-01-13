//! Linear Algebra Solvers.
//!
//! Provides utilities for solving systems of linear equations.

use nalgebra::{DMatrix, DVector};

/// Solves the Normal Equation for Least Squares.
///
/// $$ \mathbf{c} = (A^T A)^{-1} A^T \mathbf{b} $$
///
/// # Arguments
/// * `a` - Design matrix ($A$).
/// * `b` - Observation vector ($\mathbf{b}$).
///
/// # Returns
/// * `Some(c)` - Optimal coefficient vector.
/// * `None` - If $A^T A$ is singular.
pub fn normal_equation(a: &DMatrix<f64>, b: &DVector<f64>) -> Option<DVector<f64>> {
    let a_t = a.transpose();
    let a_t_a = &a_t * a;
    let a_t_b = &a_t * b;

    match a_t_a.try_inverse() {
        Some(inv) => Some(&inv * a_t_b),
        None => None,
    }
}

/// Solves a Linear System.
///
/// $$ Ax = b $$
///
/// # Arguments
/// * `a` - Matrix ($A$).
/// * `b` - Result vector ($b$).
pub fn linear_system_solve(a: &DMatrix<f64>, b: &DVector<f64>) -> Option<DVector<f64>> {
    let epsilon = 1e-9;
    a.clone().svd(true, true).solve(b, epsilon).ok()
}
