use std::fmt;

/// Errors related to Statistical Mechanics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum StatMechError {
    /// Temperature must be non-negative (absolute zero is allowed as a limit).
    InvalidTemperature(f64),
    /// Chemical potential is invalid for the particle type and state (e.g., > Energy for Bosons).
    InvalidChemicalPotential {
        chemical_potential: f64,
        energy: f64,
        reason: String,
    },
    /// Numerical instability (e.g. division by zero).
    NumericalInstability(String),
}

impl fmt::Display for StatMechError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatMechError::InvalidTemperature(t) => write!(f, "Invalid temperature: {} K", t),
            StatMechError::InvalidChemicalPotential {
                chemical_potential,
                energy,
                reason,
            } => write!(
                f,
                "Invalid chemical potential mu={} for E={}: {}",
                chemical_potential, energy, reason
            ),
            StatMechError::NumericalInstability(msg) => write!(f, "Numerical instability: {}", msg),
        }
    }
}

impl std::error::Error for StatMechError {}
