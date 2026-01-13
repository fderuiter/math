//! Thermodynamics and Bio-Heat Transfer.
//!
//! Models for heat transfer in biological tissues and electromagnetic interaction.

use num_complex::Complex;
use std::f64::consts::PI;

/// 1-D Bio-Heat Transfer Equation (Simplified Pennes Equation).
///
/// $$ \rho c \frac{\partial T}{\partial t} = K \frac{\partial^2 T}{\partial z^2} + Q $$
///
/// Returns the rate of temperature change ($\frac{\partial T}{\partial t}$).
///
/// # Arguments
/// * `density` - Mass density ($\rho$).
/// * `specific_heat` - Specific heat capacity ($c$).
/// * `conductivity` - Thermal conductivity ($K$).
/// * `heat_deposition` - Heat source term ($Q$).
/// * `laplacian_temp` - Spatial second derivative of temperature ($\frac{\partial^2 T}{\partial z^2}$).
pub fn pennes_bio_heat_rate(
    density: f64,
    specific_heat: f64,
    conductivity: f64,
    heat_deposition: f64,
    laplacian_temp: f64,
) -> f64 {
    (conductivity * laplacian_temp + heat_deposition) / (density * specific_heat)
}

/// Debye Equation for Complex Permittivity ($\epsilon^*$).
///
/// $$ \epsilon^* = \epsilon_{\infty} + \frac{\epsilon_s - \epsilon_{\infty}}{1 + j\omega\tau} + \frac{\sigma_i}{j\omega\epsilon_0} $$
///
/// Used for modeling mmWave interaction with human skin (high water content).
///
/// # Arguments
/// * `epsilon_inf` - Permittivity at infinite frequency ($\epsilon_{\infty}$).
/// * `epsilon_s` - Static permittivity ($\epsilon_s$).
/// * `tau` - Relaxation time ($\tau$).
/// * `sigma_i` - Ionic conductivity ($\sigma_i$).
/// * `omega` - Angular frequency ($\omega$).
pub fn debye_permittivity(
    epsilon_inf: f64,
    epsilon_s: f64,
    tau: f64,
    sigma_i: f64,
    omega: f64,
) -> Complex<f64> {
    let epsilon_0 = 8.854_187_817e-12;
    let j = Complex::new(0.0, 1.0);

    let term1 = epsilon_inf;
    let term2 = (epsilon_s - epsilon_inf) / (Complex::new(1.0, 0.0) + j * omega * tau);
    let term3 = sigma_i / (j * omega * epsilon_0);

    Complex::new(term1, 0.0) + term2 + term3
}

/// Dielectric Phase Delay.
///
/// $$ \Delta \phi = \frac{4\pi d}{\lambda} (\sqrt{\epsilon_r} - 1) $$
///
/// Calculates phase retardation caused by passing through a dielectric material (e.g. immobilization mask).
///
/// # Arguments
/// * `thickness` - Material thickness ($d$).
/// * `wavelength` - Operating wavelength ($\lambda$).
/// * `dielectric_constant` - Relative permittivity ($\epsilon_r$).
pub fn dielectric_phase_delay(
    thickness: f64,
    wavelength: f64,
    dielectric_constant: f64,
) -> f64 {
    (4.0 * PI * thickness / wavelength) * (dielectric_constant.sqrt() - 1.0)
}
