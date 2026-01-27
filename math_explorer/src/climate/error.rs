use std::fmt;

#[derive(Debug, Clone)]
pub enum ClimateError {
    ShapeMismatch {
        expected: (usize, usize),
        actual: (usize, usize),
        message: String,
    },
    InvalidConfiguration(String),
}

impl fmt::Display for ClimateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ShapeMismatch {
                expected,
                actual,
                message,
            } => write!(
                f,
                "Shape mismatch: expected {:?}, got {:?}. {}",
                expected, actual, message
            ),
            Self::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for ClimateError {}
