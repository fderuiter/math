//! Error types for Kelly Criterion calculations.

use std::fmt;

/// Errors that can occur in Kelly Criterion calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum KellyError {
    /// Invalid probability (must be between 0 and 1).
    InvalidProbability { value: f64 },
    /// Invalid odds (must be greater than 1.0 for decimal odds).
    InvalidOdds { value: f64 },
    /// Invalid fraction (must be between 0 and 1).
    InvalidFraction { value: f64 },
    /// No edge (negative expected value - should not bet).
    NoEdge { probability: f64, odds: f64 },
    /// Invalid bankroll amount (must be positive).
    InvalidBankroll { value: f64 },
}

impl fmt::Display for KellyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidProbability { value } => {
                write!(
                    f,
                    "Invalid probability: {} (must be between 0 and 1)",
                    value
                )
            }
            Self::InvalidOdds { value } => {
                write!(f, "Invalid odds: {} (must be > 1.0)", value)
            }
            Self::InvalidFraction { value } => {
                write!(f, "Invalid fraction: {} (must be between 0 and 1)", value)
            }
            Self::NoEdge { probability, odds } => {
                write!(
                    f,
                    "No edge: p={}, odds={} results in negative expectation",
                    probability, odds
                )
            }
            Self::InvalidBankroll { value } => {
                write!(f, "Invalid bankroll: {} (must be positive)", value)
            }
        }
    }
}

impl std::error::Error for KellyError {}
