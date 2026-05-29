use crate::physics::solid_state::types::ElectronVolts;
use thiserror::Error;
use math_core::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur during chaos analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum ChaosError {
    /// The simulation parameters are invalid (e.g., negative time step).
    InvalidParameter(String),
    /// The calculation failed (e.g., trajectories converged completely).
    CalculationError(String),
}


impl fmt::Display for ChaosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChaosError::InvalidParameter(msg) => write!(f, "Invalid chaos parameter: {}", msg),
            ChaosError::CalculationError(msg) => write!(f, "Chaos calculation error: {}", msg),
        }
    }
}


impl std::error::Error for ChaosError {}


impl Diagnostic for ChaosError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "ChaosError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


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
            Self::InvalidEnergyScale { scale, context } => write!(
                f,
                "Invalid energy scale in {}: got {}, expected > 0",
                context, scale
            ),
            Self::InvalidCouplingConstant { alpha } => {
                write!(f, "Coupling constant must be positive, got {}", alpha)
            }
            Self::LandauPole { scale } => write!(
                f,
                "Landau pole encountered: coupling diverges at scale {}",
                scale
            ),
            Self::InvalidFlavors { nf } => {
                write!(f, "Number of flavors must be non-negative, got {}", nf)
            }
        }
    }
}


impl std::error::Error for StandardModelError {}


impl Diagnostic for StandardModelError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "StandardModelError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Errors that can occur during Solid State Physics calculations.
#[derive(Error, Debug, PartialEq)]
pub enum SolidStateError {
    /// The iterative solver failed to converge within the maximum number of iterations.
    #[error("Convergence failed after {0} iterations. Last value: {1}")]
    ConvergenceFailure(usize, ElectronVolts),

    /// A parameter provided to the model or solver was invalid.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// A general computation error occurred.
    #[error("Computation error: {0}")]
    ComputationError(String),
}


impl Diagnostic for SolidStateError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "SolidStateError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


#[derive(Error, Debug, PartialEq)]
pub enum DoseFluenceError {
    #[error("Radius cannot be zero (singularity at r=0)")]
    Singularity,
    #[error("Radius must be non-negative")]
    NegativeRadius,
    #[error("Physical quantity must be non-negative: {0}")]
    InvalidPhysicalQuantity(String),
}


impl Diagnostic for DoseFluenceError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "DoseFluenceError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


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


impl Diagnostic for HighEnergyError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "HighEnergyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Errors for Fluid Dynamics.


/// Errors related to fluid properties and dynamics.
#[derive(Debug, Clone, PartialEq)]
pub enum FluidError {
    /// Density must be strictly positive.
    InvalidDensity { value: f64 },
    /// Dynamic viscosity must be non-negative.
    InvalidViscosity { value: f64 },
}


impl fmt::Display for FluidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FluidError::InvalidDensity { value } => {
                write!(f, "Density must be strictly positive, got {}", value)
            }
            FluidError::InvalidViscosity { value } => {
                write!(f, "Dynamic viscosity must be non-negative, got {}", value)
            }
        }
    }
}


impl std::error::Error for FluidError {}


impl Diagnostic for FluidError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "FluidError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


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


impl Diagnostic for StatMechError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "StatMechError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Errors for Radar Gating processing.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum RadarError {
    /// Chirp length mismatch.
    #[error("Chirp length {actual} does not match expected {expected}")]
    ChirpLengthMismatch { expected: usize, actual: usize },

    /// Insufficient snapshots for MUSIC algorithm.
    #[error("Not enough snapshots to compute stable Covariance Matrix: {actual} < {required}")]
    InsufficientSnapshots { required: usize, actual: usize },

    /// Signal subspace dimension error (e.g. >= samples).
    #[error("Signal subspace dimension {subspace} equals or exceeds sample size {samples}")]
    InvalidSignalSubspace { samples: usize, subspace: usize },

    /// Invalid configuration parameters.
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    /// Numerical instability detected (e.g. NaN/Inf).
    #[error("Numerical instability detected: {0}")]
    NumericalInstability(String),
}


impl Diagnostic for RadarError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "RadarError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}
