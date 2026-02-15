use std::fmt;

/// Errors related to Epidemiology calculations.
#[derive(Debug, Clone, PartialEq)]
pub enum EpidemiologyError {
    /// Matrix V (Transition Matrix) is singular and cannot be inverted.
    SingularTransitionMatrix,
    /// Invalid Parameter (e.g., negative rate).
    InvalidParameter { name: String, value: f64 },
    /// Matrix dimensions mismatch.
    DimensionMismatch {
        f_rows: usize,
        f_cols: usize,
        v_rows: usize,
        v_cols: usize,
    },
}

impl fmt::Display for EpidemiologyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingularTransitionMatrix => write!(
                f,
                "Transition matrix V is singular, Next Generation Matrix cannot be computed."
            ),
            Self::InvalidParameter { name, value } => {
                write!(f, "Invalid parameter {}: {}", name, value)
            }
            Self::DimensionMismatch {
                f_rows,
                f_cols,
                v_rows,
                v_cols,
            } => write!(
                f,
                "Matrix dimensions mismatch: F=({}, {}), V=({}, {})",
                f_rows, f_cols, v_rows, v_cols
            ),
        }
    }
}

impl std::error::Error for EpidemiologyError {}
