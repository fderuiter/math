//! # Battery Degradation Modeling
//!
//! This module provides functions to estimate battery cycle life and capacity fade
//! for Li-ion batteries based on Depth-of-Discharge (DoD).
//!
//! ## The Model
//!
//! The degradation is modeled using a Power Law fit to experimental data:
//! $$ N_{70}(d) = \alpha \cdot d^\beta $$
//!
//! Where:
//! - $d$ is the Depth of Discharge (0-100%).
//! - $N_{70}$ is the number of equivalent full cycles until the battery reaches 70% capacity.
//! - $\alpha, \beta$ are empirical constants (Standard Li-ion: $\alpha \approx 1.019 \times 10^5, \beta \approx -1.26$).
//!
//! This implies that **shallower discharges drastically increase cycle life**.
//!
//! ## Quick Start
//!
//! ```rust
//! use math_explorer::applied::battery_degradation::{PowerLawModel, DepthOfDischarge, Cycles};
//!
//! fn main() {
//!     // 1. Initialize the standard model
//!     let model = PowerLawModel::standard();
//!
//!     // 2. Define a scenario: 80% to 20% charge window = 60% DoD
//!     let dod = DepthOfDischarge::new(60.0).unwrap();
//!
//!     // 3. Estimate Life Expectancy (Cycles to 70% SOH)
//!     let life_cycles = model.n70(dod);
//!     println!("Expected Life: {:.0} cycles", life_cycles.as_f64());
//!
//!     // 4. Predict Capacity after 1000 cycles
//!     let current_cycles = Cycles::new(1000.0).unwrap();
//!     let remaining_capacity = model.capacity(current_cycles, dod);
//!     println!("Capacity after 1000 cycles: {:.1}%", remaining_capacity.as_f64() * 100.0);
//! }
//! ```
//!
//! ## Modules
//!
//! - [`model`]: Core logic including the `PowerLawModel` struct.
//! - [`types`]: Type-safe wrappers for `Capacity`, `Cycles`, and `DepthOfDischarge`.

pub mod model;
pub mod types;

pub use model::PowerLawModel;
pub use types::{Capacity, Cycles, DepthOfDischarge};

/// Calculates the number of equivalent full cycles to 70% capacity (N₇₀).
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().n70(DepthOfDischarge::new(d).unwrap())` instead"
)]
pub fn n70(d: f64) -> f64 {
    let d_clamped = d.clamp(0.0, 100.0);
    PowerLawModel::standard()
        .n70(DepthOfDischarge::new(d_clamped).expect("Clamped value is within range"))
        .as_f64()
}

/// Calculates the remaining battery capacity after a number of cycles.
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().capacity(...)` instead"
)]
pub fn capacity(n: f64, d: f64) -> f64 {
    let d_clamped = d.clamp(0.0, 100.0);
    let n_clamped = n.max(0.0);
    PowerLawModel::standard()
        .capacity(
            Cycles::new(n_clamped).expect("Clamped value is non-negative"),
            DepthOfDischarge::new(d_clamped).expect("Clamped value is within range"),
        )
        .as_f64()
}

/// Calculates the number of equivalent full cycles to reach a target capacity.
#[deprecated(
    since = "0.2.0",
    note = "Use `PowerLawModel::standard().cycles_to_capacity(...)` instead"
)]
pub fn cycles_to_capacity(target_capacity: f64, d: f64) -> f64 {
    let d_clamped = d.clamp(0.0, 100.0);
    let target_clamped = target_capacity.clamp(0.0, 1.0);
    PowerLawModel::standard()
        .cycles_to_capacity(
            Capacity::new(target_clamped).expect("Clamped value is within range"),
            DepthOfDischarge::new(d_clamped).expect("Clamped value is within range"),
        )
        .as_f64()
}
