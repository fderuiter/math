use pure_math::analysis::roots::AnalysisError;
use thiserror::Error;
use math_core::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;

/// Errors that can occur during LoRA ensemble combination.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoraError {
    #[error("Ensemble is empty; cannot combine.")]
    EmptyEnsemble,
    #[error("Weights cannot be empty.")]
    EmptyWeights,
    #[error("The number of weights must match the number of modules in the ensemble.")]
    WeightModuleMismatch,
    #[error("Mismatched tensor shapes for the same key.")]
    TensorShapeMismatch,
    #[error("Mismatched keys between LoRA modules.")]
    KeyMismatch,
}


impl Diagnostic for LoraError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "LoraError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Errors that can occur during isosurface extraction.
#[derive(Debug, Clone, PartialEq)]
pub enum IsosurfaceError {
    /// The grid dimensions are too small (must be at least 2x2x2).
    InvalidGrid(String),
    /// The data buffer size matches the grid dimensions.
    DataMismatch { expected: usize, actual: usize },
}


impl fmt::Display for IsosurfaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IsosurfaceError::InvalidGrid(msg) => write!(f, "Invalid grid dimensions: {}", msg),
            IsosurfaceError::DataMismatch { expected, actual } => {
                write!(f, "Data mismatch: expected {}, got {}", expected, actual)
            }
        }
    }
}


impl std::error::Error for IsosurfaceError {}


impl Diagnostic for IsosurfaceError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "IsosurfaceError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Error types for battery degradation modeling.


/// Errors that can occur when instantiating battery parameters.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum BatteryError {
    /// Depth of Discharge must be between 0.0 and 100.0.
    #[error("DepthOfDischarge must be between 0.0 and 100.0, got {0}")]
    InvalidDepthOfDischarge(f64),
    /// Capacity must be between 0.0 and 1.0.
    #[error("Capacity must be between 0.0 and 1.0, got {0}")]
    InvalidCapacity(f64),
    /// Cycles cannot be negative.
    #[error("Cycles cannot be negative, got {0}")]
    NegativeCycles(f64),
}


impl Diagnostic for BatteryError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "BatteryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


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


impl Diagnostic for GameTheoryError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "GameTheoryError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Errors related to Engineering Calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EngineeringError {
    /// Invalid parameter value (e.g. TotalBits = 0).
    InvalidParameter { name: String, value: f64 },
}


impl fmt::Display for EngineeringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
        }
    }
}


impl std::error::Error for EngineeringError {}


impl Diagnostic for EngineeringError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "EngineeringError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}


/// Errors that can occur in pharmacokinetic modeling.
#[derive(Debug, Error)]
pub enum PharmacokineticsError {
    /// An invalid parameter was provided (e.g., negative volume or rate constant).
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// A numerical analysis method failed (e.g., root finding did not converge).
    #[error("Analysis error: {0}")]
    AnalysisError(#[from] AnalysisError),
}


impl Diagnostic for PharmacokineticsError {
    fn severity(&self) -> Severity { Severity::Error }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "PharmacokineticsError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}
