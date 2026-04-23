//! Error types for Markov chain operations.

use std::fmt;

/// Errors that can occur during Markov chain operations.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkovError {
    /// Invalid probability value (must be in [0, 1] and finite).
    InvalidProbability {
        /// The invalid probability value.
        value: f64,
    },

    /// Matrix dimension mismatch.
    DimensionMismatch {
        /// Expected dimension.
        expected: usize,
        /// Actual dimension.
        actual: usize,
    },

    /// Matrix is not stochastic (rows don't sum to 1).
    NotStochastic {
        /// Description of the issue.
        reason: String,
    },

    /// Matrix is not a valid generator (rows don't sum to 0 or invalid rates).
    InvalidGenerator {
        /// Description of the issue.
        reason: String,
    },

    /// Numerical computation error.
    NumericalError {
        /// Description of the error.
        reason: String,
    },

    /// Invalid state specification.
    InvalidState {
        /// Description of the issue.
        reason: String,
    },

    /// Time index out of bounds.
    TimeIndexOutOfBounds {
        /// The requested time.
        time: f64,
        /// Valid time range.
        valid_range: (f64, f64),
    },

    /// Observation sequence incompatible with model.
    InvalidObservation {
        /// Description of the issue.
        reason: String,
    },

    /// Matrix not invertible or singular.
    SingularMatrix {
        /// Context about which matrix.
        context: String,
    },
}

impl fmt::Display for MarkovError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkovError::InvalidProbability { value } => {
                write!(
                    f,
                    "Invalid probability value: {} (must be in [0, 1])",
                    value
                )
            }
            MarkovError::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            MarkovError::NotStochastic { reason } => {
                write!(f, "Matrix is not stochastic: {}", reason)
            }
            MarkovError::InvalidGenerator { reason } => {
                write!(f, "Invalid generator matrix: {}", reason)
            }
            MarkovError::NumericalError { reason } => {
                write!(f, "Numerical error: {}", reason)
            }
            MarkovError::InvalidState { reason } => {
                write!(f, "Invalid state: {}", reason)
            }
            MarkovError::TimeIndexOutOfBounds { time, valid_range } => {
                write!(
                    f,
                    "Time {} is out of bounds (valid range: [{}, {}])",
                    time, valid_range.0, valid_range.1
                )
            }
            MarkovError::InvalidObservation { reason } => {
                write!(f, "Invalid observation: {}", reason)
            }
            MarkovError::SingularMatrix { context } => {
                write!(f, "Singular matrix: {}", context)
            }
        }
    }
}

impl std::error::Error for MarkovError {}

/// Type alias for Results with MarkovError.
pub type Result<T> = std::result::Result<T, MarkovError>;
