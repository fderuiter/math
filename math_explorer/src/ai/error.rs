use thiserror::Error;

/// Errors related to AI/Machine Learning calculations.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AIError {
    /// Dimension mismatch between tensors/matrices.
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: String, got: String },

    /// Invalid parameter value (e.g. negative learning rate).
    #[error("Invalid parameter {name}: {value}")]
    InvalidParameter { name: String, value: f64 },

    /// Optimization failed or diverged.
    #[error("Optimization diverged")]
    OptimizationDivergence,

    /// Type conversion failed.
    #[error("Conversion error: {reason}")]
    ConversionError { reason: String },

    /// State is uninitialized.
    #[error("Uninitialized state: {name}")]
    UninitializedState { name: String },

    /// Mismatched data distribution lengths.
    #[error("Distributions have different lengths: expected {expected}, got {got}")]
    DistributionLengthMismatch { expected: usize, got: usize },
}
