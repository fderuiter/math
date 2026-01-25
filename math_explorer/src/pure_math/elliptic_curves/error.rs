use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum EllipticError {
    TheoremConditionFailed {
        reason: String,
        i: u64,
        j: u64,
        psi_n: u64,
    },
}

impl fmt::Display for EllipticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TheoremConditionFailed {
                reason,
                i,
                j,
                psi_n,
            } => {
                write!(
                    f,
                    "{}: i + j ({} + {}) must be less than psi(N) ({})",
                    reason, i, j, psi_n
                )
            }
        }
    }
}

impl std::error::Error for EllipticError {}
