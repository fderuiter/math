//! Error types for Zero-Inflated Poisson regression.

use std::fmt;

/// Errors that can occur during ZIP regression analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ZipError {
    /// Invalid probability parameter (must be in [0, 1]).
    InvalidProbability { value: f64, parameter: String },
    /// Invalid rate parameter (must be positive).
    InvalidRate { value: f64 },
    /// Invalid count value (must be non-negative integer).
    InvalidCount { value: f64 },
    /// Insufficient data for regression.
    InsufficientData { required: usize, actual: usize },
    /// Regression convergence failed.
    ConvergenceFailed { iterations: usize, reason: String },
    /// Invalid predictor matrix dimensions.
    InvalidDimensions { expected: String, actual: String },
}

impl fmt::Display for ZipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidProbability { value, parameter } => {
                write!(
                    f,
                    "Invalid probability for {}: {} (must be in [0, 1])",
                    parameter, value
                )
            }
            Self::InvalidRate { value } => {
                write!(f, "Invalid rate parameter: {} (must be positive)", value)
            }
            Self::InvalidCount { value } => {
                write!(
                    f,
                    "Invalid count value: {} (must be non-negative integer)",
                    value
                )
            }
            Self::InsufficientData { required, actual } => {
                write!(
                    f,
                    "Insufficient data: required at least {}, got {}",
                    required, actual
                )
            }
            Self::ConvergenceFailed { iterations, reason } => {
                write!(
                    f,
                    "Regression failed to converge after {} iterations: {}",
                    iterations, reason
                )
            }
            Self::InvalidDimensions { expected, actual } => {
                write!(
                    f,
                    "Invalid matrix dimensions: expected {}, got {}",
                    expected, actual
                )
            }
        }
    }
}

impl std::error::Error for ZipError {}
