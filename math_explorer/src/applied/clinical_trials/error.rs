use std::fmt;

/// Errors that can occur during Survival Analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SurvivalError {
    /// Negative time value encountered.
    NegativeTime,
    /// Total observation time is zero or negative.
    ZeroTotalTime(String),
    /// No events occurred, making hazard ratio calculation impossible (infinite).
    NoEvents(String),
}

impl fmt::Display for SurvivalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SurvivalError::NegativeTime => write!(f, "Negative time values encountered"),
            SurvivalError::ZeroTotalTime(msg) => write!(f, "Total time is zero or negative: {}", msg),
            SurvivalError::NoEvents(msg) => write!(f, "No events observed: {}", msg),
        }
    }
}

impl std::error::Error for SurvivalError {}
