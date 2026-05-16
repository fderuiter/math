#![doc = include_str!("README.md")]

pub mod error;
pub mod model;
pub mod types;

pub use model::PowerLawModel;
pub use types::{Capacity, Cycles, DepthOfDischarge};

/// Calculates the number of equivalent full cycles to 70% capacity (N₇₀).
/// # Panics
///
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().n70(DepthOfDischarge::new_clamped(d))` instead"
)]
pub fn n70(d: f64) -> f64 {
    PowerLawModel::standard()
        .n70(DepthOfDischarge::new_clamped(d))
        .as_f64()
}

/// Calculates the remaining battery capacity after a number of cycles.
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().capacity(...)` instead"
)]
pub fn capacity(n: f64, d: f64) -> f64 {
    PowerLawModel::standard()
        .capacity(Cycles::new_clamped(n), DepthOfDischarge::new_clamped(d))
        .as_f64()
}

/// Calculates the number of equivalent full cycles to reach a target capacity.
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().cycles_to_capacity(...)` instead"
)]
pub fn cycles_to_capacity(target_capacity: f64, d: f64) -> f64 {
    PowerLawModel::standard()
        .cycles_to_capacity(
            Capacity::new_clamped(target_capacity),
            DepthOfDischarge::new_clamped(d),
        )
        .as_f64()
}

pub use error::BatteryError;
