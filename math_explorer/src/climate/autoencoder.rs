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
pub struct EncoderGeneric<A: ActivationFunction<f32>> {
    /// The stack of convolutional layers.
    pub layers: Vec<ConvLayer>,
    /// The activation function strategy.
    pub activation: A,
}

/// Default Encoder type alias for backward compatibility.
pub type Encoder = EncoderGeneric<LeakyReLU<f32>>;

impl Encoder {
    /// Creates a new encoder with a hardcoded architecture.
    pub fn new(in_channels: usize, latent_channels: usize) -> Self {
        Self::new_with_activation(in_channels, latent_channels, LeakyReLU::new(0.01))
    }
}

impl<A: ActivationFunction<f32>> EncoderGeneric<A> {
    pub fn new_with_activation(in_channels: usize, latent_channels: usize, activation: A) -> Self {
        let layers = vec![
            ConvLayer::new(in_channels, 64),
            ConvLayer::new(64, 64),
            ConvLayer::new(64, latent_channels), // No activation on the latent layer
        ];
        Self { layers, activation }
    }

    /// Encodes the input data into a latent representation.
    pub fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = input.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            x = conv1d(&x, &layer.kernel, &layer.bias);
            // No activation on the final layer
            if i < self.layers.len() - 1 {
                self.activation.apply(&mut x);
            }
        }
        x
    }
}

/// The decoder component of the autoencoder.
pub struct DecoderGeneric<A: ActivationFunction<f32>> {
    /// The stack of convolutional layers.
    pub layers: Vec<ConvLayer>,
    /// The activation function strategy.
    pub activation: A,
}

/// Default Decoder type alias.
pub type Decoder = DecoderGeneric<LeakyReLU<f32>>;

impl Decoder {
    /// Creates a new decoder with a hardcoded architecture.
    pub fn new(latent_channels: usize, out_channels: usize) -> Self {
        Self::new_with_activation(latent_channels, out_channels, LeakyReLU::new(0.01))
    }
}

impl<A: ActivationFunction<f32>> DecoderGeneric<A> {
    pub fn new_with_activation(
        latent_channels: usize,
        out_channels: usize,
        activation: A,
    ) -> Self {
        let layers = vec![
            ConvLayer::new(latent_channels, 64),
            ConvLayer::new(64, 64),
            ConvLayer::new(64, out_channels), // No activation on the output layer
        ];
        Self { layers, activation }
    }

    /// Reconstructs the input data from the latent representation.
    pub fn forward(&self, latent_representation: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = latent_representation.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            x = conv1d(&x, &layer.kernel, &layer.bias);
            if i < self.layers.len() - 1 {
                self.activation.apply(&mut x);
            }
        }
        x
    }
}

/// The autoencoder model for the CERA framework.
pub struct AutoencoderGeneric<A: ActivationFunction<f32>> {
    /// The encoder component.
    pub encoder: EncoderGeneric<A>,
    /// The decoder component.
    pub decoder: DecoderGeneric<A>,
}

/// Default Autoencoder type alias.
pub type Autoencoder = AutoencoderGeneric<LeakyReLU<f32>>;

impl Autoencoder {
    /// Creates a new autoencoder.
    pub fn new(in_channels: usize, latent_channels: usize) -> Self {
        Self::new_with_activation(in_channels, latent_channels, LeakyReLU::new(0.01))
    }
}

impl<A: ActivationFunction<f32> + Clone> AutoencoderGeneric<A> {
    pub fn new_with_activation(
        in_channels: usize,
        latent_channels: usize,
        activation: A,
    ) -> Self {
        let encoder = EncoderGeneric::new_with_activation(in_channels, latent_channels, activation.clone());
        let decoder = DecoderGeneric::new_with_activation(latent_channels, in_channels, activation);
        Self { encoder, decoder }
    }

    /// Performs a forward pass through the autoencoder.
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

}
