//! Battery degradation model based on depth-of-discharge (DoD).
//!
//! This module provides functions to estimate battery cycle life and capacity fade
//! for Li-ion batteries. The model is based on a power law fit to experimental data.
//!
//! # How to Use
//!
//! 1.  Determine your charge window (e.g., 20% to 80%) to find the depth-of-discharge `d`.
//!     For a window `[L, U]`, the DoD is `d = U - L`. For 20-80%, `d = 60`.
//! 2.  Use the `n70(d)` function to calculate the cycle life to 70% capacity for your DoD.
//! 3.  Use `capacity(n, d)` to plot capacity over time, or `cycles_to_capacity(target, d)`
//!     to predict when the battery will reach a specific capacity.
//!
//! All cycle counts (`n`) are in "equivalent full cycles" (EFC). For example, ten 10%
//! discharges are equivalent to one full cycle (1 EFC).
//!
//! # Model Details
//!
//! The model uses a power law `N₇₀(d) = α * d^β` fit to the following anchor data
//! for cycles to 70% capacity (N₇₀):
//!
//! - (DoD=100%, N₇₀=300)
//! - (DoD=80%, N₇₀=400)
//! - (DoD=60%, N₇₀=600)
//! - (DoD=40%, N₇₀=1000)
//! - (DoD=20%, N₇₀=2000)
//! - (DoD=10%, N₇₀=6000)
//!
//! The least squares fit resulted in `α ≈ 1.019e5` and `β ≈ -1.2639`.
//!
//! # Limits
//!
//! This is a DoD-only model. It does not account for other factors that affect battery
//! aging, such as heat, high charge/discharge rates, or calendar aging (time-based decay).
//! The results should be treated as order-of-magnitude estimates, not guarantees.

pub mod model;
pub mod types;

pub use model::PowerLawModel;
pub use types::{Capacity, Cycles, DepthOfDischarge};

/// Calculates the number of equivalent full cycles to 70% capacity (N₇₀)
/// for a given depth-of-discharge (DoD).
///
/// # Arguments
///
/// * `d` - Depth-of-discharge, as a percentage (e.g., 60.0 for 60%).
///
/// # Returns
///
/// The estimated number of cycles to reach 70% capacity.
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().n70(DepthOfDischarge::new(d))` instead"
)]
pub fn n70(d: f64) -> f64 {
    // Avoid panics for legacy users who might be passing bad values (though unlikely to work well)
    // We clamp to 0-100 to be safe
    let d_clamped = d.clamp(0.0, 100.0);
    PowerLawModel::standard()
        .n70(DepthOfDischarge::new(d_clamped))
        .as_f64()
}

/// Calculates the remaining battery capacity after a number of cycles.
///
/// # Arguments
///
/// * `n` - Number of equivalent full cycles.
/// * `d` - Depth-of-discharge, as a percentage (e.g., 60.0 for 60%).
///
/// # Returns
///
/// The battery capacity as a fraction of its initial capacity (e.g., 0.9 for 90%).
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().capacity(...)` instead"
)]
pub fn capacity(n: f64, d: f64) -> f64 {
    let d_clamped = d.clamp(0.0, 100.0);
    let n_clamped = n.max(0.0);
    PowerLawModel::standard()
        .capacity(Cycles::new(n_clamped), DepthOfDischarge::new(d_clamped))
        .as_f64()
}

/// Calculates the number of equivalent full cycles to reach a target capacity.
///
/// # Arguments
///
/// * `target_capacity` - The target capacity as a fraction (e.g., 0.9 for 90%).
/// * `d` - Depth-of-discharge, as a percentage (e.g., 60.0 for 60%).
///
/// # Returns
///
/// The estimated number of cycles to reach the target capacity.
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().cycles_to_capacity(...)` instead"
)]
pub fn cycles_to_capacity(target_capacity: f64, d: f64) -> f64 {
    let d_clamped = d.clamp(0.0, 100.0);
    let target_clamped = target_capacity.clamp(0.0, 1.0);
    PowerLawModel::standard()
        .cycles_to_capacity(
            Capacity::new(target_clamped),
            DepthOfDischarge::new(d_clamped),
        )
        .as_f64()
}
