use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ClimateError {
    InvalidConfiguration {
        reason: String,
        parameter: String,
        value: String,
    },
}

impl fmt::Display for ClimateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration {
                reason,
                parameter,
                value,
            } => {
                write!(
                    f,
                    "Invalid configuration for {}: {} (value: {})",
                    parameter, reason, value
                )
            }
        }
    }
}

impl std::error::Error for ClimateError {}
