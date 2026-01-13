//! Regularization Techniques.
//!
//! Methods to prevent overfitting in statistical models.

use nalgebra::{DMatrix, DVector};

/// L1 Norm-Regularized Least Squares (Lasso).
///
/// $$ J(x) = \frac{1}{2} \| y - A x \|_2^2 + \lambda \| x \|_1 $$
///
/// Note: The prompt formula has $J(x) = \frac{1}{2} \ y^ - z(Wx)\ ^2_2 + \lambda \ x$.
/// This seems garbled. Assuming standard Lasso formulation: Minimize $0.5 * ||Ax - b||^2 + lambda * ||x||_1$.
/// This is a convex optimization problem. A simple implementation uses coordinate descent (shooting algorithm)
/// or proximal gradient descent (ISTA).
///
/// Here we implement Iterative Soft-Thresholding Algorithm (ISTA).
///
/// # Arguments
/// * `a` - Design matrix ($A$ or $W$).
/// * `y` - Target vector ($y$).
/// * `lambda` - Regularization parameter ($\lambda$).
/// * `max_iter` - Maximum iterations.
pub fn l1_regularized_least_squares(
    a: &DMatrix<f64>,
    y: &DVector<f64>,
    lambda: f64,
    max_iter: usize,
) -> DVector<f64> {
    let n_features = a.ncols();
    let mut x = DVector::zeros(n_features);

    // Lipschitz constant estimate (largest eigenvalue of A^T A) or just step size
    // For simplicity, use a small fixed step size or backtracking.
    // L <= ||A||^2
    let step_size = 1.0 / (a.norm_squared() + 1e-10);

    for _ in 0..max_iter {
        // Gradient of data fidelity term: A^T (Ax - y)
        let residual = a * &x - y;
        let grad = a.transpose() * residual;

        // Gradient descent step
        let x_temp = &x - step_size * grad;

        // Proximal operator (Soft Thresholding)
        for i in 0..n_features {
            x[i] = soft_threshold(x_temp[i], lambda * step_size);
        }
    }

    x
}

fn soft_threshold(value: f64, threshold: f64) -> f64 {
    if value > threshold {
        value - threshold
    } else if value < -threshold {
        value + threshold
    } else {
        0.0
    }
}
