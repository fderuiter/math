use std::fmt;

/// Errors that can occur during LoRA operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoraError {
    /// The ensemble is empty.
    EmptyEnsemble,
    /// The provided weights are empty.
    EmptyWeights,
    /// The number of weights does not match the number of modules.
    MismatchLength,
    /// Mismatched tensor shapes for the same key.
    MismatchShape,
    /// Mismatched keys between LoRA modules.
    MismatchKeys,
}

impl fmt::Display for LoraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoraError::EmptyEnsemble => write!(f, "Ensemble is empty; cannot combine."),
            LoraError::EmptyWeights => write!(f, "Weights cannot be empty."),
            LoraError::MismatchLength => write!(
                f,
                "The number of weights must match the number of modules in the ensemble."
            ),
            LoraError::MismatchShape => write!(f, "Mismatched tensor shapes for the same key."),
            LoraError::MismatchKeys => write!(f, "Mismatched keys between LoRA modules."),
        }
    }
}

impl std::error::Error for LoraError {}
