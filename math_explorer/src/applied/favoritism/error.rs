use std::fmt;

/// Errors that can occur during favoritism score calculation.
#[derive(Debug, Clone)]
pub enum FavoritismError {
    /// Invalid input parameter (e.g. NaN, Infinity, or negative where not allowed).
    InvalidInput(String),
}

impl fmt::Display for FavoritismError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for FavoritismError {}
