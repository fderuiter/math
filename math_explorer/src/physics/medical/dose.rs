//! Dose Calculation Algorithms.
//!
//! Models how radiation interacts with matter using convolution/superposition principles.

use super::error::MedicalPhysicsError;

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
/// * `Result<f64, MedicalPhysicsError>` - The kernel value at radius $r$.
///
/// # Formula
///
/// $K(r) = \frac{A}{r^2} e^{-\beta r}$
///
/// *Note*: This is a singular kernel at r=0.
pub fn point_kernel(radius: f64, amplitude: f64, beta: f64) -> Result<f64, MedicalPhysicsError> {
    if radius.abs() < 1e-6 {
        return Err(MedicalPhysicsError::InvalidRadius {
            radius,
            message: "Radius cannot be zero (singularity at r=0)".to_string(),
        });
    }
    if radius < 0.0 {
        return Err(MedicalPhysicsError::InvalidRadius {
            radius,
            message: "Radius must be non-negative".to_string(),
        });
    }

    let val = (amplitude / (radius * radius)) * (-beta * radius).exp();
    Ok(val)
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
        assert!(matches!(
            point_kernel(0.0, 1.0, 1.0),
            Err(MedicalPhysicsError::InvalidRadius { .. })
        ));

        // Check calculation
        let r = 2.0;
        let a = 4.0;
        let b = 0.5;
        // K = (4 / 4) * exp(-0.5 * 2) = 1 * e^-1 = 0.367879
        let k = point_kernel(r, a, b).unwrap();
        assert!((k - (-1.0_f64).exp()).abs() < 1e-5);
    }
}
