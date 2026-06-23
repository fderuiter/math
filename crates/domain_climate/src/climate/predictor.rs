//! This module defines the predictor model for the CERA framework.

use crate::climate::autoencoder::{leaky_relu, ConvLayer};
use domain_ai::ai::optimization::Optimizer;
use nalgebra::{DMatrix, DVector};

/// A trait representing the predictor model interface.
/// This allows for different predictor architectures and decouples the training loop.
pub trait PredictorModel {
    /// Performs a forward pass through the predictor.
    fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32>;

    /// Updates the weights of the predictor using the provided optimizer.
    /// Note: This replaces the previous simplified learning rate update.
    fn update_weights<O: Optimizer<f32>>(
        &mut self,
        optimizer: &mut O,
    ) -> Result<(), domain_ai::ai::optimization::OptimizationError>;
}

/// A multi-layer perceptron (MLP) used as the predictor in the CERA framework.
///
/// The predictor takes the flattened, aligned latent representation from the
/// autoencoder's encoder and maps it to the target output variables.
pub struct Predictor {
    /// The stack of layers (using `ConvLayer` for simplicity as dense layers).
    pub layers: Vec<ConvLayer>,
    // Store dimensions for clarity
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
        Self::new_from_layers(layers, input_size, output_size)
    }

    /// Creates a new predictor model with custom layers.
    ///
    /// # Arguments
    ///
    /// * `layers` - A vector of convolutional layers.
    /// * `input_size` - The dimension of the input vector.
    /// * `output_size` - The dimension of the output vector.
    ///
    /// # Returns
    ///
    /// A new `Predictor` instance.
    pub fn new_from_layers(layers: Vec<ConvLayer>, input_size: usize, output_size: usize) -> Self {
        Self {
            layers,
            input_size,
            output_size,
        }
    }
}

impl PredictorModel for Predictor {
    fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32> {
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

    fn update_weights<O: Optimizer<f32>>(
        &mut self,
        optimizer: &mut O,
    ) -> Result<(), domain_ai::ai::optimization::OptimizationError> {
        for (i, layer) in self.layers.iter_mut().enumerate() {
            // Placeholder: Generate random gradients to simulate training dynamics
            // In a real implementation, these would come from backpropagation.
            let grad_k = DMatrix::from_fn(layer.kernel.nrows(), layer.kernel.ncols(), |_, _| {
                rand::random::<f32>() - 0.5
            });
            let grad_b = DVector::from_fn(layer.bias.len(), |_, _| rand::random::<f32>() - 0.5);

            // Use the optimizer strategy to update weights
            optimizer.update_matrix_legacy(i, &mut layer.kernel, &grad_k)?;
            optimizer.update_vector_legacy(i, &mut layer.bias, &grad_b)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain_ai::ai::optimization::SGD;
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

    #[test]
    fn test_predictor_update_weights_with_optimizer() {
        let input_size = 10;
        let output_size = 5;
        let mut predictor = Predictor::new(input_size, output_size);
        let mut optimizer = SGD::new(0.1f32);

        let initial_kernel = predictor.layers[0].kernel.clone();

        // Update weights
        let _ = predictor.update_weights(&mut optimizer);

        // Check that weights have changed
        let new_kernel = &predictor.layers[0].kernel;
        assert_ne!(initial_kernel, *new_kernel);
    }
}
