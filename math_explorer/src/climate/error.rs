//! Error types for the CERA climate framework.

use std::fmt;

/// Errors that can occur in the CERA climate framework.
#[derive(Debug, Clone)]
pub enum ClimateError {
    /// Dimension mismatch between matrices (e.g., in loss calculation).
    DimensionMismatch { expected: String, actual: String },
    /// Invalid configuration parameters.
    InvalidConfig(String),
    /// Error during calculation (e.g., NaN encountered).
    CalculationError(String),
}

impl fmt::Display for ClimateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DimensionMismatch { expected, actual } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, actual)
            }
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            Self::CalculationError(msg) => write!(f, "Calculation error: {}", msg),
        }
    }
}

impl std::error::Error for ClimateError {}
