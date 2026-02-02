use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MedicalPhysicsError {
    InvalidRadius { radius: f64, message: String },
    NegativeDensity { density: f64 },
    InvalidInput { message: String },
    CalculationError { message: String },
}

impl fmt::Display for MedicalPhysicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRadius { radius, message } => {
                write!(f, "Invalid radius {}: {}", radius, message)
            }
            Self::NegativeDensity { density } => {
                write!(f, "Density cannot be negative: {}", density)
            }
            Self::InvalidInput { message } => write!(f, "Invalid input: {}", message),
            Self::CalculationError { message } => write!(f, "Calculation error: {}", message),
        }
    }
}

impl std::error::Error for MedicalPhysicsError {}
