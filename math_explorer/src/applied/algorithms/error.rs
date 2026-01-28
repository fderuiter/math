use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlgorithmError {
    SingularMatrix,
    DimensionMismatch { expected: usize, actual: usize },
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingularMatrix => write!(f, "Matrix is singular (non-invertible)"),
            Self::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
        }
    }
}

impl std::error::Error for AlgorithmError {}
