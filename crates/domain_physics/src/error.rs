use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

use crate::physics::solid_state::types::ElectronVolts;

#[derive(Debug, Clone, PartialEq)]
pub enum ChaosError {
    /// The simulation parameters are invalid (e.g., negative time step).
    InvalidParameter(String),
    /// The calculation failed (e.g., trajectories converged completely).
    CalculationError(String),
}

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

#[derive(Error, Debug, PartialEq)]
pub enum DoseFluenceError {
    #[error("Radius cannot be zero (singularity at r=0)")]
    Singularity,
    #[error("Radius must be non-negative")]
    NegativeRadius,
    #[error("Physical quantity must be non-negative: {0}")]
    InvalidPhysicalQuantity(String),
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum FluidError {
    /// Density must be strictly positive.
    InvalidDensity { value: f64 },
    /// Dynamic viscosity must be non-negative.
    InvalidViscosity { value: f64 },
}

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

impl Diagnostic for ChaosError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "ChaosError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for StandardModelError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "StandardModelError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for SolidStateError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "SolidStateError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for DoseFluenceError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "DoseFluenceError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for RadarError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "RadarError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for HighEnergyError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "HighEnergyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for FluidError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "FluidError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl Diagnostic for StatMechError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "StatMechError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl std::fmt::Display for ChaosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ChaosError {}

impl std::fmt::Display for StandardModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StandardModelError {}

impl std::fmt::Display for HighEnergyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for HighEnergyError {}

impl std::fmt::Display for FluidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for FluidError {}

impl std::fmt::Display for StatMechError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for StatMechError {}
