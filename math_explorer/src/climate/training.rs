//! This module handles the training process for the CERA model.

use crate::ai::optimization::{Optimizer, SGD};
use crate::climate::autoencoder::AutoencoderModel;
use crate::climate::cera::Cera;
use crate::climate::loss::{cera_loss, earth_movers_distance, mse_loss};
use crate::climate::predictor::PredictorModel;
use nalgebra::DMatrix;

/// A trainer for the CERA model.
pub struct CeraTrainer<'a, A: AutoencoderModel, P: PredictorModel> {
    pub model: &'a mut Cera<A, P>,
    /// The optimizer strategy (e.g., SGD, Adam).
    pub optimizer: Box<dyn Optimizer<f32>>,
}

impl<'a, A: AutoencoderModel, P: PredictorModel> CeraTrainer<'a, A, P> {
    /// Creates a new CeraTrainer.
    pub fn new(model: &'a mut Cera<A, P>) -> Self {
        // Initialize optimizer from config.
        // Currently defaulting to SGD, but could be configurable.
        let lr = model.config.learning_rate;
        let optimizer = Box::new(SGD::new(lr));
        Self { model, optimizer }
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
        // Use the autoencoder's interface for updates
        self.model.autoencoder.update_weights(&mut *self.optimizer);

        // Use the predictor's interface for updates
        self.model.predictor.update_weights(&mut *self.optimizer);
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
        let batch_size = self.model.config.batch_size;
        let num_levels = self.model.config.num_levels;
        let aligned_channels = self.model.config.aligned_channels;

        // Ensure we don't divide by zero if inputs are empty, though new() checks dimensions.
        if control_inputs.nrows() == 0 || num_levels == 0 || batch_size == 0 {
            return;
        }

        let n_samples = control_inputs.nrows() / num_levels;
        let n_batches = n_samples / batch_size;

        for epoch in 0..self.model.config.epochs {
            let mut total_loss = 0.0;
            for i in 0..n_batches {
                // --- Create batches ---
                let input_start = i * batch_size * num_levels;
                let input_rows = batch_size * num_levels;
                let control_input_batch =
                    control_inputs.rows(input_start, input_rows).clone_owned();
                let warm_input_batch = warm_inputs.rows(input_start, input_rows).clone_owned();

                let target_start = i * batch_size;
                let control_target_batch =
                    control_targets.rows(target_start, batch_size).clone_owned();

                // --- Forward pass ---
                let (control_latent, control_recon) =
                    self.model.autoencoder.forward(&control_input_batch);
                let (warm_latent, warm_recon) = self.model.autoencoder.forward(&warm_input_batch);

                // --- Reshape and predict ---
                let control_aligned_latent =
                    control_latent.columns(0, aligned_channels).clone_owned();
                // Reuse the method from Cera
                let predictor_input = self
                    .model
                    .reshape_for_predictor(&control_aligned_latent, batch_size);
                let prediction = self.model.predictor.forward(&predictor_input);

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
                    self.model.config.lambda_pred,
                    self.model.config.lambda_emd,
                );

                // --- Backward pass and optimization ---
                self.optimizer_step();
                total_loss += loss;
            }
            if n_batches > 0 {
                println!(
                    "Epoch {}, Average Loss: {}",
                    epoch,
                    total_loss / n_batches as f32
                );
            }
        }
    }
}
