use std::fmt;

/// Errors for Game Theory calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum GameTheoryError {
    /// Payoff matrix is not square.
    NonSquarePayoffMatrix { rows: usize, cols: usize },
    /// Invalid parameter value (e.g. frequency outside [0, 1]).
    InvalidParameter { name: String, value: f64 },
}

impl fmt::Display for GameTheoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonSquarePayoffMatrix { rows, cols } => write!(
                f,
                "Payoff matrix must be square, but dimensions are ({}, {})",
                rows, cols
            ),
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
        }
    }
}

impl std::error::Error for GameTheoryError {}
