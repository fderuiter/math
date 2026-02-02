//! This module defines the core CERA framework, integrating the autoencoder and predictor.

use crate::climate::autoencoder::{Autoencoder, AutoencoderModel};
use crate::climate::predictor::{Predictor, PredictorModel};
use nalgebra::DMatrix;
// Re-export CeraConfig for backward compatibility (or convenience)
pub use crate::climate::config::CeraConfig;

/// The main CERA model.
pub struct Cera {
    /// The autoencoder component.
    pub autoencoder: Box<dyn AutoencoderModel>,
    /// The predictor component.
    pub predictor: Box<dyn PredictorModel>,
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
        Ok(Self {
            autoencoder: Box::new(autoencoder),
            predictor: Box::new(predictor),
            config,
        })
    }

    /// Creates a new CERA model with a custom predictor.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration struct.
    /// * `predictor` - A custom predictor implementation.
    ///
    /// # Returns
    ///
    /// A result containing the new `Cera` instance or an error message.
    pub fn new_with_predictor(
        config: CeraConfig,
        predictor: Box<dyn PredictorModel>,
    ) -> Result<Self, String> {
        if config.aligned_channels > config.latent_channels {
            return Err(format!(
                "aligned_channels ({}) cannot be greater than latent_channels ({})",
                config.aligned_channels, config.latent_channels
            ));
        }
        // ... (Other checks omitted for brevity in this variant, but ideally shared)
        // For strict correctness, we should validate config here too.
        if config.aligned_channels == 0 {
            return Err("aligned_channels must be greater than 0".to_string());
        }
        if config.num_levels == 0 {
            return Err("num_levels must be greater than 0".to_string());
        }

        let autoencoder = Autoencoder::new(config.in_channels, config.latent_channels);
        Ok(Self {
            autoencoder: Box::new(autoencoder),
            predictor,
            config,
        })
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
    fn reshape_for_predictor(
        &self,
        latent_matrix: &DMatrix<f32>,
        batch_size: usize,
    ) -> DMatrix<f32> {
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

        let latent = self.autoencoder.encode(inputs);
        let aligned_latent = latent.columns(0, aligned_channels).clone_owned();
        let predictor_input = self.reshape_for_predictor(&aligned_latent, batch_size);
        self.predictor.forward(&predictor_input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::climate::training::CeraTrainer;
    use nalgebra::DMatrix; // Import Trainer

    // Helper constant for tests
    const TEST_NUM_LEVELS: usize = 30;
    const TEST_IN_CHANNELS: usize = 2;
    const TEST_OUTPUT_SIZE: usize = 148;

    fn generate_data(n_samples: usize, offset: f32) -> (DMatrix<f32>, DMatrix<f32>) {
        let inputs = DMatrix::from_fn(n_samples * TEST_NUM_LEVELS, TEST_IN_CHANNELS, |_, _| {
            rand::random::<f32>() + offset
        });
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

        // Use Trainer
        let mut trainer = CeraTrainer::new(&mut cera);
        trainer.train(&control_inputs, &control_targets, &warm_inputs);

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
