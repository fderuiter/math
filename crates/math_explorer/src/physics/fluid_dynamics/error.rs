//! Errors for Fluid Dynamics.

use std::fmt;

/// Errors related to fluid properties and dynamics.
#[derive(Debug, Clone, PartialEq)]
pub enum FluidError {
    /// Density must be strictly positive.
    InvalidDensity { value: f64 },
    /// Dynamic viscosity must be non-negative.
    InvalidViscosity { value: f64 },
}

impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::InvalidDensity { value } => {
                write!(f, "Density must be strictly positive, got {}", value)
            }
            FluidError::InvalidViscosity { value } => {
                write!(f, "Dynamic viscosity must be non-negative, got {}", value)
            }
        }
    }
}

impl std::error::Error for FluidError {}
