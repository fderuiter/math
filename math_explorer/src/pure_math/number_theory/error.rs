use thiserror::Error;

/// Number Theory module errors
#[derive(Debug, Error)]
pub enum NumberTheoryError {
    #[error("Failed to parse integer from string: {0}")]
    ParseError(String),

    #[error("Modulo operation failed")]
    ModuloError,

    #[error("Conversion to usize failed")]
    ConversionError,

    #[error("Division by zero QSeries")]
    DivisionByZeroQSeries,

    #[error("Division by a QSeries with zero constant term")]
    DivisionByZeroConstantTerm,
}