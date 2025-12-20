//! # Battery Degradation
//!
//! A physics-informed model for estimating the cycle life and capacity fade of Lithium-Ion batteries.
//!
//! ## 🧠 The Theory
//!
//! Batteries degrade as they are charged and discharged. The rate of degradation depends heavily on the
//! **Depth of Discharge (DoD)**—how much of the battery's capacity is used in a cycle. Shallow cycles
//! (e.g., 60% to 40%) cause significantly less wear than deep cycles (e.g., 100% to 0%).
//!
//! This module implements a **Power Law Model** ($N_{70} = \alpha \cdot d^\beta$), fitted to experimental
//! data, to predict:
//!
//! 1.  **Cycle Life ($N_{70}$)**: The number of cycles until the battery hits 70% of its original capacity.
//! 2.  **Capacity Fade**: The remaining capacity after a given number of cycles.
//!
//! ## 🚀 Quickstart
//!
//! ```rust
//! use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge, Cycles};
//!
//! fn main() {
//!     // 1. Define your usage pattern: 80% Depth of Discharge (e.g., 100% -> 20%)
//!     let dod = DepthOfDischarge::new(80.0);
//!
//!     // 2. Initialize the standard Li-ion model
//!     let model = PowerLawModel::standard();
//!
//!     // 3. Calculate life expectancy
//!     let cycles_to_70 = model.n70(dod);
//!
//!     println!("At 80% DoD, expected life is {}", cycles_to_70);
//! }
//! ```
//!
//! ## ⚠️ Constraints
//!
//! *   **DoD-Only**: This model accounts for mechanical stress due to cycling depth. It does *not* account for
//!     temperature, C-rate (charging speed), or calendar aging.
//! *   **Estimation**: Results are statistical estimates based on curve fitting, not guarantees.

pub mod model;
pub mod types;

// Re-export core components for easier access (Hub and Spoke)
pub use model::*;
pub use types::*;

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
#[deprecated(since = "0.2.0", note = "Use `PowerLawModel::standard().n70(DepthOfDischarge::new(d))` instead")]
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
#[deprecated(since = "0.2.0", note = "Use `PowerLawModel::standard().capacity(...)` instead")]
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
#[deprecated(since = "0.2.0", note = "Use `PowerLawModel::standard().cycles_to_capacity(...)` instead")]
pub fn cycles_to_capacity(target_capacity: f64, d: f64) -> f64 {
    let d_clamped = d.clamp(0.0, 100.0);
    let target_clamped = target_capacity.clamp(0.0, 1.0);
    PowerLawModel::standard()
        .cycles_to_capacity(Capacity::new(target_clamped), DepthOfDischarge::new(d_clamped))
        .as_f64()
}
