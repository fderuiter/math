use crate::diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HodgkinHuxleyError {
    #[error("Invalid gating variable value: {0} (must be between 0 and 1)")]
    InvalidGatingVariable(f64),
    #[error("Invalid conductance value: {0} (must be non-negative)")]
    InvalidConductance(f64),
}

impl Diagnostic for HodgkinHuxleyError {
    fn severity(&self) -> Severity {
        Severity::Error
    }
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "HodgkinHuxleyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}
