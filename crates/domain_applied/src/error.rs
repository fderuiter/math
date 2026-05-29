use pure_math::pure_math::analysis::roots::AnalysisError;
use thiserror::Error;
use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Error)]
pub enum PharmacokineticsError {
    /// An invalid parameter was provided (e.g., negative volume or rate constant).
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// A numerical analysis method failed (e.g., root finding did not converge).
    #[error("Analysis error: {0}")]
    AnalysisError(#[from] AnalysisError),
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum GameTheoryError {
    /// Payoff matrix is not square.
    NonSquarePayoffMatrix { rows: usize, cols: usize },
    /// Invalid parameter value (e.g. frequency outside [0, 1]).
    InvalidParameter { name: String, value: f64 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EngineeringError {
    /// Invalid parameter value (e.g. TotalBits = 0).
    InvalidParameter { name: String, value: f64 },
}

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

#[derive(Debug, Clone, PartialEq)]
pub enum IsosurfaceError {
    /// The grid dimensions are too small (must be at least 2x2x2).
    InvalidGrid(String),
    /// The data buffer size matches the grid dimensions.
    DataMismatch { expected: usize, actual: usize },
}

