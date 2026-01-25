use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum BatteryError {
    InvalidDepthOfDischarge(f64),
    InvalidCapacity(f64),
    NegativeCycles(f64),
}

impl fmt::Display for BatteryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDepthOfDischarge(val) => write!(
                f,
                "DepthOfDischarge must be between 0.0 and 100.0, got {}",
                val
            ),
            Self::InvalidCapacity(val) => {
                write!(f, "Capacity must be between 0.0 and 1.0, got {}", val)
            }
            Self::NegativeCycles(val) => write!(f, "Cycles cannot be negative, got {}", val),
        }
    }
}

impl std::error::Error for BatteryError {}
