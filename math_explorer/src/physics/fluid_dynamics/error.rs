//! Error types for Fluid Dynamics.

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluidError {
    /// The Laplacian of velocity is required but was not provided.
    MissingLaplacian,
    /// Invalid configuration for the flow model.
    InvalidConfiguration(String),
    /// Other errors.
    Other(String),
}

impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::MissingLaplacian => write!(f, "Missing Laplacian of velocity"),
            FluidError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
            FluidError::Other(msg) => write!(f, "Fluid dynamics error: {}", msg),
        }
    }
}

impl Error for FluidError {}
