use super::error::LoraError;
use super::strategies::{
    CombinationStrategy, L1RegularizationStrategy, LinearCombinationStrategy, ObjectiveStrategy,
};
use super::types::LoraStateDict;

/// A specialized ensemble of LoRA modules that can be combined dynamically.
///
/// `LoraEnsemble` encapsulates the collection of LoRA state dictionaries
/// and provides methods to compute weighted combinations and objective scores.
///
/// # Architecture
/// This struct uses the Strategy Pattern to decouple the combination logic
/// and objective evaluation from the data structure.
pub struct LoraEnsemble {
    modules: Vec<LoraStateDict>,
    combination_strategy: Box<dyn CombinationStrategy>,
    objective_strategy: Box<dyn ObjectiveStrategy>,
}

impl LoraEnsemble {
    /// Creates a new `LoraEnsemble` with default strategies (Linear Combination, L1 Regularization).
    ///
    /// # Arguments
    /// * `modules` - A vector of LoRA state dictionaries.
    pub fn new(modules: Vec<LoraStateDict>) -> Self {
        Self {
            modules,
            combination_strategy: Box::new(LinearCombinationStrategy),
            // Default alpha=0.0 since it wasn't stored before.
            // But evaluate_objective overrides this anyway for legacy calls.
            objective_strategy: Box::new(L1RegularizationStrategy::new(0.0)),
        }
    }

    /// Creates a new `LoraEnsemble` with custom strategies.
    pub fn with_strategies(
        modules: Vec<LoraStateDict>,
        combination_strategy: Box<dyn CombinationStrategy>,
        objective_strategy: Box<dyn ObjectiveStrategy>,
    ) -> Self {
        Self {
            modules,
            combination_strategy,
            objective_strategy,
        }
    }

    /// Combines the encapsulated LoRA modules using the configured strategy.
    ///
    /// # Arguments
    /// * `weights` - A slice of weights corresponding to each LoRA module.
    ///
    /// # Returns
    /// A `Result` containing the combined `LoraStateDict`, or a `LoraError`
    /// if the inputs are invalid.
    pub fn combine(&self, weights: &[f64]) -> Result<LoraStateDict, LoraError> {
        self.combination_strategy.combine(&self.modules, weights)
    }

    /// Calculates the objective score using the provided alpha (Legacy).
    ///
    /// # Compatibility
    /// This method preserves the original API signature by creating a temporary
    /// `L1RegularizationStrategy` using the provided `alpha`.
    ///
    /// # Arguments
    /// * `weights` - The weights being evaluated.
    /// * `mock_loss` - The external loss value.
    /// * `alpha` - Regularization strength.
    pub fn evaluate_objective(&self, weights: &[f64], mock_loss: f64, alpha: f64) -> f64 {
        let strategy = L1RegularizationStrategy::new(alpha);
        strategy.evaluate(weights, mock_loss)
    }

    /// Calculates the objective score using the configured strategy.
    pub fn evaluate(&self, weights: &[f64], mock_loss: f64) -> f64 {
        self.objective_strategy.evaluate(weights, mock_loss)
    }
}
