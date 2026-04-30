use std::fmt;

/// Errors related to High Energy Physics calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum HighEnergyError {
    /// Mass must be positive.
    InvalidMass { mass: f64 },
    /// Radius must be greater than Schwarzschild radius.
    InvalidRadius { radius: f64, limit: f64 },
    /// Energy density cannot be negative.
    InvalidEnergyDensity { u_b: f64 },
    /// Lorentz factor must be >= 1.
    InvalidLorentzFactor { gamma: f64 },
    /// Power law index p must be > 1.
    InvalidPowerLawIndex { p: f64 },
    /// Adiabatic index must be > 1.
    InvalidAdiabaticIndex { gamma: f64 },
    /// Density must be positive.
    InvalidDensity { rho: f64 },
    /// Pressure cannot be negative.
    InvalidPressure { p: f64 },
    /// Velocity is invalid (e.g. >= c or < 0 if speed).
    InvalidVelocity { v: f64 },
    /// Momentum is invalid (e.g. m^2 + p^2 < 0 somehow? or just checking validity).
    InvalidMomentum,
    /// Invalid statistics parameters (e.g. negative counts).
    InvalidStatisticsParams { reason: String },
    /// Error during calculation (e.g. negative sqrt).
    CalculationError { reason: String },
}

impl fmt::Display for HighEnergyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMass { mass } => write!(f, "Mass must be positive, got {}", mass),
            Self::InvalidRadius { radius, limit } => write!(
                f,
                "Radius {} must be greater than Schwarzschild radius {}",
                radius, limit
            ),
            Self::InvalidEnergyDensity { u_b } => {
                write!(f, "Energy density must be non-negative, got {}", u_b)
            }
            Self::InvalidLorentzFactor { gamma } => {
                write!(f, "Lorentz factor must be >= 1, got {}", gamma)
            }
            Self::InvalidPowerLawIndex { p } => write!(f, "Power law index must be > 1, got {}", p),
            Self::InvalidAdiabaticIndex { gamma } => {
                write!(f, "Adiabatic index must be > 1, got {}", gamma)
            }
            Self::InvalidDensity { rho } => write!(f, "Density must be positive, got {}", rho),
            Self::InvalidPressure { p } => write!(f, "Pressure must be non-negative, got {}", p),
            Self::InvalidVelocity { v } => write!(f, "Velocity {} is invalid (must be < c)", v),
            Self::InvalidMomentum => write!(f, "Momentum vector is invalid"),
            Self::InvalidStatisticsParams { reason } => {
                write!(f, "Invalid statistics parameters: {}", reason)
            }
            Self::CalculationError { reason } => write!(f, "Calculation error: {}", reason),
        }
    }
}

impl std::error::Error for HighEnergyError {}
