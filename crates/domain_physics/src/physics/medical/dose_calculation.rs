//! Dose Calculation Algorithms.
//!
//! **DEPRECATED**: This module has been split into `dose`, `accelerator`, `imaging`, and `signal`.
//! Please migrate to the new modules.

#![allow(deprecated)]

pub use super::accelerator::beam_loading_energy;
pub use super::imaging::tracking_error;
pub use super::signal::{dirac_pulse_count, signal_front_delay};

use super::dose::algorithm::calculate_terma as calculate_terma_new;
use super::dose::kernel::{DoseKernel, ExponentialKernel};

/// Calculates the Total Energy Released per Mass (TERMA) for a ray segment.
#[deprecated(note = "Use 'physics::medical::dose::algorithm::calculate_terma' instead.")]
#[verified_engine::verified]
pub fn calculate_terma(incident_fluence: f64, mu: f64, depth: f64) -> f64 {
    // Legacy behavior: return 0.0 on error
    calculate_terma_new(incident_fluence, mu, depth).unwrap_or(0.0)
}

/// Calculates a simplified analytical Point Spread Function (Kernel).
#[deprecated(note = "Use 'physics::medical::dose::kernel::ExponentialKernel' instead.")]
#[verified_engine::verified]
pub fn point_kernel(radius: f64, amplitude: f64, beta: f64) -> Result<f64, String> {
    let kernel = ExponentialKernel::new(amplitude, beta);
    kernel.value_at(radius).map_err(|e| e.to_string())
}
