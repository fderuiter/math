//! Errors for Fluid Dynamics.

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FluidError {
    /// Indicates that the fluid properties are invalid (e.g., negative density).
    InvalidProperties(String),
    /// Indicates that the Laplacian of the velocity field is required but missing.
    MissingLaplacian,
}

impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::InvalidProperties(msg) => write!(f, "Invalid Fluid Properties: {}", msg),
            FluidError::MissingLaplacian => {
                write!(
                    f,
                    "Missing Laplacian of velocity (required for Viscous flow)"
                )
            }
        }
    }
}

impl Error for FluidError {}
