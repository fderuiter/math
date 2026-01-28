use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FavoritismError {
    InvalidInput(String),
}

impl fmt::Display for FavoritismError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid favoritism input: {}", msg),
        }
    }
}

impl std::error::Error for FavoritismError {}
