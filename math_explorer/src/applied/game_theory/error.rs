use std::fmt;

/// Errors for Game Theory calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum GameTheoryError {
    /// Payoff matrix is not square.
    NonSquarePayoffMatrix { rows: usize, cols: usize },
    /// Invalid parameter value (e.g. frequency outside [0, 1]).
    InvalidParameter { name: String, value: f64 },
    /// Population has gone extinct (sum close to zero), preventing normalization.
    PopulationExtinction,
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
            Self::PopulationExtinction => {
                write!(f, "Population extinction: sum of proportions is zero.")
            }
        }
    }
}

impl std::error::Error for GameTheoryError {}
