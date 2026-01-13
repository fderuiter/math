//! Radiotherapy Physics and Machinery.
//!
//! Models the physics of medical linear accelerators (Linacs) and patient interaction.

use nalgebra::{DMatrix, DVector};

/// Calculates the average energy for a Standing Wave accelerator based on the Beam Loading Line.
///
/// $$ E = 5.925 - I_b \times 0.00808 $$
///
/// # Arguments
/// * `current_ma` - Beam current in milliamperes ($I_b$).
///
/// # Returns
/// * Average energy in MeV ($E$).
pub fn beam_loading_line(current_ma: f64) -> f64 {
    5.925 - current_ma * 0.00808
}

/// Calculates the gimbal rate command using the Moore-Penrose Pseudoinverse.
///
/// $$ \dot{\gamma} = J^{\dagger} \tau_{cmd} $$
///
/// # Arguments
/// * `jacobian` - The Jacobian matrix ($J$).
/// * `torque_cmd` - The commanded torque vector ($\tau_{cmd}$).
pub fn gimbal_rate_command(jacobian: &DMatrix<f64>, torque_cmd: &DVector<f64>) -> Option<DVector<f64>> {
    // J_pseudo = (J^T J)^-1 J^T or via SVD
    // nalgebra's pseudo_inverse
    let epsilon = 1e-9;
    match jacobian.clone().pseudo_inverse(epsilon) {
        Ok(j_pinv) => Some(&j_pinv * torque_cmd),
        Err(_) => None,
    }
}

/// Calculates Tracking Error for cine EPID images.
///
/// $$ E_{EPID} = C_{target} - C_{field} $$
///
/// # Arguments
/// * `target_center` - Center of the target ($C_{target}$).
/// * `field_centroid` - Centroid of the radiation field ($C_{field}$).
pub fn tracking_error_epid(target_center: &DVector<f64>, field_centroid: &DVector<f64>) -> DVector<f64> {
    target_center - field_centroid
}

/// Models the motion of a translation stage for latency testing.
///
/// $$ y_1 = a \cdot (t - T)^2 + b $$
///
/// # Arguments
/// * `t` - Time.
/// * `big_t` - Half the movement period ($T$).
/// * `a` - Acceleration coefficient.
/// * `b` - Offset.
pub fn translation_stage_motion(t: f64, big_t: f64, a: f64, b: f64) -> f64 {
    a * (t - big_t).powi(2) + b
}

/// Calculates Signal-Front Delay in dispersive media.
///
/// $$ t_{\text{delay}} = \lim_{\omega \to \infty} (\frac{\phi(\omega)}{\omega}) $$
///
/// # Arguments
/// * `phi_omega` - Phase function evaluated at a high frequency.
/// * `omega` - High angular frequency ($\omega$).
///
/// Note: Since we cannot compute a limit numerically, this function expects `omega` to be sufficiently large.
pub fn signal_front_delay(phi_omega: f64, omega: f64) -> f64 {
    phi_omega / omega
}

/// Models a Dirac Delta Composite Function for dose convolution.
///
/// $$ g(x) = \sum_{n=0}^{(t_{off} - t_{on})/\Delta t} \delta(x - x_n) $$
///
/// In a discrete software implementation, this returns a vector of impulse locations.
///
/// # Arguments
/// * `t_on` - Beam on time.
/// * `t_off` - Beam off time.
/// * `delta_t` - Pulse interval.
/// * `position_fn` - Function mapping index to position $x_n$.
pub fn dirac_delta_composite<F>(t_on: f64, t_off: f64, delta_t: f64, position_fn: F) -> Vec<f64>
where
    F: Fn(usize) -> f64,
{
    let count = ((t_off - t_on) / delta_t).floor() as usize;
    let mut impulses = Vec::with_capacity(count + 1);
    for n in 0..=count {
        impulses.push(position_fn(n));
    }
    impulses
}

/// 1-D Bio-Heat Transfer Equation term.
///
/// $$ \rho c \frac{\partial T}{\partial t} = K \frac{\partial^2 T}{\partial z^2} + Q $$
///
/// Calculates the rate of temperature change $\frac{\partial T}{\partial t}$.
///
/// # Arguments
/// * `rho` - Mass density ($\rho$).
/// * `c` - Specific heat ($c$).
/// * `k` - Thermal conductivity ($K$).
/// * `d2t_dz2` - Second spatial derivative of temperature ($\frac{\partial^2 T}{\partial z^2}$).
/// * `q` - Heat deposition ($Q$).
pub fn bio_heat_transfer_rate(rho: f64, c: f64, k: f64, d2t_dz2: f64, q: f64) -> f64 {
    (k * d2t_dz2 + q) / (rho * c)
}

/// Debye Equation for Complex Permittivity.
///
/// $$ \epsilon^* = \epsilon_{\infty} + \frac{\epsilon_s - \epsilon_{\infty}}{1 + j\omega\tau} + \frac{\sigma_i}{j\omega\epsilon_0} $$
///
/// # Arguments
/// * `eps_inf` - Infinite frequency permittivity ($\epsilon_{\infty}$).
/// * `eps_s` - Static permittivity ($\epsilon_s$).
/// * `tau` - Relaxation time ($\tau$).
/// * `sigma_i` - Ionic conductivity ($\sigma_i$).
/// * `omega` - Angular frequency ($\omega$).
/// * `eps_0` - Vacuum permittivity.
pub fn debye_equation(
    eps_inf: f64,
    eps_s: f64,
    tau: f64,
    sigma_i: f64,
    omega: f64,
    eps_0: f64,
) -> num_complex::Complex<f64> {
    let j = num_complex::Complex::new(0.0, 1.0);
    let term1 = eps_inf;
    let term2 = (eps_s - eps_inf) / (1.0 + j * omega * tau);
    let term3 = sigma_i / (j * omega * eps_0);

    term1 + term2 + term3
}

/// Cosine Respiratory Curve Model.
///
/// $$ Z(t) = -b \cdot \cos(6\pi t / \tau + \pi / 2) $$
///
/// # Arguments
/// * `b` - Amplitude.
/// * `tau` - Period of respiratory curve.
/// * `t` - Time.
pub fn cosine_respiratory_curve(b: f64, tau: f64, t: f64) -> f64 {
    let phase = (6.0 * std::f64::consts::PI * t / tau) + std::f64::consts::FRAC_PI_2;
    -b * phase.cos()
}
