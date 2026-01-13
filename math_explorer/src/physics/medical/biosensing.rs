//! Biosensing and Optical Monitoring.

use std::f64::consts::PI;

/// Lambertian Intensity Profile.
///
/// $$ I_{tx}(r_i, \phi_i) = \frac{(n + 1) P_{tx}}{2\pi r_i^2} \cos^n(\phi_i) $$
///
/// Models the radiant intensity of an LED source (diffuse emitter).
///
/// # Arguments
/// * `power_tx` - Optical power ($P_{tx}$).
/// * `distance` - Distance from source ($r_i$).
/// * `angle` - Off-normal angle ($\phi_i$).
/// * `order` - Lambertian order ($n$).
pub fn lambertian_intensity(power_tx: f64, distance: f64, angle: f64, order: f64) -> f64 {
    let numerator = (order + 1.0) * power_tx;
    let denominator = 2.0 * PI * distance.powi(2);
    (numerator / denominator) * angle.cos().powf(order)
}

/// LWS Range Resolution (ADC-Limited).
///
/// $$ \Delta r_{min} = r_0 - [(r_0 - \Delta r_{max})^{-4} 2^{B_{min}} + r_0^{-4}]^{-1/4} $$
///
/// Note: The formula provided: $\Delta r_{min} = r_0 - [(r_0 - \Delta r_{max})^{-4} 2^{B_{min}} + r_0^{-4}]^{-1/4}$
/// seems to contain a typo in the user prompt or reference (likely $2^{B_{min}}$ should be related to SNR or quantization steps).
/// However, we implement it exactly as specified.
///
/// # Arguments
/// * `range` - Nominal range ($r_0$).
/// * `max_disp` - Maximum displacement range ($\Delta r_{max}$).
/// * `adc_bits` - Number of ADC bits ($B_{min}$).
pub fn lws_range_resolution(range: f64, max_disp: f64, adc_bits: u32) -> f64 {
    let term1 = (range - max_disp).powi(-4);
    let term2 = 2.0_f64.powi(adc_bits as i32);
    let term3 = range.powi(-4);

    let bracket = term1 * term2 + term3;
    // The negative power -1/4
    let result_term = bracket.powf(-0.25);

    range - result_term
}

/// Cosine Respiratory Curve Model.
///
/// $$ Z(t) = -b \cdot \cos(6\pi t / \tau + \pi / 2) $$
///
/// Simulates a breathing signal.
///
/// # Arguments
/// * `t` - Time ($t$).
/// * `amplitude` - Amplitude ($b$).
/// * `period` - Breathing period ($\tau$).
pub fn cosine_respiratory_curve(t: f64, amplitude: f64, period: f64) -> f64 {
    -amplitude * ((6.0 * PI * t / period) + (PI / 2.0)).cos()
}

/// Lock-In Phase Sensitive Detection (PSD).
///
/// $$ V_{m1} = \frac{V_s V_r}{2}\cos\{(\Delta\omega)t + (\Delta\phi)\} - \cos\{(\sum\omega)t + (\sum\phi)\} $$
///
/// Actually, the standard product-to-sum formula is:
/// $\cos(A)\cos(B) = 0.5 [\cos(A-B) + \cos(A+B)]$.
/// The user formula has a minus sign, which implies $\sin(A)\sin(B)$ or mixed.
///
/// User formula: $V_{m1} = \frac{V_s V_r}{2}\cos\{(\Delta\omega)t + (\Delta\phi)\} - \cos\{(\sum\omega)t + (\sum\phi)\}$
/// This looks like: $\cos(A-B) - \cos(A+B) = 2\sin(A)\sin(B)$.
/// So this models the mixing of two Sine waves?
///
/// We implement the formula exactly as provided.
///
/// # Arguments
/// * `amp_s`, `amp_r` - Amplitudes ($V_s, V_r$).
/// * `delta_omega`, `delta_phi` - Difference frequency/phase.
/// * `sum_omega`, `sum_phi` - Sum frequency/phase.
/// * `t` - Time.
pub fn lock_in_phase_detection(
    amp_s: f64,
    amp_r: f64,
    delta_omega: f64,
    delta_phi: f64,
    sum_omega: f64,
    sum_phi: f64,
    t: f64,
) -> f64 {
    let term1 = ((delta_omega * t) + delta_phi).cos();
    let term2 = ((sum_omega * t) + sum_phi).cos();

    (amp_s * amp_r / 2.0) * (term1 - term2)
}
