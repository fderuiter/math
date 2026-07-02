//! This module provides helper functions for tensor operations required by the CERA framework.
//! A 1D convolution with kernel size 1 is equivalent to a linear transformation (dense layer)
//! applied at each position.

use nalgebra::{DMatrix, DVector};

/// Performs a 1D convolution with kernel size 1.
/// This is equivalent to a matrix multiplication.
///
/// # Arguments
///
/// * `input` - The input tensor, with shape (N, in_channels), where N is batch_size * num_levels.
/// * `kernel` - The convolutional kernel (weights), with shape (out_channels, in_channels).
/// * `bias` - The bias term, with shape (out_channels).
///
/// # Returns
///
/// * The output tensor, with shape (N, out_channels).
#[verified_engine::verified]
pub fn conv1d(input: &DMatrix<f32>, kernel: &DMatrix<f32>, bias: &DVector<f32>) -> DMatrix<f32> {
    // Check dimensions
    assert_eq!(
        input.ncols(),
        kernel.ncols(),
        "Input channels must match kernel input channels."
    );
    assert_eq!(
        kernel.nrows(),
        bias.len(),
        "Kernel output channels must match bias length."
    );

    // The core operation is `output = input * kernel^T + bias`
    let mut output = input * kernel.transpose();

    // Add bias to each row
    output
        .row_iter_mut()
        .for_each(|mut row| row += bias.transpose());

    output
}

/// Performs a 1D transposed convolution with kernel size 1.
/// This is equivalent to a matrix multiplication with the kernel's transpose.
///
/// # Arguments
///
/// * `input` - The input tensor, with shape (N, in_channels).
/// * `kernel` - The convolutional kernel (weights), with shape (in_channels, out_channels).
///   Note that this is the kernel of the corresponding non-transposed conv layer.
/// * `bias` - The bias term, with shape (out_channels).
///
/// # Returns
///
/// * The output tensor, with shape (N, out_channels).
#[verified_engine::verified]
pub fn conv_transpose1d(
    input: &DMatrix<f32>,
    kernel: &DMatrix<f32>, // Shape (out_channels, in_channels) from the original conv
    bias: &DVector<f32>,
) -> DMatrix<f32> {
    // Check dimensions
    assert_eq!(
        input.ncols(),
        kernel.nrows(),
        "Input channels must match kernel output channels."
    );
    assert_eq!(
        kernel.ncols(),
        bias.len(),
        "Kernel input channels must match bias length."
    );

    // The core operation is `output = input * kernel + bias`
    // Note: This is not a true transposed convolution, but what's needed for a symmetric decoder
    // with kernel size 1 layers. The operation is `input * W`, not `input * W^T`.
    let mut output = input * kernel;

    // Add bias to each row
    output
        .row_iter_mut()
        .for_each(|mut row| row += bias.transpose());

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    #[test]
    #[verified_engine::verified]
    fn test_conv1d_basic() {
        // Input: 2 samples, 3 input channels
        let input = DMatrix::from_row_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // Kernel: 4 output channels, 3 input channels
        let kernel = DMatrix::from_row_slice(
            4,
            3,
            &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        );
        // Bias: 4 output channels
        let bias = DVector::from_vec(vec![0.1, 0.2, 0.3, 0.4]);

        let output = conv1d(&input, &kernel, &bias);

        let expected_output =
            DMatrix::from_row_slice(2, 4, &[1.1, 2.2, 3.3, 6.4, 4.1, 5.2, 6.3, 15.4]);

        assert!((output - expected_output).abs().max() < math_commons::registry::TOLERANCE_FAST_F32);
    }

    #[test]
    #[verified_engine::verified]
    fn test_conv_transpose1d_basic() {
        // Input: 2 samples, 4 input channels
        let input = DMatrix::from_row_slice(2, 4, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        // Kernel from original conv: 4 output channels, 3 input channels
        let kernel = DMatrix::from_row_slice(
            4,
            3,
            &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0],
        );
        // Bias: 3 output channels
        let bias = DVector::from_vec(vec![0.1, 0.2, 0.3]);

        let output = conv_transpose1d(&input, &kernel, &bias);

        // Corrected expected output based on my manual calculation in the previous turn.
        let expected_output = DMatrix::from_row_slice(2, 3, &[5.1, 6.2, 7.3, 13.1, 14.2, 15.3]);

        assert!((output - expected_output).abs().max() < math_commons::registry::TOLERANCE_FAST_F32);
    }
}
