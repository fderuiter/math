use std::fmt;

/// Errors related to Standard Model calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum StandardModelError {
    /// Energy scale must be positive.
    InvalidEnergyScale { scale: f64, context: String },
    /// Coupling constant must be positive.
    InvalidCouplingConstant { alpha: f64 },
    /// The coupling constant diverges at this scale (Landau Pole).
    LandauPole { scale: f64 },
    /// Invalid number of quark flavors (e.g., negative).
    InvalidFlavors { nf: f64 },
}

impl fmt::Display for StandardModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidEnergyScale { scale, context } => write!(f, "Invalid energy scale in {}: got {}, expected > 0", context, scale),
            Self::InvalidCouplingConstant { alpha } => write!(f, "Coupling constant must be positive, got {}", alpha),
            Self::LandauPole { scale } => write!(f, "Landau pole encountered: coupling diverges at scale {}", scale),
            Self::InvalidFlavors { nf } => write!(f, "Number of flavors must be non-negative, got {}", nf),
        }
    }
}

impl std::error::Error for StandardModelError {}
