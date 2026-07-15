use diagnostics::{Diagnostic, Severity};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum EpidemiologyError {
    /// Matrix V (Transition Matrix) is singular and cannot be inverted.
    SingularTransitionMatrix,
    /// Invalid Parameter (e.g., negative rate).
    #[allow(missing_docs)]
    InvalidParameter { name: String, value: f64 },
    /// Missing Parameter (e.g., required field not set in builder).
    #[allow(missing_docs)]
    MissingParameter { name: String },
    /// Matrix dimensions mismatch.
    DimensionMismatch {
        #[allow(missing_docs)]
        f_rows: usize,
        #[allow(missing_docs)]
        f_cols: usize,
        #[allow(missing_docs)]
        v_rows: usize,
        #[allow(missing_docs)]
        v_cols: usize,
    },
}

impl Diagnostic for EpidemiologyError {
    #[verified_engine::verified]
    fn severity(&self) -> Severity {
        Severity::Error
    }
    #[verified_engine::verified]
    fn metadata(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("error_type".to_string(), "EpidemiologyError".to_string());
        map.insert("description".to_string(), self.to_string());
        map
    }
}

impl std::fmt::Display for EpidemiologyError {
    #[verified_engine::verified]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EpidemiologyError {}
