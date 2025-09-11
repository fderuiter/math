//! This module defines the autoencoder architecture for the CERA framework.

use nalgebra::{DMatrix, DVector};
use crate::climate::tensor_ops::{conv1d};

/// A simple leaky ReLU activation function.
pub fn leaky_relu(x: &mut DMatrix<f32>, alpha: f32) {
    x.iter_mut().for_each(|val| {
        if *val < 0.0 {
            *val *= alpha;
        }
    });
}

/// A single layer for the Encoder or Decoder, consisting of a convolution and activation.
pub struct ConvLayer {
    pub kernel: DMatrix<f32>,
    pub bias: DVector<f32>,
    // Store dimensions for clarity
    in_channels: usize,
    out_channels: usize,
}

impl ConvLayer {
    /// Creates a new convolutional layer with random initialization.
    pub fn new(in_channels: usize, out_channels: usize) -> Self {
        // Simple random initialization using from_fn
        let kernel = DMatrix::from_fn(out_channels, in_channels, |_, _| rand::random::<f32>() * 2.0 - 1.0);
        let bias = DVector::from_fn(out_channels, |_, _| rand::random::<f32>() * 2.0 - 1.0);
        Self { kernel, bias, in_channels, out_channels }
    }
}


/// The encoder component of the autoencoder.
pub struct Encoder {
    pub layers: Vec<ConvLayer>,
}

impl Encoder {
    /// Creates a new encoder with a hardcoded architecture.
    /// Input (2 channels) -> 64 -> 64 -> Latent (3 channels)
    pub fn new(in_channels: usize, latent_channels: usize) -> Self {
        let layers = vec![
            ConvLayer::new(in_channels, 64),
            ConvLayer::new(64, 64),
            ConvLayer::new(64, latent_channels), // No activation on the latent layer
        ];
        Self { layers }
    }

    /// Encodes the input data into a latent representation.
    pub fn forward(&self, input: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = input.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            x = conv1d(&x, &layer.kernel, &layer.bias);
            // No activation on the final layer
            if i < self.layers.len() - 1 {
                leaky_relu(&mut x, 0.01);
            }
        }
        x
    }
}

/// The decoder component of the autoencoder.
pub struct Decoder {
    pub layers: Vec<ConvLayer>,
}

impl Decoder {
    /// Creates a new decoder with a hardcoded architecture.
    /// Latent (3 channels) -> 64 -> 64 -> Output (2 channels)
    pub fn new(latent_channels: usize, out_channels: usize) -> Self {
        let layers = vec![
            ConvLayer::new(latent_channels, 64),
            ConvLayer::new(64, 64),
            ConvLayer::new(64, out_channels), // No activation on the output layer
        ];
        Self { layers }
    }

    /// Reconstructs the input data from the latent representation.
    pub fn forward(&self, latent_representation: &DMatrix<f32>) -> DMatrix<f32> {
        let mut x = latent_representation.clone();
        for (i, layer) in self.layers.iter().enumerate() {
            x = conv1d(&x, &layer.kernel, &layer.bias);
            if i < self.layers.len() - 1 {
                leaky_relu(&mut x, 0.01);
            }
        }
        x
    }
}

/// The autoencoder model for the CERA framework.
pub struct Autoencoder {
    pub encoder: Encoder,
    pub decoder: Decoder,
}

impl Autoencoder {
    /// Creates a new autoencoder.
    /// The paper specifies 2 input channels and 3 latent channels.
    pub fn new(in_channels: usize, latent_channels: usize) -> Self {
        // The decoder's input is the encoder's output, and vice versa.
        let encoder = Encoder::new(in_channels, latent_channels);
        let decoder = Decoder::new(latent_channels, in_channels);
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

    #[test]
    fn test_leaky_relu() {
        let mut matrix = DMatrix::from_row_slice(2, 2, &[-1.0, 2.0, -3.0, 0.0]);
        leaky_relu(&mut matrix, 0.1);
        let expected = DMatrix::from_row_slice(2, 2, &[-0.1, 2.0, -0.3, 0.0]);
        assert!((matrix - expected).abs().max() < 1e-6);
    }
}
