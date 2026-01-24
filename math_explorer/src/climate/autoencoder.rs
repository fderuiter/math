//! This module defines the autoencoder architecture for the CERA framework.

use crate::ai::activations::{ActivationFunction, LeakyReLU};
use crate::climate::tensor_ops::conv1d;
use nalgebra::{DMatrix, DVector};

/// A single layer for the Encoder or Decoder, consisting of a convolution and activation.
pub struct ConvLayer {
    /// The convolution kernel matrix.
    pub kernel: DMatrix<f32>,
    /// The bias vector.
    pub bias: DVector<f32>,
    // Store dimensions for clarity
    #[allow(dead_code)]
    in_channels: usize,
    #[allow(dead_code)]
    out_channels: usize,
}

impl ConvLayer {
    /// Creates a new convolutional layer with random initialization.
    ///
    /// # Arguments
    ///
    /// * `in_channels` - Number of input channels.
    /// * `out_channels` - Number of output channels.
    ///
    /// # Returns
    ///
    /// A new `ConvLayer`.
    pub fn new(in_channels: usize, out_channels: usize) -> Self {
        // Simple random initialization using from_fn
        let kernel = DMatrix::from_fn(out_channels, in_channels, |_, _| {
            rand::random::<f32>() * 2.0 - 1.0
        });
        let bias = DVector::from_fn(out_channels, |_, _| rand::random::<f32>() * 2.0 - 1.0);
        Self {
            kernel,
            bias,
            in_channels,
            out_channels,
        }
    }
}

/// The encoder component of the autoencoder.
pub struct Encoder {
    /// The stack of convolutional layers.
    pub layers: Vec<ConvLayer>,
}

impl Encoder {
    /// Creates a new encoder with a hardcoded architecture.
    /// Input (2 channels) -> 64 -> 64 -> Latent (3 channels)
    ///
    /// # Arguments
    ///
    /// * `in_channels` - Number of input channels.
    /// * `latent_channels` - Dimension of the latent space.
    ///
    /// # Returns
    ///
    /// A new `Encoder`.
    pub fn new(in_channels: usize, latent_channels: usize) -> Self {
        let layers = vec![
            ConvLayer::new(in_channels, 64),
            ConvLayer::new(64, 64),
            ConvLayer::new(64, latent_channels), // No activation on the latent layer
        ];
        Self { layers }
    }

    /// Encodes the input data into a latent representation.
    ///
    /// # Arguments
    ///
    /// * `input` - The input data matrix.
    ///
    /// # Returns
    ///
    /// The latent representation matrix.
    pub fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = input.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            x = conv1d(&x, &layer.kernel, &layer.bias);
            // No activation on the final layer
            if i < self.layers.len() - 1 {
                let activation = LeakyReLU { alpha: 0.01 };
                activation.apply(&mut x);
            }
        }
        x
    }
}

/// The decoder component of the autoencoder.
pub struct Decoder {
    /// The stack of convolutional layers.
    pub layers: Vec<ConvLayer>,
}

impl Decoder {
    /// Creates a new decoder with a hardcoded architecture.
    /// Latent (3 channels) -> 64 -> 64 -> Output (2 channels)
    ///
    /// # Arguments
    ///
    /// * `latent_channels` - Dimension of the latent space.
    /// * `out_channels` - Number of output channels.
    ///
    /// # Returns
    ///
    /// A new `Decoder`.
    pub fn new(latent_channels: usize, out_channels: usize) -> Self {
        let layers = vec![
            ConvLayer::new(latent_channels, 64),
            ConvLayer::new(64, 64),
            ConvLayer::new(64, out_channels), // No activation on the output layer
        ];
        Self { layers }
    }

    /// Reconstructs the input data from the latent representation.
    ///
    /// # Arguments
    ///
    /// * `latent_representation` - The latent representation matrix.
    ///
    /// # Returns
    ///
    /// The reconstructed data matrix.
    pub fn forward(&self, latent_representation: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = latent_representation.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            x = conv1d(&x, &layer.kernel, &layer.bias);
            if i < self.layers.len() - 1 {
                let activation = LeakyReLU { alpha: 0.01 };
                activation.apply(&mut x);
            }
        }
        x
    }
}

/// The autoencoder model for the CERA framework.
pub struct Autoencoder {
    /// The encoder component.
    pub encoder: Encoder,
    /// The decoder component.
    pub decoder: Decoder,
}

impl Autoencoder {
    /// Creates a new autoencoder.
    /// The paper specifies 2 input channels and 3 latent channels.
    ///
    /// # Arguments
    ///
    /// * `in_channels` - Number of input channels.
    /// * `latent_channels` - Dimension of the latent space.
    ///
    /// # Returns
    ///
    /// A new `Autoencoder`.
    pub fn new(in_channels: usize, latent_channels: usize) -> Self {
        // The decoder's input is the encoder's output, and vice versa.
        let encoder = Encoder::new(in_channels, latent_channels);
        let decoder = Decoder::new(latent_channels, in_channels);
        Self { encoder, decoder }
    }

    /// Performs a forward pass through the autoencoder.
    ///
    /// # Arguments
    ///
    /// * `input` - The input data matrix.
    ///
    /// # Returns
    ///
    /// A tuple containing `(latent_representation, reconstruction)`.
    pub fn forward(&self, input: &DMatrix<f32>) -> (DMatrix<f32>, DMatrix<f32>) {
        let latent = self.encoder.forward(input);
        let reconstruction = self.decoder.forward(&latent);
        (latent, reconstruction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn test_autoencoder_forward_pass() {
        let in_channels = 2;
        let latent_channels = 3;
        let num_levels = 30;
        let batch_size = 4;
        let n_samples = num_levels * batch_size;

        let autoencoder = Autoencoder::new(in_channels, latent_channels);

        let input = DMatrix::from_fn(n_samples, in_channels, |_, _| rand::random());

        let (latent, reconstruction) = autoencoder.forward(&input);

        assert_eq!(latent.nrows(), n_samples);
        assert_eq!(latent.ncols(), latent_channels);
        assert_eq!(reconstruction.nrows(), n_samples);
        assert_eq!(reconstruction.ncols(), in_channels);
    }

    #[test]
    fn test_leaky_relu() {
        let mut matrix = DMatrix::from_row_slice(2, 2, &[-1.0, 2.0, -3.0, 0.0]);
        let activation = LeakyReLU { alpha: 0.1 };
        activation.apply(&mut matrix);
        let expected = DMatrix::from_row_slice(2, 2, &[-0.1, 2.0, -0.3, 0.0]);
        assert!((matrix - expected).abs().max() < 1e-6);
    }
}
