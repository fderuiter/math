//! Radiotherapy Specific Physics and Mechanics.

use nalgebra::{Point2, Vector2};

/// Beam Loading Line for Linear Accelerators.
///
/// $$ E = 5.925 - I_b \times 0.00808 $$
///
/// Describes the relationship between beam current and average energy in standing wave structures.
///
/// # Arguments
/// * `current_ma` - Beam current ($I_b$) in mA.
///
/// # Returns
/// * Average energy ($E$) in MeV.
pub fn beam_loading_energy(current_ma: f64) -> f64 {
    5.925 - current_ma * 0.00808
}

/// Tracking Error for Cine EPID.
///
/// $$ E_{EPID} = C_{target} - C_{field} $$
///
/// Calculates the geometric error vector between the target center and the radiation field centroid.
///
/// # Arguments
/// * `target_center` - Center of the target (tumor/fiducial) ($C_{target}$).
/// * `field_centroid` - Centroid of the radiation aperture ($C_{field}$).
///
/// # Returns
/// * A Vector2 representing the error (x, y).
pub fn cine_epid_tracking_error(
    target_center: Point2<f64>,
    field_centroid: Point2<f64>,
) -> Vector2<f64> {
    target_center - field_centroid
}

/// Translation Stage Motion Formula (Uniformly Accelerated).
///
/// $$ y_1 = a \cdot (t - T)^2 + b $$
///
/// Used for determining latency by moving a phantom on a stage.
///
/// # Arguments
/// * `t` - Current time ($t$).
/// * `half_period` - Half the movement period ($T$).
/// * `a` - Acceleration coefficient.
/// * `b` - Offset coefficient.
pub fn translation_stage_position(t: f64, half_period: f64, a: f64, b: f64) -> f64 {
    a * (t - half_period).powi(2) + b
}

/// Dirac Delta Composite Function.
///
/// $$ g(x) = \sum_{n} \delta(x - x_n) $$
///
/// Represents a train of discrete pulses (e.g., LINAC pulses).
/// Since we cannot return a distribution, this function evaluates the influence of a pulse train
/// on a given point $x$, assuming the delta functions have been convolved with a kernel $K(x)$.
///
/// Effectively returns 1.0 if $x$ is close to any pulse time $x_n$, else 0.0.
///
/// # Arguments
/// * `x` - Query point (time).
/// * `start_time` - Start of pulse train.
/// * `end_time` - End of pulse train.
/// * `interval` - Pulse interval ($\Delta t$).
/// * `tolerance` - Numerical width to consider "hit".
pub fn pulse_train_influence(
    x: f64,
    start_time: f64,
    end_time: f64,
    interval: f64,
    tolerance: f64,
) -> f64 {
    if x < start_time || x > end_time {
        return 0.0;
    }

    // Check if x is close to start_time + n * interval
    let offset = x - start_time;
    let n = (offset / interval).round();
    let closest_pulse = start_time + n * interval;

    if (x - closest_pulse).abs() < tolerance {
        1.0
    } else {
        0.0
    }
}
