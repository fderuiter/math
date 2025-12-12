use nalgebra::DMatrix;

/// 4.1 Residual Calculation
/// Input: Predicted Noise (final), Added Noise (epsilon).
/// Operation: grad_2d = epsilon_final - epsilon
pub fn compute_residual(
    predicted_noise: &DMatrix<f64>,
    added_noise: &DMatrix<f64>,
) -> DMatrix<f64> {
    predicted_noise - added_noise
}

/// 4.2 Weighting Scheme
/// Input: Residual, Timestep weighting w(t).
/// Operation: delta_sds = w(t) * residual
pub fn apply_weighting(residual: &DMatrix<f64>, weight: f64) -> DMatrix<f64> {
    residual * weight
}

/// 4.3 The "Detach" Operation
/// In Rust's ownership model or typical tensor libraries (like Torch), "detach" stops gradient tracking.
/// Since we are implementing the math explicitly here using DMatrix (which doesn't have an autodiff graph attached by default),
/// the result is effectively already detached unless we were building a graph.
/// This function serves as an explicit marker for the operation.
pub fn detach(tensor: DMatrix<f64>) -> DMatrix<f64> {
    // Identity function in this context, but signifies logical break in gradient graph.
    tensor
}

/// Computes the full SDS gradient vector (delta_SDS).
pub fn compute_sds_gradient(
    predicted_noise: &DMatrix<f64>,
    added_noise: &DMatrix<f64>,
    weight: f64,
) -> DMatrix<f64> {
    let residual = compute_residual(predicted_noise, added_noise);
    let weighted = apply_weighting(&residual, weight);
    detach(weighted)
}
