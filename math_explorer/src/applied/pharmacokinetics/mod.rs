//! # Pharmacokinetics
//!
//! This module provides tools for pharmacokinetic modeling, including the Bateman function,
//! superposition principles, and support for enantiomer and extended-release formulations.
//!
//! It uses a trait-based approach `PharmacokineticModel` to allow composition of different
//! drug behaviors (e.g., superposition of extended-release enantiomers).

pub mod bateman;
pub mod enantiomer;
pub mod superposition;
pub mod traits;
pub mod two_pulse;

pub use bateman::{BatemanModel, PKParameters, half_life, solve_ka, t_max};
pub use enantiomer::EnantiomerModel;
pub use superposition::SuperpositionModel;
pub use traits::PharmacokineticModel;
pub use two_pulse::TwoPulseModel;

/// Computes the concentration at time t for a single dose using the Bateman function.
///
/// This is a convenience wrapper around `BatemanModel`.
///
/// # Arguments
/// * `params` - The pharmacokinetic parameters.
/// * `t` - The time after the dose.
pub fn concentration_bateman(params: &PKParameters, t: f64) -> f64 {
    let model = BatemanModel::new(*params);
    model.concentration(t)
}

/// Computes the total concentration at time t from multiple doses using superposition.
///
/// This is a convenience wrapper around `SuperpositionModel` using `BatemanModel` as the base.
///
/// # Arguments
/// * `params` - The pharmacokinetic parameters for a single dose.
/// * `dose_times` - A slice of times at which doses were administered.
/// * `t` - The time at which to calculate the total concentration.
pub fn concentration_superposition(params: &PKParameters, dose_times: &[f64], t: f64) -> f64 {
    let base_model = BatemanModel::new(*params);
    let model = SuperpositionModel::new(base_model, dose_times.to_vec());
    model.concentration(t)
}
