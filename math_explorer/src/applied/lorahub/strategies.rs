use super::types::LoraStateDict;

/// Strategy for combining multiple LoRA state dictionaries.
pub trait CombinationStrategy {
    fn combine(
        &self,
        modules: &[LoraStateDict],
        weights: &[f64],
    ) -> Result<LoraStateDict, &'static str>;
}

/// Strategy for calculating the objective score.
pub trait ObjectiveStrategy {
    fn evaluate(&self, weights: &[f64], mock_loss: f64) -> f64;
}

/// The standard linear combination strategy.
pub struct LinearCombinationStrategy;

impl CombinationStrategy for LinearCombinationStrategy {
    fn combine(
        &self,
        modules: &[LoraStateDict],
        weights: &[f64],
    ) -> Result<LoraStateDict, &'static str> {
        if modules.is_empty() {
            return Err("Ensemble is empty; cannot combine.");
        }
        if weights.is_empty() {
            return Err("Weights cannot be empty.");
        }
        if modules.len() != weights.len() {
            return Err("The number of weights must match the number of modules in the ensemble.");
        }

        let first_lora = &modules[0];
        // Initialize with the first weighted LoRA
        let mut final_state_dict: LoraStateDict = first_lora
            .iter()
            .map(|(key, tensor)| (key.clone(), tensor * weights[0]))
            .collect();

        // Accumulate the rest
        for (i, lora_state_dict) in modules.iter().enumerate().skip(1) {
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
        }

        Ok(final_state_dict)
    }
}

/// L1 Regularization strategy.
pub struct L1RegularizationStrategy {
    pub alpha: f64,
}

impl L1RegularizationStrategy {
    pub fn new(alpha: f64) -> Self {
        Self { alpha }
    }
}

impl ObjectiveStrategy for L1RegularizationStrategy {
    fn evaluate(&self, weights: &[f64], mock_loss: f64) -> f64 {
        if weights.is_empty() {
            return mock_loss;
        }
        let sum_abs = weights.iter().map(|w| w.abs()).sum::<f64>();
        let reg_term = self.alpha * sum_abs / (weights.len() as f64);
        mock_loss + reg_term
    }
}
