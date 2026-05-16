use std::fmt;

/// Errors that can occur during chaos analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ChaosError {
    /// The simulation parameters are invalid (e.g., negative time step).
    InvalidParameter(String),
    /// The calculation failed (e.g., trajectories converged completely).
    CalculationError(String),
}

impl fmt::Display for ChaosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChaosError::InvalidParameter(msg) => write!(f, "Invalid chaos parameter: {}", msg),
            ChaosError::CalculationError(msg) => write!(f, "Chaos calculation error: {}", msg),
        }
    }
}

impl std::error::Error for ChaosError {}
