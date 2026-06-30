#![doc = include_str!("README.md")]

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
#[verified_engine::verified]
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
#[verified_engine::verified]
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
#[verified_engine::verified]
pub fn cycles_to_capacity(target_capacity: f64, d: f64) -> f64 {
    PowerLawModel::standard()
        .cycles_to_capacity(
            Capacity::new_clamped(target_capacity),
            DepthOfDischarge::new_clamped(d),
        )
        .as_f64()
}

// [cite:algorithmic_information_rust]

use pure_math::theory_verification;

theory_verification!(
    module = "battery_degradation",
    epsilon = 1e-6,
    constants = {
        TARGET_CAP = 0.8;
    },
    test = {
        let model = model::PowerLawModel::standard();
        assert_relative_eq!(
            model
                .capacity(
                    types::Cycles::new_clamped(0.0),
                    types::DepthOfDischarge::new_clamped(1.0)
                )
                .as_f64(),
            1.0,
            epsilon = 1e-6
        );
    }
);
