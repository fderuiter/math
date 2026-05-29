use thiserror::Error;
use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;

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

