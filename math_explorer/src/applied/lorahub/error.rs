use thiserror::Error;

/// Errors that can occur during LoRA ensemble combination.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoraError {
    #[error("Ensemble is empty; cannot combine.")]
    EmptyEnsemble,
    #[error("Weights cannot be empty.")]
    EmptyWeights,
    #[error("The number of weights must match the number of modules in the ensemble.")]
    WeightModuleMismatch,
    #[error("Mismatched tensor shapes for the same key.")]
    TensorShapeMismatch,
    #[error("Mismatched keys between LoRA modules.")]
    KeyMismatch,
}
