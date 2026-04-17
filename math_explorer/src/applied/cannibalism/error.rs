//! Error types for cannibalism modeling.

use thiserror::Error;

/// Errors related to cannibalism ODE modeling.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum CannibalismError {
    /// A required parameter is missing during builder construction.
    #[error("Missing parameter: {0}")]
    MissingParameter(String),
    /// A parameter is invalid (e.g. negative when it must be positive).
    #[error("Invalid parameter: {name} cannot be negative (got {value})")]
    InvalidParameter { name: String, value: f64 },
}
