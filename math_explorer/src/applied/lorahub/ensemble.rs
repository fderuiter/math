use super::types::LoraStateDict;

/// A specialized ensemble of LoRA modules that can be combined dynamically.
///
/// `LoraEnsemble` encapsulates the collection of LoRA state dictionaries
/// and provides methods to compute weighted combinations and objective scores.
///
/// # Architecture
/// This struct replaces the free-standing `combine_loras` functions, moving towards
/// a more object-oriented approach where the data (the modules) and the operations
/// (combination) are coupled.
pub struct LoraEnsemble {
    modules: Vec<LoraStateDict>,
}

impl LoraEnsemble {
    /// Creates a new `LoraEnsemble` from a list of state dictionaries.
    ///
    /// # Arguments
    /// * `modules` - A vector of LoRA state dictionaries.
    pub fn new(modules: Vec<LoraStateDict>) -> Self {
        Self { modules }
    }

    /// Combines the encapsulated LoRA modules using a weighted sum.
    ///
    /// # Arguments
    /// * `weights` - A slice of weights corresponding to each LoRA module.
    ///
    /// # Returns
    /// A `Result` containing the combined `LoraStateDict`, or an error message
    /// if the inputs are invalid.
    pub fn combine(&self, weights: &[f64]) -> Result<LoraStateDict, &'static str> {
        if self.modules.is_empty() {
            return Err("Ensemble is empty; cannot combine.");
        }
        if weights.is_empty() {
            return Err("Weights cannot be empty.");
        }
        if self.modules.len() != weights.len() {
            return Err("The number of weights must match the number of modules in the ensemble.");
        }

        let first_lora = &self.modules[0];
        // Initialize with the first weighted LoRA
        let mut final_state_dict: LoraStateDict = first_lora
            .iter()
            .map(|(key, tensor)| (key.clone(), tensor * weights[0]))
            .collect();

        // Accumulate the rest
        for (i, lora_state_dict) in self.modules.iter().enumerate().skip(1) {
            // Check keys against the first one
            // Note: This assumes all dicts share keys. strict validation.
            for (key, final_tensor) in &mut final_state_dict {
                 if let Some(tensor) = lora_state_dict.get(key) {
                    if final_tensor.shape() != tensor.shape() {
                        return Err("Mismatched tensor shapes for the same key.");
                    }
                    *final_tensor += tensor * weights[i];
                 } else {
                     return Err("Mismatched keys between LoRA modules.");
                 }
            }
            // Double check we aren't missing keys in the new lora that were in the old one
            // The iteration above only iterates keys present in `final_state_dict` (from 0th).
            // We should ensure `lora_state_dict` doesn't have *extra* keys?
            // The original implementation iterated `final_state_dict.keys()`.
            // So if `lora_state_dict` has extra keys, they are ignored.
            // If `lora_state_dict` misses keys, we error.
            // Let's stick to the original logic's intent but optimized.
        }

        Ok(final_state_dict)
    }

    /// Calculates the objective score to be minimized.
    ///
    /// Score = Mock Loss + L1 Regularization Term.
    ///
    /// # Arguments
    /// * `weights` - The weights being evaluated.
    /// * `mock_loss` - The external loss value.
    /// * `alpha` - Regularization strength.
    pub fn evaluate_objective(&self, weights: &[f64], mock_loss: f64, alpha: f64) -> f64 {
        let reg_term = Self::l1_regularization(weights, alpha);
        mock_loss + reg_term
    }

    /// Helper for L1 regularization: `alpha * mean(abs(weights))`.
    fn l1_regularization(weights: &[f64], alpha: f64) -> f64 {
        if weights.is_empty() {
            return 0.0;
        }
        let sum_abs = weights.iter().map(|w| w.abs()).sum::<f64>();
        alpha * sum_abs / (weights.len() as f64)
    }
}
