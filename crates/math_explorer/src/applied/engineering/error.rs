use std::fmt;

/// Errors related to Engineering Calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineeringError {
    /// Invalid parameter value (e.g. TotalBits = 0).
    InvalidParameter { name: String, value: f64 },
}

impl fmt::Display for EngineeringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
        }
    }
}

impl std::error::Error for EngineeringError {}
