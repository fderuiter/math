use thiserror::Error;

/// Errors that can occur during numerical integration of ODEs.
#[derive(Error, Debug, PartialEq)]
pub enum OdeError {
    /// The solver failed to advance the system (e.g., step size too small).
    #[error("Integration failed: {0}")]
    IntegrationFailure(String),

    /// The state vector is invalid (e.g., dimension mismatch).
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// An internal error occurred in the solver.
    #[error("Solver internal error: {0}")]
    InternalError(String),
}
