//! Error types for Glicko-2 rating system.

use std::fmt;

/// Errors that can occur in Glicko-2 rating calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum Glicko2Error {
    /// Invalid rating value.
    InvalidRating { value: f64 },
    /// Invalid rating deviation (must be positive).
    InvalidRatingDeviation { value: f64 },
    /// Invalid volatility (must be positive).
    InvalidVolatility { value: f64 },
    /// Invalid system constant tau (must be between 0.3 and 1.2).
    InvalidSystemConstant { value: f64 },
    /// Invalid opponent count (must be at least 1).
    InvalidOpponentCount { count: usize },
    /// Volatility convergence failed after maximum iterations.
    VolatilityConvergenceFailed { iterations: usize },
    /// Invalid score (must be between 0 and 1).
    InvalidScore { value: f64 },
    /// Empty match results provided.
    EmptyMatchResults,
}

impl fmt::Display for Glicko2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidRating { value } => {
                write!(f, "Invalid rating: {} (must be finite)", value)
            }
            Self::InvalidRatingDeviation { value } => {
                write!(
                    f,
                    "Invalid rating deviation: {} (must be positive and finite)",
                    value
                )
            }
            Self::InvalidVolatility { value } => {
                write!(
                    f,
                    "Invalid volatility: {} (must be positive and finite)",
                    value
                )
            }
            Self::InvalidSystemConstant { value } => {
                write!(
                    f,
                    "Invalid system constant tau: {} (must be between 0.3 and 1.2)",
                    value
                )
            }
            Self::InvalidOpponentCount { count } => {
                write!(f, "Invalid opponent count: {} (must be at least 1)", count)
            }
            Self::VolatilityConvergenceFailed { iterations } => {
                write!(
                    f,
                    "Volatility convergence failed after {} iterations",
                    iterations
                )
            }
            Self::InvalidScore { value } => {
                write!(f, "Invalid score: {} (must be between 0 and 1)", value)
            }
            Self::EmptyMatchResults => {
                write!(f, "Empty match results provided")
            }
        }
    }
}

impl std::error::Error for Glicko2Error {}
