//! Errors for the Algorithm module.

use std::fmt;

/// Errors that can occur during algorithm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgorithmError {
    /// Indicates that a matrix operation failed because the matrix was singular (non-invertible).
    SingularMatrix,
    /// Indicates that input dimensions do not match the expected dimensions (e.g., matrix multiplication).
    DimensionMismatch { expected: String, actual: String },
    /// Indicates invalid input parameters.
    InvalidInput(String),
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgorithmError::SingularMatrix => write!(f, "Matrix is singular and cannot be inverted."),
            AlgorithmError::DimensionMismatch { expected, actual } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, actual)
            }
            AlgorithmError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for AlgorithmError {}
