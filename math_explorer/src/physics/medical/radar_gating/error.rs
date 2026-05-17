use thiserror::Error;

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
