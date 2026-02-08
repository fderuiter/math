use thiserror::Error;
use crate::pure_math::analysis::roots::AnalysisError;

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
