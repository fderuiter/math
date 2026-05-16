use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DoseFluenceError {
    #[error("Radius cannot be zero (singularity at r=0)")]
    Singularity,
    #[error("Radius must be non-negative")]
    NegativeRadius,
    #[error("Physical quantity must be non-negative: {0}")]
    InvalidPhysicalQuantity(String),
}
