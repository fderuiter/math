//! Error types for LoraHub.

use std::fmt;

/// Errors that can occur during LoRA operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoraError {
    /// The ensemble contains no modules.
    EmptyEnsemble,
    /// The provided weights slice is empty.
    EmptyWeights,
    /// The number of weights does not match the number of modules.
    WeightCountMismatch {
        /// Number of weights provided.
        weights: usize,
        /// Number of modules in the ensemble.
        modules: usize,
    },
    /// A tensor shape mismatch was detected.
    ShapeMismatch {
        /// The key of the tensor.
        key: String,
        /// The expected shape (from the first module).
        expected: (usize, usize),
        /// The actual shape (from the mismatched module).
        actual: (usize, usize),
    },
    /// A key is missing in one of the modules.
    KeyMismatch {
        /// The missing key.
        key: String,
    },
}

impl fmt::Display for LoraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoraError::EmptyEnsemble => write!(f, "Ensemble is empty; cannot combine."),
            LoraError::EmptyWeights => write!(f, "Weights cannot be empty."),
            LoraError::WeightCountMismatch { weights, modules } => {
                write!(
                    f,
                    "Weight count {} does not match module count {}.",
                    weights, modules
                )
            }
            LoraError::ShapeMismatch {
                key,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Tensor shape mismatch for key '{}': expected {:?}, got {:?}.",
                    key, expected, actual
                )
            }
            LoraError::KeyMismatch { key } => {
                write!(f, "Key '{}' missing in one of the modules.", key)
            }
        }
    }
}

impl std::error::Error for LoraError {}
