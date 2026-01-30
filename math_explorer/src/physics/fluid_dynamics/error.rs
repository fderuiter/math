use std::fmt;

/// Errors that can occur during fluid dynamics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum FluidError {
    /// The Laplacian of the velocity field is required but was not provided.
    MissingLaplacian,
    /// Fluid density is zero, causing division by zero.
    ZeroDensity,
    /// Invalid configuration of fluid properties or state.
    InvalidConfiguration(String),
}

impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::MissingLaplacian => {
                write!(
                    f,
                    "Navier-Stokes equation requires the Laplacian of the velocity field."
                )
            }
            FluidError::ZeroDensity => write!(f, "Fluid density cannot be zero."),
            FluidError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for FluidError {}
