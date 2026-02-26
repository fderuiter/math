use super::linear_algebra::Vector;
use super::probability::softmax;
use super::model::{Trainable, TwoLayerMLP};
use crate::ai::optimization::{Optimizer, cross_entropy_softmax_prime};
use std::ops::{Deref, DerefMut};

/// The Deep Learning Cycle: Forward -> Loss -> Backward -> Update
///
/// This struct manages the training loop for any `Trainable` model.
/// Architecture: Decoupled via `Trainable` trait.
///
/// # Generics
/// * `M`: The model type, implementing `Trainable`. Defaults to `TwoLayerMLP` for backward compatibility.
pub struct TrainingLoop<M: Trainable = TwoLayerMLP> {
    pub model: M,
    pub optimizer: Box<dyn Optimizer<f64>>,
}

// Deref implementation allows `network.layer1` access if `M` has those fields (like `TwoLayerMLP`).
impl<M: Trainable> Deref for TrainingLoop<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl<M: Trainable> DerefMut for TrainingLoop<M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.model
    }
}

impl TrainingLoop<TwoLayerMLP> {
    /// Backward compatibility constructor for the default MLP architecture.
    pub fn new(
        input_dim: usize,
        hidden_dim: usize,
        output_dim: usize,
        optimizer: Box<dyn Optimizer<f64>>,
    ) -> Self {
        Self {
            model: TwoLayerMLP::new(input_dim, hidden_dim, output_dim),
            optimizer,
        }
    }
}

impl<M: Trainable> TrainingLoop<M> {
    /// Creates a new Training Loop with a custom model.
    pub fn new_with_model(model: M, optimizer: Box<dyn Optimizer<f64>>) -> Self {
        Self { model, optimizer }
    }

    /// Performs one iteration of training (Forward, Backward, Update).
    ///
    /// # Arguments
    /// * `x` - Input vector.
    /// * `y_true` - One-hot encoded target vector.
    ///
    /// # Returns
    /// The scalar loss value.
    pub fn train_step(&mut self, x: &Vector, y_true: &Vector) -> f64 {
        // --- 1. Forward Pass (Linear Algebra) ---
        // Get logits from the model
        let z = self.model.forward(x);
        // Prediction (Softmax)
        let y_pred = softmax(&z);

        // --- 2. Loss Calculation (Probability & Statistics) ---
        // Using Cross-Entropy.
        let epsilon = 1e-15;
        let loss = -(y_true.dot(&y_pred.map(|v| (v + epsilon).ln())));

        // --- 3. Backward Pass (Calculus) ---
        // Gradient of Loss w.r.t logits (z)
        // dL/dz = y_pred - y_true
        let loss_grad = cross_entropy_softmax_prime(&z, y_true);

        // Delegate specific backprop logic to the model strategy
        self.model.backward_update(x, &loss_grad, &mut *self.optimizer);

        loss
    }

    /// Predicts the class for a given input.
    pub fn predict(&self, x: &Vector) -> Vector {
        let z = self.model.forward(x);
        softmax(&z)
    }
}
