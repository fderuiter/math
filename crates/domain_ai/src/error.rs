use diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum AIError {
    /// Invalid parameter value (e.g. negative learning rate).
    #[error("Invalid parameter {name}: {value}")]
    InvalidParameter { name: String, value: f64 },

    /// Optimization failed or diverged.
    #[error("Optimization diverged")]
    OptimizationDivergence,

    /// State is uninitialized.
    #[error("Uninitialized state: {name}")]
    UninitializedState { name: String },

    /// Mismatched data distribution lengths.
    #[error("Distributions have different lengths: expected {expected}, got {got}")]
    DistributionLengthMismatch { expected: usize, got: usize },

    #[error(transparent)]
    Math(#[from] math_commons::error::MathError),
}

pub type Result<T> = math_commons::error::MathResult<T>;

impl Diagnostic for AIError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        match self {
            Self::Math(e) => {
                use diagnostics::Severity as MS;
                match diagnostics::Diagnostic::severity(e) {
                    MS::Info => Severity::Info,
                    MS::Warning => Severity::Warning,
                    MS::Error => Severity::Error,
                    MS::Fatal => Severity::Fatal,
                }
            },
            _ => Severity::Error,
        }
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        match self {
            Self::Math(e) => diagnostics::Diagnostic::metadata(e),
            _ => {
                let mut map = HashMap::new();
                map.insert("error_type".to_string(), "AIError".to_string());
                map.insert("description".to_string(), self.to_string());
                map
            }
        }
    }
}
