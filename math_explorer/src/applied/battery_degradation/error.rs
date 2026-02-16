use thiserror::Error;

/// Errors related to battery degradation modeling.
#[derive(Debug, Error)]
pub enum BatteryError {
    #[error("Depth of discharge must be between 0.0 and 100.0, got {0}")]
    InvalidDepthOfDischarge(f64),
    #[error("Capacity must be between 0.0 and 1.0, got {0}")]
    InvalidCapacity(f64),
    #[error("Cycles cannot be negative, got {0}")]
    InvalidCycles(f64),
}
