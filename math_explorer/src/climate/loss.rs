//! This module implements the loss functions for the CERA framework.

use nalgebra::DMatrix;

/// Computes the Mean Squared Error (MSE) between two matrices.
pub fn mse_loss(y_true: &DMatrix<f32>, y_pred: &DMatrix<f32>) -> f32 {
    assert_eq!(y_true.shape(), y_pred.shape(), "Matrices must have the same shape for MSE.");
    let diff = y_true - y_pred;
    diff.norm_squared() / (diff.nrows() * diff.ncols()) as f32
}

/// Computes the Earth Mover's Distance (EMD) for multiple 1D distributions.
///
/// This function takes two matrices where each column represents a 1D distribution.
/// It computes the EMD for each pair of columns and returns the average.
///
/// # Arguments
///
/// * `z1` - The first set of distributions, with shape (n_samples, n_distributions).
/// * `z2` - The second set of distributions, with shape (n_samples, n_distributions).
pub fn earth_movers_distance(z1: &DMatrix<f32>, z2: &DMatrix<f32>) -> f32 {
    assert_eq!(z1.shape(), z2.shape(), "Matrices must have the same shape for EMD.");
    if z1.ncols() == 0 {
        return 0.0;
    }

    let mut total_emd = 0.0;

    for i in 0..z1.ncols() {
        let mut col1: Vec<f32> = z1.column(i).iter().cloned().collect();
        let mut col2: Vec<f32> = z2.column(i).iter().cloned().collect();

        // The EMD for 1D distributions is the L1 norm of the difference
        // between the sorted samples.
        col1.sort_by(|a, b| a.partial_cmp(b).unwrap());
        col2.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let emd_i: f32 = col1.iter().zip(col2.iter()).map(|(a, b)| (a - b).abs()).sum();
        total_emd += emd_i / col1.len() as f32;
    }

    total_emd / z1.ncols() as f32
}

/// Computes the combined loss for the CERA framework.
/// The total loss is a weighted sum of the reconstruction loss, the prediction
/// loss, and the EMD loss.
pub fn cera_loss(
    reconstruction_loss: f32,
    prediction_loss: f32,
    emd_loss: f32,
    lambda_pred: f32,
    lambda_emd: f32,
) -> f32 {
    (1.0 - lambda_pred - lambda_emd).max(0.0) * reconstruction_loss
        + lambda_pred * prediction_loss
        + lambda_emd * emd_loss
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_mse_loss() {
        let y_true = DMatrix::from_row_slice(2, 2, &[1.0, 2.0, 3.0, 4.0]);
        let y_pred = DMatrix::from_row_slice(2, 2, &[1.0, 3.0, 2.0, 4.0]);
        // Differences are 0, -1, 1, 0. Squared differences are 0, 1, 1, 0.
        // Sum of squared differences is 2.
        // MSE = 2 / 4 = 0.5.
        let loss = mse_loss(&y_true, &y_pred);
        assert!((loss - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_earth_movers_distance() {
        // Two distributions for the first channel, two for the second
        let z1 = DMatrix::from_row_slice(4, 2, &[
            1.0, 8.0,
            2.0, 7.0,
            3.0, 6.0,
            4.0, 5.0,
        ]);
        let z2 = DMatrix::from_row_slice(4, 2, &[
            5.0, 4.0,
            6.0, 3.0,
            7.0, 2.0,
            8.0, 1.0,
        ]);

        // Channel 1: z1_sorted = [1,2,3,4], z2_sorted = [5,6,7,8]
        // Diff = [4,4,4,4]. Sum of abs diff = 16. EMD_1 = 16 / 4 = 4.

        // Channel 2: z1_sorted = [5,6,7,8], z2_sorted = [1,2,3,4]
        // Diff = [4,4,4,4]. Sum of abs diff = 16. EMD_2 = 16 / 4 = 4.

        // Average EMD = (4 + 4) / 2 = 4.
        let emd = earth_movers_distance(&z1, &z2);
        assert!((emd - 4.0).abs() < 1e-6);
    }
}
