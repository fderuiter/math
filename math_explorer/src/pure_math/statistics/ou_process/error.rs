//! Error types for Ornstein-Uhlenbeck process.

use std::fmt;

/// Errors that can occur during OU process simulation.
#[derive(Debug, Clone, PartialEq)]
pub enum OuError {
    /// Invalid mean reversion rate (must be positive).
    InvalidMeanReversionRate { value: f64 },
    /// Invalid volatility (must be non-negative).
    InvalidVolatility { value: f64 },
    /// Invalid time step (must be positive).
    InvalidTimeStep { value: f64 },
    /// Invalid simulation parameters.
    InvalidSimulationParams { reason: String },
    /// Insufficient data for parameter estimation.
    InsufficientData { required: usize, actual: usize },
}

impl fmt::Display for OuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidMeanReversionRate { value } => {
                write!(
                    f,
                    "Invalid mean reversion rate: {} (must be positive)",
                    value
                )
            }
            Self::InvalidVolatility { value } => {
                write!(f, "Invalid volatility: {} (must be non-negative)", value)
            }
            Self::InvalidTimeStep { value } => {
                write!(f, "Invalid time step: {} (must be positive)", value)
            }
            Self::InvalidSimulationParams { reason } => {
                write!(f, "Invalid simulation parameters: {}", reason)
            }
            Self::InsufficientData { required, actual } => {
                write!(
                    f,
                    "Insufficient data: required at least {}, got {}",
                    required, actual
                )
            }
        }
    }
}

impl std::error::Error for OuError {}
