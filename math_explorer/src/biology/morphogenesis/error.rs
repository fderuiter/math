use thiserror::Error;

/// Errors specific to Morphogenesis simulations.
#[derive(Debug, Clone, Error)]
pub enum MorphogenesisError {
    /// The buffer sizes do not match the expected grid dimensions.
    #[error("Buffer size mismatch: expected {expected}, got {found}")]
    BufferSizeMismatch { expected: usize, found: usize },

    /// The number of species does not match the model's configuration.
    #[error("Species count mismatch: expected {expected}, got {found}")]
    SpeciesCountMismatch { expected: usize, found: usize },

    /// The simulation has diverged (values -> Infinity or NaN).
    #[error("Simulation diverged: state contains NaN or Infinite values")]
    Divergence,

    /// An internal error from the solver strategy.
    #[error("Solver strategy error: {0}")]
    SolverError(String),
}
