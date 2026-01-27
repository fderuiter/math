//! This module defines the predictor model for the CERA framework.

use crate::ai::activations::{ActivationFunction, LeakyReLU};
use crate::climate::autoencoder::ConvLayer;
use nalgebra::{DMatrix, DVector};

/// A trait representing the predictor model interface.
/// This allows for different predictor architectures and decouples the training loop.
pub trait PredictorModel {
    /// Performs a forward pass through the predictor.
    fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32>;

    /// Updates the weights of the predictor using a learning rate.
    /// Note: This is a simplified update step for demonstration/simulation purposes.
    fn update_weights(&mut self, learning_rate: f32);
}

/// A multi-layer perceptron (MLP) used as the predictor in the CERA framework.
///
/// The predictor takes the flattened, aligned latent representation from the
/// autoencoder's encoder and maps it to the target output variables.
pub struct Predictor<A: ActivationFunction<f32>> {
    /// The stack of layers (using `ConvLayer` for simplicity as dense layers).
    pub layers: Vec<ConvLayer>,
    /// The activation function strategy.
    pub activation: A,
    // Store dimensions for clarity
    #[allow(dead_code)]
    input_size: usize,
    /// The size of the output vector.
    #[allow(dead_code)]
    output_size: usize,
}

impl Predictor<LeakyReLU<f32>> {
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
        Self::new_with_activation(input_size, output_size, LeakyReLU::new(0.01))
    }
}

impl<A: ActivationFunction<f32>> Predictor<A> {
    /// Creates a new predictor with a custom activation function.
    pub fn new_with_activation(input_size: usize, output_size: usize, activation: A) -> Self {
        let layers = vec![
            ConvLayer::new(input_size, 128),
            ConvLayer::new(128, 128),
            ConvLayer::new(128, 128),
            ConvLayer::new(128, 128),
            ConvLayer::new(128, output_size), // No activation on the final layer
        ];
        Self {
            layers,
            activation,
            input_size,
            output_size,
        }
    }
}

impl<A: ActivationFunction<f32>> PredictorModel for Predictor<A> {
    fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = input.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            // A dense layer is equivalent to a 1D convolution with kernel size 1
            // if we treat the input as having 1 level.
            // Our conv1d function `input * kernel.transpose()` works directly for this.
            x = crate::climate::tensor_ops::conv1d(&x, &layer.kernel, &layer.bias);
            // No activation on the final layer
            if i < self.layers.len() - 1 {
                self.activation.apply(&mut x);
            }
        }
        x
    }

    fn update_weights(&mut self, learning_rate: f32) {
        for layer in self.layers.iter_mut() {
            let grad_k = DMatrix::from_fn(layer.kernel.nrows(), layer.kernel.ncols(), |_, _| {
                rand::random::<f32>() - 0.5
            });
            let grad_b = DVector::from_fn(layer.bias.len(), |_, _| rand::random::<f32>() - 0.5);
            layer.kernel -= grad_k * learning_rate;
            layer.bias -= grad_b * learning_rate;
        }
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
