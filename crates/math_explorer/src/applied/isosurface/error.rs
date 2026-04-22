use std::fmt;

/// Errors that can occur during isosurface extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum IsosurfaceError {
    /// The grid dimensions are too small (must be at least 2x2x2).
    InvalidGrid(String),
    /// The data buffer size matches the grid dimensions.
    DataMismatch { expected: usize, actual: usize },
}

impl fmt::Display for IsosurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsosurfaceError::InvalidGrid(msg) => write!(f, "Invalid grid dimensions: {}", msg),
            IsosurfaceError::DataMismatch { expected, actual } => {
                write!(f, "Data mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}

impl std::error::Error for IsosurfaceError {}
