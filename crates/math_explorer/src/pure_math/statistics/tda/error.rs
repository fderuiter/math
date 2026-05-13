//! Error types for Topological Data Analysis.

use std::fmt;

/// Errors that can occur in TDA computations.
#[derive(Debug, Clone, PartialEq)]
pub enum TdaError {
    /// Empty point cloud provided.
    EmptyPointCloud,
    /// Invalid radius (must be non-negative).
    InvalidRadius { value: f64 },
    /// Invalid dimension for Betti number computation.
    InvalidDimension { dimension: usize },
    /// Insufficient points for the requested operation.
    InsufficientPoints { required: usize, actual: usize },
    /// Invalid simplex (e.g., duplicate vertices).
    InvalidSimplex { reason: String },
    /// Matrix computation error.
    MatrixError { reason: String },
}

impl fmt::Display for TdaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyPointCloud => {
                write!(f, "Empty point cloud provided")
            }
            Self::InvalidRadius { value } => {
                write!(f, "Invalid radius: {} (must be non-negative)", value)
            }
            Self::InvalidDimension { dimension } => {
                write!(
                    f,
                    "Invalid dimension: {} (only 0 and 1 are supported)",
                    dimension
                )
            }
            Self::InsufficientPoints { required, actual } => {
                write!(
                    f,
                    "Insufficient points: required at least {}, got {}",
                    required, actual
                )
            }
            Self::InvalidSimplex { reason } => {
                write!(f, "Invalid simplex: {}", reason)
            }
            Self::MatrixError { reason } => {
                write!(f, "Matrix computation error: {}", reason)
            }
        }
    }
}

impl std::error::Error for TdaError {}
