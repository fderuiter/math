use std::fmt;

/// Errors that can occur in the CERA climate modeling framework.
#[derive(Debug, Clone, PartialEq)]
pub enum ClimateError {
    /// Dimension mismatch between tensors/matrices.
    DimensionMismatch(String),
    /// Error during sorting (e.g., NaN encountered in safe sort wrapper, though total_cmp handles it).
    SortError(String),
    /// Invalid configuration parameters.
    InvalidConfig(String),
    /// Generic error message.
    Other(String),
}

impl fmt::Display for ClimateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch(msg) => write!(f, "Dimension Mismatch: {}", msg),
            Self::SortError(msg) => write!(f, "Sort Error: {}", msg),
            Self::InvalidConfig(msg) => write!(f, "Invalid Configuration: {}", msg),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for ClimateError {}
