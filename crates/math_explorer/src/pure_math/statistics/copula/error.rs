//! Error types for copula operations.

use std::fmt;

/// Errors that can occur during copula operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CopulaError {
    /// Invalid probability (must be in [0, 1]).
    InvalidProbability { value: f64 },
    /// Invalid correlation (must be in [-1, 1]).
    InvalidCorrelation { value: f64 },
    /// Dimension mismatch in matrices or vectors.
    DimensionMismatch { expected: usize, actual: usize },
    /// Matrix is not positive definite.
    NotPositiveDefinite,
    /// Matrix is not symmetric.
    NotSymmetric,
    /// Numerical computation failed.
    NumericalError { reason: String },
}

impl fmt::Display for CopulaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidProbability { value } => {
                write!(f, "Invalid probability: {} (must be in [0, 1])", value)
            }
            Self::InvalidCorrelation { value } => {
                write!(f, "Invalid correlation: {} (must be in [-1, 1])", value)
            }
            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            Self::NotPositiveDefinite => {
                write!(f, "Correlation matrix is not positive definite")
            }
            Self::NotSymmetric => {
                write!(f, "Correlation matrix is not symmetric")
            }
            Self::NumericalError { reason } => {
                write!(f, "Numerical error: {}", reason)
            }
        }
    }
}

impl std::error::Error for CopulaError {}
