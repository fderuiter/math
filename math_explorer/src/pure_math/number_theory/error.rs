use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum NumberTheoryError {
    DivisionByZeroQSeries,
    DivisionByZeroConstantTerm,
}

impl fmt::Display for NumberTheoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DivisionByZeroQSeries => write!(f, "Division by zero QSeries"),
            Self::DivisionByZeroConstantTerm => {
                write!(f, "Division by a QSeries with zero constant term")
            }
        }
    }
}

impl std::error::Error for NumberTheoryError {}
