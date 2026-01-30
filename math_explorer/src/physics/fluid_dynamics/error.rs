//! Errors for Fluid Dynamics.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluidError {
    /// The Laplacian of velocity is required but was not provided.
    MissingLaplacian,
    /// Generic invalid configuration.
    InvalidConfiguration(String),
}

impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::MissingLaplacian => {
                write!(f, "Laplacian of velocity is required for this operation")
            }
            FluidError::InvalidConfiguration(msg) => {
                write!(f, "Invalid fluid configuration: {}", msg)
            }
        }
    }
}

impl std::error::Error for FluidError {}
