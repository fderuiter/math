//! Thermal Physics for Medical Applications.
//!
//! Models heat transfer in biological tissues and dielectric properties of skin.

use num_complex::Complex64;
use std::fmt;

/// Errors related to thermodynamic calculations.
#[derive(Debug, Clone)]
pub enum ThermodynamicsError {
    /// Property value is physically invalid (e.g., negative density).
    InvalidProperty(String),
}

impl fmt::Display for ThermodynamicsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProperty(msg) => write!(f, "Invalid property: {}", msg),
        }
    }
}

impl std::error::Error for ThermodynamicsError {}

/// Physical properties of biological tissue.
#[derive(Debug, Clone, Copy)]
pub struct TissueProperties {
    /// Density in kg/m^3.
    pub density: f64,
    /// Specific heat capacity in J/(kg·K).
    pub specific_heat: f64,
    /// Thermal conductivity in W/(m·K).
    pub thermal_conductivity: f64,
}

impl TissueProperties {
    /// Creates a new `TissueProperties` instance.
    ///
    /// # Errors
    /// Returns `ThermodynamicsError` if any property is non-positive.
    pub fn new(
        density: f64,
        specific_heat: f64,
        thermal_conductivity: f64,
    ) -> Result<Self, ThermodynamicsError> {
        if density <= 0.0 {
            return Err(ThermodynamicsError::InvalidProperty(format!(
                "Density must be positive, got {}",
                density
            )));
        }
        if specific_heat <= 0.0 {
            return Err(ThermodynamicsError::InvalidProperty(format!(
                "Specific heat must be positive, got {}",
                specific_heat
            )));
        }
        if thermal_conductivity < 0.0 {
            return Err(ThermodynamicsError::InvalidProperty(format!(
                "Thermal conductivity must be non-negative, got {}",
                thermal_conductivity
            )));
        }
        Ok(Self {
            density,
            specific_heat,
            thermal_conductivity,
        })
    }
}

/// Parameters for the Debye relaxation model.
#[derive(Debug, Clone, Copy)]
pub struct DebyeModel {
    /// Permittivity at high frequency (infinite frequency limit).
    pub epsilon_inf: f64,
    /// Static permittivity (low frequency limit).
    pub epsilon_s: f64,
    /// Relaxation time constant in seconds.
    pub relaxation_time: f64,
    /// Ionic conductivity in S/m.
    pub sigma_i: f64,
}

impl DebyeModel {
    /// Creates a new `DebyeModel` instance.
    pub fn new(
        epsilon_inf: f64,
        epsilon_s: f64,
        relaxation_time: f64,
        sigma_i: f64,
    ) -> Result<Self, ThermodynamicsError> {
        if epsilon_inf < 0.0 || epsilon_s < 0.0 {
            return Err(ThermodynamicsError::InvalidProperty(
                "Permittivity must be non-negative".into(),
            ));
        }
        if relaxation_time < 0.0 {
            return Err(ThermodynamicsError::InvalidProperty(
                "Relaxation time must be non-negative".into(),
            ));
        }
        if sigma_i < 0.0 {
            return Err(ThermodynamicsError::InvalidProperty(
                "Conductivity must be non-negative".into(),
            ));
        }
        Ok(Self {
            epsilon_inf,
            epsilon_s,
            relaxation_time,
            sigma_i,
        })
    }
}

/// Vacuum permittivity (F/m).
const EPSILON_0: f64 = 8.854_187_817e-12;

/// Calculates the temperature change rate using the 1-D Bio-Heat Transfer Equation.
///
/// $$ \rho c \frac{\partial T}{\partial t} = K \frac{\partial^2 T}{\partial z^2} + Q $$
///
/// # Arguments
///
/// * `tissue` - The physical properties of the tissue.
/// * `second_derivative_t` ($\frac{\partial^2 T}{\partial z^2}$) - Curvature of temperature profile (K/m^2).
/// * `heat_deposition` ($Q$) - Volumetric heat generation rate (W/m^3).
///
/// # Returns
///
/// * `f64` - The rate of temperature change (K/s).
pub fn bio_heat_transfer_rate(
    tissue: &TissueProperties,
    second_derivative_t: f64,
    heat_deposition: f64,
) -> f64 {
    let term1 = tissue.thermal_conductivity * second_derivative_t;
    let total_heat_flux = term1 + heat_deposition;
    let heat_capacity = tissue.density * tissue.specific_heat;

    // Heat capacity is guaranteed positive by TissueProperties::new
    total_heat_flux / heat_capacity
}

/// Calculates complex permittivity using the Debye Equation.
///
/// Models the dielectric relaxation of tissues (e.g., skin) at mmWave frequencies.
///
/// $$ \epsilon^* = \epsilon_{\infty} + \frac{\epsilon_s - \epsilon_{\infty}}{1 + j\omega\tau} + \frac{\sigma_i}{j\omega\epsilon_0} $$
///
/// # Arguments
///
/// * `model` - The Debye model parameters.
/// * `frequency` ($f$) - Frequency in Hz.
///
/// # Returns
///
/// * `Complex64` - The complex permittivity.
pub fn debye_permittivity(model: &DebyeModel, frequency: f64) -> Complex64 {
    let omega = 2.0 * std::f64::consts::PI * frequency;

    // Debye term: (es - einf) / (1 + j*w*t)
    let wt = omega * model.relaxation_time;
    let denom = Complex64::new(1.0, wt);
    let delta_eps = model.epsilon_s - model.epsilon_inf;
    let debye_term = Complex64::new(delta_eps, 0.0) / denom;

    // Conductivity term: sigma / (j * w * eps0)
    // = -j * sigma / (w * eps0)
    let conductivity_term = Complex64::new(0.0, -model.sigma_i / (omega * EPSILON_0));

    Complex64::new(model.epsilon_inf, 0.0) + debye_term + conductivity_term
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bio_heat() {
        let tissue = TissueProperties::new(1000.0, 4000.0, 0.5).unwrap();
        let d2t = 10.0;
        let q = 100.0;

        // dT/dt = (0.5*10 + 100) / (1000*4000) = 105 / 4e6
        let rate = bio_heat_transfer_rate(&tissue, d2t, q);
        assert!((rate - 2.625e-5).abs() < 1e-9);
    }

    #[test]
    fn test_invalid_tissue() {
        assert!(TissueProperties::new(-100.0, 4000.0, 0.5).is_err());
        assert!(TissueProperties::new(1000.0, 0.0, 0.5).is_err());
    }

    #[test]
    fn test_debye() {
        let model = DebyeModel::new(5.0, 50.0, 1e-9, 0.0).unwrap();
        let freq = 1e9; // 1 GHz

        let epsilon = debye_permittivity(&model, freq);

        assert!(epsilon.re > 5.0);
        assert!(epsilon.re < 50.0);
        assert!(epsilon.im < 0.0); // Lossy
    }
}
