use std::fmt;

/// Errors related to AI/Machine Learning calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum AIError {
    /// Dimension mismatch between tensors/matrices.
    DimensionMismatch { expected: String, got: String },
    /// Invalid parameter value (e.g. negative learning rate).
    InvalidParameter { name: String, value: f64 },
    /// Missing required parameter for model construction.
    MissingParameter { name: String },
    /// Optimization failed or diverged.
    OptimizationDivergence,
}

impl fmt::Display for AIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, got)
            }
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
            Self::MissingParameter { name } => {
                write!(f, "Missing required parameter: {}", name)
            }
            Self::OptimizationDivergence => write!(f, "Optimization diverged"),
        }
    }
}

impl std::error::Error for AIError {}
