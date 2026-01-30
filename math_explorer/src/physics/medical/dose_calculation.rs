//! Dose Calculation Algorithms.
//!
//! # Refactoring Note
//!
//! This module has been refactored to use the Strategy Pattern for Dose Calculation.
//! Legacy functions are preserved but deprecated.

use std::fmt;

// --- Domain Errors ---

#[derive(Debug, Clone, PartialEq)]
pub enum DoseError {
    InvalidInput(String),
    CalculationError(String),
}

impl fmt::Display for DoseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DoseError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            DoseError::CalculationError(msg) => write!(f, "Calculation Error: {}", msg),
        }
    }
}

impl std::error::Error for DoseError {}

// --- Parameter Objects ---

pub struct TermaInput {
    pub incident_fluence: f64,
    pub mu: f64,
    pub depth: f64,
}

impl TermaInput {
    pub fn new(incident_fluence: f64, mu: f64, depth: f64) -> Result<Self, DoseError> {
        if incident_fluence < 0.0 {
            return Err(DoseError::InvalidInput("Fluence must be non-negative".into()));
        }
        if mu < 0.0 {
            return Err(DoseError::InvalidInput("Attenuation coefficient (mu) must be non-negative".into()));
        }
        if depth < 0.0 {
            return Err(DoseError::InvalidInput("Depth must be non-negative".into()));
        }
        Ok(Self { incident_fluence, mu, depth })
    }
}

pub struct KernelInput {
    pub radius: f64,
}

impl KernelInput {
    pub fn new(radius: f64) -> Result<Self, DoseError> {
        if radius < 0.0 {
             return Err(DoseError::InvalidInput("Radius must be non-negative".into()));
        }
        // Singularity check
        if radius.abs() < 1e-6 {
             return Err(DoseError::InvalidInput("Radius cannot be zero (singularity at r=0)".into()));
        }
        Ok(Self { radius })
    }
}

// --- Strategies ---

pub trait TermaModel {
    fn calculate(&self, input: &TermaInput) -> Result<f64, DoseError>;
}

pub trait DoseKernel {
    fn evaluate(&self, input: &KernelInput) -> Result<f64, DoseError>;
}

// --- Implementations ---

pub struct ExponentialTerma;

impl TermaModel for ExponentialTerma {
    fn calculate(&self, input: &TermaInput) -> Result<f64, DoseError> {
        // T = mu * Psi0 * e^(-mu * d)
        Ok(input.mu * input.incident_fluence * (-input.mu * input.depth).exp())
    }
}

pub struct PointSpreadKernel {
    pub amplitude: f64,
    pub beta: f64,
}

impl PointSpreadKernel {
    pub fn new(amplitude: f64, beta: f64) -> Self {
        Self { amplitude, beta }
    }
}

impl DoseKernel for PointSpreadKernel {
    fn evaluate(&self, input: &KernelInput) -> Result<f64, DoseError> {
        let r = input.radius;
        // K = (A / r^2) * e^(-beta * r)
        let val = (self.amplitude / (r * r)) * (-self.beta * r).exp();
        Ok(val)
    }
}

// --- Legacy Functions ---

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
#[deprecated(since = "0.2.0", note = "Use TermaModel strategy (ExponentialTerma) instead")]
pub fn calculate_terma(incident_fluence: f64, mu: f64, depth: f64) -> f64 {
    let input = match TermaInput::new(incident_fluence, mu, depth) {
        Ok(i) => i,
        Err(_) => return 0.0, // Legacy behavior
    };

    let model = ExponentialTerma;
    model.calculate(&input).unwrap_or(0.0)
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
#[deprecated(since = "0.2.0", note = "Use DoseKernel strategy (PointSpreadKernel) instead")]
pub fn point_kernel(radius: f64, amplitude: f64, beta: f64) -> Result<f64, String> {
    let input = KernelInput::new(radius).map_err(|e| e.to_string())?;

    let kernel = PointSpreadKernel::new(amplitude, beta);
    kernel.evaluate(&input).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_terma_calculation() {
        // Simple case: no attenuation (mu=0) -> T = 0
        assert_eq!(calculate_terma(100.0, 0.0, 10.0), 0.0);

        // d=0 -> T = mu * Psi0
        let t0 = calculate_terma(100.0, 0.1, 0.0);
        assert!((t0 - 10.0).abs() < 1e-6);
    }

    #[test]
    #[allow(deprecated)]
    fn test_point_kernel() {
        // Error on r=0
        assert!(point_kernel(0.0, 1.0, 1.0).is_err());

        // Check calculation
        let r = 2.0;
        let a = 4.0;
        let b = 0.5;
        // K = (4 / 4) * exp(-0.5 * 2) = 1 * e^-1 = 0.367879
        let k = point_kernel(r, a, b).unwrap();
        assert!((k - (-1.0_f64).exp()).abs() < 1e-5);
    }

    // New tests for refactored code
    #[test]
    fn test_terma_input_validation() {
        assert!(TermaInput::new(-1.0, 0.1, 1.0).is_err());
        assert!(TermaInput::new(100.0, -0.1, 1.0).is_err());
        assert!(TermaInput::new(100.0, 0.1, -1.0).is_err());
    }

    #[test]
    fn test_exponential_terma_strategy() {
        let input = TermaInput::new(100.0, 0.1, 5.0).unwrap();
        let model = ExponentialTerma;
        let result = model.calculate(&input).unwrap();
        // 0.1 * 100 * exp(-0.1 * 5) = 10 * exp(-0.5)
        let expected = 10.0 * (-0.5_f64).exp();
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn test_kernel_input_validation() {
        assert!(KernelInput::new(-1.0).is_err());
        assert!(KernelInput::new(0.0).is_err()); // Singularity
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
