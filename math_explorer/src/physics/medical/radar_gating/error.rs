use std::fmt;

/// Errors for Radar Gating processing.
#[derive(Debug, Clone, PartialEq)]
pub enum RadarError {
    /// Chirp length mismatch.
    ChirpLengthMismatch { expected: usize, actual: usize },
    /// Insufficient snapshots for MUSIC algorithm.
    InsufficientSnapshots { required: usize, actual: usize },
    /// Signal subspace dimension error (e.g. >= samples).
    InvalidSignalSubspace { samples: usize, subspace: usize },
    /// Eigenvalue decomposition failed or produced NaNs.
    NumericalInstability,
}

impl fmt::Display for RadarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ChirpLengthMismatch { expected, actual } => write!(
                f,
                "Chirp length {} does not match expected {}",
                actual, expected
            ),
            Self::InsufficientSnapshots { required, actual } => write!(
                f,
                "Not enough snapshots to compute stable Covariance Matrix: {} < {}",
                actual, required
            ),
            Self::InvalidSignalSubspace { samples, subspace } => write!(
                f,
                "Signal subspace dimension {} equals or exceeds sample size {}",
                subspace, samples
            ),
            Self::NumericalInstability => {
                write!(f, "Eigenvalue decomposition failed or produced NaNs.")
            }
        }
    }
}

impl std::error::Error for RadarError {}
