// Battery degradation model based on depth-of-discharge (DoD).
// This module provides functions to estimate battery cycle life and capacity fade.
// The model is based on a power law fit to experimental data for Li-ion batteries.
//
// For details on the model, see the project's README.md or the original request.
//
// Key functions:
// - n70(d): Calculates the number of equivalent full cycles to 70% capacity for a given DoD.
// - capacity(n, d): Calculates the remaining capacity after n equivalent full cycles at a given DoD.
// - cycles_to_capacity(target_capacity, d): Calculates the number of cycles to reach a specific capacity level.

const ALPHA: f64 = 1.019e5;
const BETA: f64 = -1.2639;
const LN_0_7: f64 = -0.3566749439387324; // ln(0.7)

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
pub fn n70(d: f64) -> f64 {
    ALPHA * d.powf(BETA)
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
pub fn capacity(n: f64, d: f64) -> f64 {
    let n70_val = n70(d);
    0.7_f64.powf(n / n70_val)
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
pub fn cycles_to_capacity(target_capacity: f64, d: f64) -> f64 {
    let n70_val = n70(d);
    (target_capacity.ln() / LN_0_7) * n70_val
}
