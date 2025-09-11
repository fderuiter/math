//! # LoraHub Mathematical Core
//!
//! This module contains the Rust implementation of the core mathematical
//! functions found in the LoraHub project. LoraHub's main idea is to
//! combine multiple Low-Rank Adaptation (LoRA) modules by finding an
//! optimal set of weights for their linear combination.
//!
//! The key mathematical operations are:
//! 1.  A weighted sum of the LoRA tensors.
//! 2.  An objective function to be minimized, which typically consists of
//!     a model's loss and a regularization term.
//! 3.  A regularization term, such as L1 regularization, to encourage
//!     sparsity in the weights.
//!
//! This implementation uses `nalgebra` for linear algebra operations.
use nalgebra::DMatrix;
use std::collections::HashMap;

/// Represents a LoRA state dictionary as a map from tensor names to matrices.
pub type LoraStateDict = HashMap<String, DMatrix<f64>>;

/// Combines multiple LoRA state dictionaries using a weighted sum.
///
/// # Arguments
/// * `loras` - A slice of LoRA state dictionaries to be combined.
/// * `weights` - A slice of weights corresponding to each LoRA module.
///
/// # Returns
/// A `Result` containing the combined `LoraStateDict`, or an error message
/// if the inputs are invalid (e.g., mismatched lengths or empty).
pub fn combine_loras(loras: &[LoraStateDict], weights: &[f64]) -> Result<LoraStateDict, &'static str> {
    if loras.is_empty() || weights.is_empty() {
        return Err("Input loras or weights cannot be empty.");
    }
    if loras.len() != weights.len() {
        return Err("The number of loras and weights must be the same.");
    }

    let first_lora = &loras[0];
    let mut final_state_dict: LoraStateDict = first_lora
        .iter()
        .map(|(key, tensor)| (key.clone(), tensor * weights[0]))
        .collect();

    for (i, lora_state_dict) in loras.iter().enumerate().skip(1) {
        let keys: Vec<String> = final_state_dict.keys().cloned().collect();
        for key in keys {
            if let Some(tensor) = lora_state_dict.get(&key) {
                if let Some(final_tensor) = final_state_dict.get_mut(&key) {
                    // Ensure dimensions match before adding
                    if final_tensor.shape() != tensor.shape() {
                        return Err("Mismatched tensor shapes for the same key.");
                    }
                    *final_tensor += tensor * weights[i];
                }
            } else {
                return Err("Mismatched keys between LoRA modules.");
            }
        }
    }

    Ok(final_state_dict)
}

/// Calculates the L1 regularization term for a given set of weights.
///
/// The formula is: `alpha * sum(abs(w_i)) / n`, where `alpha` is a scaling factor.
///
/// # Arguments
/// * `weights` - A slice of weights.
/// * `alpha` - The regularization strength.
///
/// # Returns
/// The L1 regularization value.
pub fn l1_regularization(weights: &[f64], alpha: f64) -> f64 {
    if weights.is_empty() {
        return 0.0;
    }
    let sum_abs = weights.iter().map(|w| w.abs()).sum::<f64>();
    alpha * sum_abs / (weights.len() as f64)
}

/// Calculates the objective score to be minimized by the optimizer.
///
/// This score is the sum of a model's loss and a regularization term.
///
/// # Arguments
/// * `weights` - The current weights for the LoRA combination.
/// * `mock_loss` - A simulated loss value from the model. In a real scenario,
///   this would be calculated after combining the LoRAs and running a forward pass.
/// * `alpha` - The L1 regularization strength.
///
/// # Returns
/// The objective score.
pub fn calculate_objective_score(weights: &[f64], mock_loss: f64, alpha: f64) -> f64 {
    let regularization_term = l1_regularization(weights, alpha);
    mock_loss + regularization_term
}
