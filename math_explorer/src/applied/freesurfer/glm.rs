use nalgebra::{DMatrix, DVector, RealField};

/// Estimates the beta parameters of the GLM using ordinary least squares.
/// beta = (X^T * X)^-1 * X^T * Y
///
/// # Arguments
/// * `x` - The design matrix (subjects x predictors).
/// * `y` - The data vector (e.g., thickness at a vertex for all subjects).
///
/// # Returns
/// The estimated beta parameters, or an error if the matrix is not invertible.
pub fn estimate_beta<T: RealField>(
    x: &DMatrix<T>,
    y: &DVector<T>,
) -> Result<DVector<T>, &'static str> {
    let xt = x.transpose();
    let xtx = &xt * x;
    let xtx_inv = xtx.try_inverse().ok_or("X^T * X is not invertible")?;
    let xty = xt * y;
    Ok(xtx_inv * xty)
}

/// Calculates the t-statistic for a given contrast.
/// t = c^T * beta / sqrt(sigma^2 * c^T * (X^T * X)^-1 * c)
///
/// # Arguments
/// * `c` - The contrast vector.
/// * `beta` - The estimated beta parameters.
/// * `x` - The design matrix.
/// * `residual_variance` - sigma^2, the variance of the residuals (Y - X*beta).
///
/// # Returns
/// The t-statistic, or an error if matrices are non-conformable or non-invertible.
pub fn t_statistic<T: RealField + Copy>(
    c: &DVector<T>,
    beta: &DVector<T>,
    x: &DMatrix<T>,
    residual_variance: T,
) -> Result<T, &'static str> {
    let xtx = x.transpose() * x;
    let xtx_inv = xtx.try_inverse().ok_or("X^T * X is not invertible")?;

    let numerator = c.dot(beta);

    let variance_term = c.dot(&(xtx_inv * c));
    let denominator = (residual_variance * variance_term).sqrt();

    if denominator == T::zero() {
        return Err("Denominator is zero, t-statistic is undefined.");
    }

    Ok(numerator / denominator)
}
