use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum EpidemiologyError {
    /// Matrix V (Transition Matrix) is singular and cannot be inverted.
    SingularTransitionMatrix,
    /// Invalid Parameter (e.g., negative rate).
    InvalidParameter { name: String, value: f64 },
    /// Missing Parameter (e.g., required field not set in builder).
    MissingParameter { name: String },
    /// Matrix dimensions mismatch.
    DimensionMismatch {
        f_rows: usize,
        f_cols: usize,
        v_rows: usize,
        v_cols: usize,
    },
}

impl Diagnostic for EpidemiologyError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "EpidemiologyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl std::fmt::Display for EpidemiologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EpidemiologyError {}
