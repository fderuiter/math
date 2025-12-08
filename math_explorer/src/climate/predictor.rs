//! This module defines the predictor model for the CERA framework.

use nalgebra::DMatrix;
use crate::climate::autoencoder::{ConvLayer, leaky_relu}; // Re-use from autoencoder module

/// A multi-layer perceptron (MLP) used as the predictor in the CERA framework.
///
/// The predictor takes the flattened, aligned latent representation from the
/// autoencoder's encoder and maps it to the target output variables.
pub struct Predictor {
    /// The stack of layers (using `ConvLayer` for simplicity as dense layers).
    pub layers: Vec<ConvLayer>,
    // Store dimensions for clarity
    /// The size of the input vector.
    #[allow(dead_code)]
    input_size: usize,
    /// The size of the output vector.
    #[allow(dead_code)]
    output_size: usize,
}

impl Predictor {
    /// Creates a new predictor model with a hardcoded architecture.
    /// Input (60) -> 128 -> 128 -> 128 -> 128 -> Output (148)
    ///
    /// # Arguments
    ///
    /// * `input_size` - The dimension of the input vector.
    /// * `output_size` - The dimension of the output vector.
    ///
    /// # Returns
    ///
    /// A new `Predictor` instance.
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let layers = vec![
            ConvLayer::new(input_size, 128),
            ConvLayer::new(128, 128),
            ConvLayer::new(128, 128),
            ConvLayer::new(128, 128),
            ConvLayer::new(128, output_size), // No activation on the final layer
        ];
        Self { layers, input_size, output_size }
    }

    /// Performs a forward pass through the predictor.
    ///
    /// # Arguments
    ///
    /// * `input` - The flattened latent representation, with shape (batch_size, input_size).
    ///
    /// # Returns
    ///
    /// The predicted output matrix of shape (batch_size, output_size).
    pub fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = input.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            // A dense layer is equivalent to a 1D convolution with kernel size 1
            // if we treat the input as having 1 level.
            // Our conv1d function `input * kernel.transpose()` works directly for this.
            x = crate::climate::tensor_ops::conv1d(&x, &layer.kernel, &layer.bias);
            // No activation on the final layer
            if i < self.layers.len() - 1 {
                leaky_relu(&mut x, 0.01);
            }
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_predictor_forward_pass() {
        let input_size = 30 * 2; // 30 levels, 2 aligned latent channels
        let output_size = 148;
        let batch_size = 4;

        let predictor = Predictor::new(input_size, output_size);

        let input = DMatrix::from_fn(batch_size, input_size, |_, _| rand::random());

        let output = predictor.forward(&input);

        assert_eq!(output.nrows(), batch_size);
        assert_eq!(output.ncols(), output_size);
    }
}
