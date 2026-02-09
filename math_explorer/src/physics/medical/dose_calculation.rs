//! Dose Calculation Algorithms.
use thiserror::Error;

/// Errors related to Dose and Fluence calculations.
#[derive(Debug, Clone, Error)]
pub enum DoseFluenceError {
    /// Calculation resulted in a singularity (e.g., division by zero).
    #[error("Singularity detected: {0}")]
    Singularity(String),
    /// Invalid parameter value (e.g., negative radius or attenuation).
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

/// Trait for Point Spread Functions (Kernels) used in Dose Calculation.
///
/// A kernel describes the radial distribution of dose deposited by secondary particles
/// originating from a primary interaction site.
pub trait DoseKernel {
    /// Evaluates the kernel at a given radial distance.
    ///
    /// # Arguments
    ///
    /// * `radius` - The radial distance from the interaction point (cm).
    ///
    /// # Returns
    ///
    /// * `Result<f64, DoseFluenceError>` - The kernel value or an error.
    fn evaluate(&self, radius: f64) -> Result<f64, DoseFluenceError>;
}

/// A simplified analytical exponential point kernel.
///
/// Formula: $K(r) = \frac{A}{r^2} e^{-\beta r}$
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExponentialKernel {
    amplitude: f64,
    beta: f64,
}

/// Tolerance for treating radius as zero (singularity check).
const SINGULARITY_TOLERANCE: f64 = 1e-6;

impl ExponentialKernel {
    /// Creates a new ExponentialKernel.
    ///
    /// # Arguments
    ///
    /// * `amplitude` - Scaling factor.
    /// * `beta` - Decay constant.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DoseFluenceError>`
    pub fn new(amplitude: f64, beta: f64) -> Result<Self, DoseFluenceError> {
        if beta <= 0.0 {
            return Err(DoseFluenceError::InvalidParameter(
                "Beta must be positive".to_string(),
            ));
        }
        Ok(Self { amplitude, beta })
    }
}

impl DoseKernel for ExponentialKernel {
    fn evaluate(&self, radius: f64) -> Result<f64, DoseFluenceError> {
        if radius.abs() < SINGULARITY_TOLERANCE {
            return Err(DoseFluenceError::Singularity(
                "Radius cannot be zero".to_string(),
            ));
        }
        if radius < 0.0 {
            return Err(DoseFluenceError::InvalidParameter(
                "Radius must be non-negative".to_string(),
            ));
        }

        let val = (self.amplitude / (radius * radius)) * (-self.beta * radius).exp();
        Ok(val)
    }
}

/// Calculates the Total Energy Released per Mass (TERMA) for a ray segment.
///
/// TERMA represents the primary energy fluence released into the medium at a point,
/// before accounting for secondary electron transport (scatter).
///
/// # Arguments
///
/// * `incident_fluence` ($\Psi_0$) - The initial radiant energy fluence.
/// * `mu` ($\mu$) - The linear attenuation coefficient of the medium (cm⁻¹).
/// * `depth` ($d$) - The radiological depth along the ray (cm).
///
/// # Returns
///
/// * `f64` - The TERMA value.
///
/// # Formula
///
/// $T = \mu \Psi_0 e^{-\mu d}$
pub fn calculate_terma(incident_fluence: f64, mu: f64, depth: f64) -> f64 {
    if incident_fluence < 0.0 || mu < 0.0 || depth < 0.0 {
        // Physical quantities should be non-negative, but we return 0.0 or handle gracefully.
        return 0.0;
    }
    mu * incident_fluence * (-mu * depth).exp()
}

/// Calculates a simplified analytical Point Spread Function (Kernel).
///
/// This kernel represents the distribution of dose deposited by secondary particles
/// scattered from a primary interaction point. It describes how TERMA is redistributed into Dose.
///
/// # Arguments
///
/// * `radius` ($r$) - Radial distance from the interaction point (cm).
/// * `amplitude` ($A$) - Scaling factor proportional to the total energy fraction.
/// * `beta` ($\beta$) - Decay constant representing the mean free path of secondary particles.
///
/// # Returns
///
/// * `Result<f64, String>` - The kernel value at radius $r$.
///
/// # Formula
///
/// $K(r) = \frac{A}{r^2} e^{-\beta r}$
///
/// *Note*: This is a singular kernel at r=0. In practice, finite voxel size integration is used.
/// Here we return an error or handle the singularity if r is too close to 0.
#[deprecated(since = "0.2.0", note = "Use ExponentialKernel struct instead")]
pub fn point_kernel(radius: f64, amplitude: f64, beta: f64) -> Result<f64, String> {
    // We delegate to the new Strategy-based implementation.
    // Note: This enforces beta > 0, which corrects the previous permissive behavior.
    let kernel = ExponentialKernel::new(amplitude, beta).map_err(|e| e.to_string())?;
    kernel.evaluate(radius).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terma_calculation() {
        // Simple case: no attenuation (mu=0) -> T = 0
        assert_eq!(calculate_terma(100.0, 0.0, 10.0), 0.0);

        // d=0 -> T = mu * Psi0
        let t0 = calculate_terma(100.0, 0.1, 0.0);
        assert!((t0 - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_point_kernel() {
        // Error on r=0
        assert!(point_kernel(0.0, 1.0, 1.0).is_err());

        // Check calculation
        let r = 2.0;
        let a = 4.0;
        let b = 0.5;
        // K = (4 / 4) * exp(-0.5 * 2) = 1 * e^-1 = 0.367879
        #[allow(deprecated)]
        let k = point_kernel(r, a, b).unwrap();
        assert!((k - (-1.0_f64).exp()).abs() < 1e-5);
    }

    #[test]
    fn test_exponential_kernel() {
        // Valid kernel
        let kernel = ExponentialKernel::new(1.0, 1.0).unwrap();

        // Evaluate at r=1
        // K = (1/1^2) * exp(-1*1) = exp(-1) = 0.367879
        let k = kernel.evaluate(1.0).unwrap();
        assert!((k - (-1.0_f64).exp()).abs() < 1e-6);

        // Invalid Beta
        assert!(matches!(
            ExponentialKernel::new(1.0, 0.0),
            Err(DoseFluenceError::InvalidParameter(_))
        ));
        assert!(matches!(
            ExponentialKernel::new(1.0, -1.0),
            Err(DoseFluenceError::InvalidParameter(_))
        ));

        // Singularity at r=0
        assert!(matches!(
            kernel.evaluate(0.0),
            Err(DoseFluenceError::Singularity(_))
        ));

        // Invalid radius
        assert!(matches!(
            kernel.evaluate(-1.0),
            Err(DoseFluenceError::InvalidParameter(_))
        ));
    }

    #[test]
    fn test_legacy_point_kernel() {
        // Should still work for valid inputs
        #[allow(deprecated)]
        let res = point_kernel(2.0, 4.0, 0.5);
        assert!(res.is_ok());

        // Should error for invalid beta now (new behavior)
        #[allow(deprecated)]
        let invalid_beta = point_kernel(2.0, 4.0, 0.0);
        assert!(invalid_beta.is_err());
    }
}

/// Calculates the average energy for the Beam Loading Line.
///
/// $$ E = 5.925 - I_b \times 0.00808 $$
///
/// # Arguments
///
/// * `beam_current` ($I_b$) - Beam current in mA.
///
/// # Returns
///
/// * `f64` - Average energy in MeV.
pub fn beam_loading_energy(beam_current: f64) -> f64 {
    5.925 - beam_current * 0.00808
}

/// Calculates Tracking Error for cine EPID.
///
/// $$ E_{EPID} = C_{target} - C_{field} $$
///
/// # Arguments
///
/// * `target_center` ($C_{target}$) - Position of the target center.
/// * `field_centroid` ($C_{field}$) - Position of the field centroid.
///
/// # Returns
///
/// * `f64` - The tracking error.
pub fn tracking_error(target_center: f64, field_centroid: f64) -> f64 {
    target_center - field_centroid
}

/// Models a Dirac Delta Composite Function for beam pulses.
///
/// $$ g(x) = \sum_{n=0}^{(t_{off} - t_{on})/\Delta t} \delta(x - x_n) $$
///
/// In practice, discrete modeling represents this as a sequence of pulses.
/// This function returns the number of pulses in the window.
///
/// # Arguments
///
/// * `t_on` ($t_{on}$) - Start time.
/// * `t_off` ($t_{off}$) - End time.
/// * `delta_t` ($\Delta t$) - Pulse interval.
///
/// # Returns
///
/// * `usize` - Number of pulses.
pub fn dirac_pulse_count(t_on: f64, t_off: f64, delta_t: f64) -> usize {
    if delta_t <= 0.0 || t_off < t_on {
        return 0;
    }
    ((t_off - t_on) / delta_t).floor() as usize + 1
}

/// Calculates the Signal-Front Delay.
///
/// $$ t_{\text{delay}} = \lim_{\omega \to \infty} (\frac{\phi(\omega)}{\omega}) $$
///
/// Represents the delay of the wavefront (signal front) in a dispersive medium.
///
/// # Arguments
///
/// * `phase_slope_high_freq` - The limit of $\phi(\omega) / \omega$ as $\omega \to \infty$.
///
/// # Returns
///
/// * `f64` - The delay time.
pub fn signal_front_delay(phase_slope_high_freq: f64) -> f64 {
    phase_slope_high_freq
}
