//! This module defines the core CERA framework, integrating the autoencoder and predictor.

use nalgebra::{DMatrix, DVector};
use crate::climate::autoencoder::Autoencoder;
use crate::climate::predictor::Predictor;
use crate::climate::loss::{cera_loss, mse_loss, earth_movers_distance};

/// Configuration for the CERA model.
#[derive(Clone, Debug)]
pub struct CeraConfig {
    /// Learning rate for the optimizer.
    pub learning_rate: f32,
    /// Weight for the prediction loss term.
    pub lambda_pred: f32,
    /// Weight for the Earth Mover's Distance (EMD) loss term.
    pub lambda_emd: f32,
    /// Number of training epochs.
    pub epochs: usize,
    /// Batch size for training.
    pub batch_size: usize,
    /// Number of input channels.
    pub in_channels: usize,
    /// Dimension of the latent space.
    pub latent_channels: usize,
    /// Number of channels used for alignment/prediction.
    pub aligned_channels: usize,
    /// Number of vertical levels (or time steps) in the input.
    pub num_levels: usize,
    /// Size of the output vector.
    pub output_size: usize,
}

/// The main CERA model.
pub struct Cera {
    /// The autoencoder component.
    pub autoencoder: Autoencoder,
    /// The predictor component.
    pub predictor: Predictor,
    /// The model configuration.
    pub config: CeraConfig,
}

impl Cera {
    /// Creates a new CERA model with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration struct.
    ///
    /// # Returns
    ///
    /// A result containing the new `Cera` instance or an error message.
    pub fn new(config: CeraConfig) -> Result<Self, String> {
        if config.aligned_channels > config.latent_channels {
            return Err(format!(
                "aligned_channels ({}) cannot be greater than latent_channels ({})",
                config.aligned_channels, config.latent_channels
            ));
        }
        if config.aligned_channels == 0 {
             return Err("aligned_channels must be greater than 0".to_string());
        }
        if config.num_levels == 0 {
             return Err("num_levels must be greater than 0".to_string());
        }

        let autoencoder = Autoencoder::new(config.in_channels, config.latent_channels);
        let predictor_input_size = config.num_levels * config.aligned_channels;
        let predictor = Predictor::new(predictor_input_size, config.output_size);
        Ok(Self { autoencoder, predictor, config })
    }

    /// Placeholder for the backpropagation and optimization step.
    ///
    /// **CRITICAL NOTE:** This function is a placeholder and does **not** perform
    /// real backpropagation or gradient descent. It simulates a weight update by
    /// subtracting small random values from the weights. This is done to allow
    /// the end-to-end testing of the model's architecture and data flow.
    /// A full implementation would require a proper autograd engine to compute
    /// gradients and an optimizer (e.g., Adam) to update the weights.
    fn optimizer_step(&mut self) {
        let lr = self.config.learning_rate;
        for layer in self.autoencoder.encoder.layers.iter_mut() {
            let grad_k = DMatrix::from_fn(layer.kernel.nrows(), layer.kernel.ncols(), |_,_| rand::random::<f32>() - 0.5);
            let grad_b = DVector::from_fn(layer.bias.len(), |_,_| rand::random::<f32>() - 0.5);
            layer.kernel -= grad_k * lr;
            layer.bias -= grad_b * lr;
        }
        for layer in self.autoencoder.decoder.layers.iter_mut() {
            let grad_k = DMatrix::from_fn(layer.kernel.nrows(), layer.kernel.ncols(), |_,_| rand::random::<f32>() - 0.5);
            let grad_b = DVector::from_fn(layer.bias.len(), |_,_| rand::random::<f32>() - 0.5);
            layer.kernel -= grad_k * lr;
            layer.bias -= grad_b * lr;
        }
        for layer in self.predictor.layers.iter_mut() {
            let grad_k = DMatrix::from_fn(layer.kernel.nrows(), layer.kernel.ncols(), |_,_| rand::random::<f32>() - 0.5);
            let grad_b = DVector::from_fn(layer.bias.len(), |_,_| rand::random::<f32>() - 0.5);
            layer.kernel -= grad_k * lr;
            layer.bias -= grad_b * lr;
        }
    }

    /// Reshapes a batch of latent vectors for the predictor.
    /// From (batch_size * num_levels, channels) to (batch_size, num_levels * channels).
    ///
    /// # Arguments
    ///
    /// * `latent_matrix` - The matrix of latent vectors.
    /// * `batch_size` - The number of samples in the batch.
    ///
    /// # Returns
    ///
    /// The reshaped matrix ready for the predictor.
    fn reshape_for_predictor(&self, latent_matrix: &DMatrix<f32>, batch_size: usize) -> DMatrix<f32> {
        let num_levels = self.config.num_levels;
        let aligned_channels = self.config.aligned_channels;
        let mut reshaped_data = Vec::with_capacity(batch_size * num_levels * aligned_channels);
        for i in 0..batch_size {
            let start_row = i * num_levels;
            let sample_latent = latent_matrix.rows(start_row, num_levels);
            for r in sample_latent.row_iter() {
                for element in r.iter() {
                    reshaped_data.push(*element);
                }
            }
        }
        DMatrix::from_row_slice(batch_size, num_levels * aligned_channels, &reshaped_data)
    }

    /// Trains the CERA model on synthetic data.
    ///
    /// # Arguments
    ///
    /// * `control_inputs` - Input data for the control climate.
    /// * `control_targets` - Target outputs for the control climate.
    /// * `warm_inputs` - Input data for the warm climate.
    pub fn train(
        &mut self,
        control_inputs: &DMatrix<f32>,
        control_targets: &DMatrix<f32>,
        warm_inputs: &DMatrix<f32>,
    ) {
        let batch_size = self.config.batch_size;
        let num_levels = self.config.num_levels;
        let aligned_channels = self.config.aligned_channels;

        let n_samples = control_inputs.nrows() / num_levels;
        let n_batches = n_samples / batch_size;

        for epoch in 0..self.config.epochs {
            let mut total_loss = 0.0;
            for i in 0..n_batches {
                // --- Create batches ---
                let input_start = i * batch_size * num_levels;
                let input_rows = batch_size * num_levels;
                let control_input_batch = control_inputs.rows(input_start, input_rows).clone_owned();
                let warm_input_batch = warm_inputs.rows(input_start, input_rows).clone_owned();

                let target_start = i * batch_size;
                let control_target_batch = control_targets.rows(target_start, batch_size).clone_owned();

                // --- Forward pass ---
                let (control_latent, control_recon) = self.autoencoder.forward(&control_input_batch);
                let (warm_latent, warm_recon) = self.autoencoder.forward(&warm_input_batch);

                // --- Reshape and predict ---
                let control_aligned_latent = control_latent.columns(0, aligned_channels).clone_owned();
                let predictor_input = self.reshape_for_predictor(&control_aligned_latent, batch_size);
                let prediction = self.predictor.forward(&predictor_input);

                // --- Calculate losses ---
                let recon_loss_control = mse_loss(&control_input_batch, &control_recon);
                let recon_loss_warm = mse_loss(&warm_input_batch, &warm_recon);
                let reconstruction_loss = (recon_loss_control + recon_loss_warm) / 2.0;

                let prediction_loss = mse_loss(&control_target_batch, &prediction);

                let warm_aligned_latent = warm_latent.columns(0, aligned_channels).clone_owned();
                let emd_loss = earth_movers_distance(&control_aligned_latent, &warm_aligned_latent);

                let loss = cera_loss(
                    reconstruction_loss,
                    prediction_loss,
                    emd_loss,
                    self.config.lambda_pred,
                    self.config.lambda_emd,
                );

                // --- Backward pass and optimization ---
                self.optimizer_step();
                total_loss += loss;
            }
            println!("Epoch {}, Average Loss: {}", epoch, total_loss / n_batches as f32);
        }
    }

    /// Makes a prediction using the trained CERA model.
    ///
    /// # Arguments
    ///
    /// * `inputs` - Input data matrix.
    ///
    /// # Returns
    ///
    /// The predicted output matrix.
    pub fn predict(&self, inputs: &DMatrix<f32>) -> DMatrix<f32> {
        let num_levels = self.config.num_levels;
        let aligned_channels = self.config.aligned_channels;
        let batch_size = inputs.nrows() / num_levels;

        let latent = self.autoencoder.encoder.forward(inputs);
        let aligned_latent = latent.columns(0, aligned_channels).clone_owned();
        let predictor_input = self.reshape_for_predictor(&aligned_latent, batch_size);
        self.predictor.forward(&predictor_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    // Helper constant for tests
    const TEST_NUM_LEVELS: usize = 30;
    const TEST_IN_CHANNELS: usize = 2;
    const TEST_OUTPUT_SIZE: usize = 148;

    fn generate_data(n_samples: usize, offset: f32) -> (DMatrix<f32>, DMatrix<f32>) {
        let inputs = DMatrix::from_fn(n_samples * TEST_NUM_LEVELS, TEST_IN_CHANNELS, |_, _| rand::random::<f32>() + offset);
        let targets = DMatrix::from_fn(n_samples, TEST_OUTPUT_SIZE, |_, _| rand::random());
        (inputs, targets)
    }

    #[test]
    fn test_cera_training_and_prediction() {
        let config = CeraConfig {
            learning_rate: 0.001,
            lambda_pred: 0.1,
            lambda_emd: 0.01,
            epochs: 2, // Keep it short for testing
            batch_size: 4,
            in_channels: TEST_IN_CHANNELS,
            latent_channels: 3,
            aligned_channels: 2,
            num_levels: TEST_NUM_LEVELS,
            output_size: TEST_OUTPUT_SIZE,
        };

        let mut cera = Cera::new(config).expect("Failed to create CERA model");

        let n_samples = 16;
        let (control_inputs, control_targets) = generate_data(n_samples, 0.0);
        let (warm_inputs, _) = generate_data(n_samples, 1.0); // Warm climate has a different distribution

        cera.train(&control_inputs, &control_targets, &warm_inputs);

        let (test_inputs, _) = generate_data(4, 0.5);
        let prediction = cera.predict(&test_inputs);

        assert_eq!(prediction.nrows(), 4);
        assert_eq!(prediction.ncols(), TEST_OUTPUT_SIZE);
    }

    #[test]
    fn test_cera_invalid_config() {
        let config = CeraConfig {
            learning_rate: 0.001,
            lambda_pred: 0.1,
            lambda_emd: 0.01,
            epochs: 1,
            batch_size: 1,
            in_channels: 2,
            latent_channels: 3,
            aligned_channels: 4, // Invalid: > latent_channels
            num_levels: 30,
            output_size: 10,
        };
        assert!(Cera::new(config).is_err());
    }
}
