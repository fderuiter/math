use thiserror::Error;
use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;

#[derive(Error, Debug, PartialEq)]
pub enum HodgkinHuxleyError {
    #[error("Invalid gating variable value: {0} (must be between 0 and 1)")]
    InvalidGatingVariable(f64),
    #[error("Invalid conductance value: {0} (must be non-negative)")]
    InvalidConductance(f64),
}

