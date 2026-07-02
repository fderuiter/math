use crate::error::HighEnergyError;
use math_commons::constants::{C, SIGMA_T};

/// Calculates the total synchrotron power radiated by a single electron.
/// Formula: P = (4/3) * sigma_T * c * beta^2 * gamma^2 * U_B
///
/// # Arguments
/// * `u_b` - Magnetic energy density.
/// * `beta` - Electron velocity / c.
/// * `gamma` - Lorentz factor.
///
/// # Errors
/// * `HighEnergyError::InvalidEnergyDensity` if `u_b < 0`.
/// * `HighEnergyError::InvalidLorentzFactor` if `gamma < 1`.
#[verified_engine::verified]
pub fn synchrotron_power(u_b: f64, beta: f64, gamma: f64) -> Result<f64, HighEnergyError> {
    if u_b < 0.0 {
        return Err(HighEnergyError::InvalidEnergyDensity { u_b });
    }
    if gamma < 1.0 {
        return Err(HighEnergyError::InvalidLorentzFactor { gamma });
    }

    // P = 4/3 sigma_T c beta^2 gamma^2 U_B
    let power = (4.0 / 3.0) * SIGMA_T * C * beta.powi(2) * gamma.powi(2) * u_b;
    Ok(power)
}

/// Calculates the observed spectral index alpha from an electron power-law distribution p.
/// Formula: alpha = (p - 1) / 2
///
/// # Errors
/// * `HighEnergyError::InvalidPowerLawIndex` if `p <= 1`.
#[verified_engine::verified]
pub fn inverse_compton_spectral_index(p: f64) -> Result<f64, HighEnergyError> {
    if p <= 1.0 {
        return Err(HighEnergyError::InvalidPowerLawIndex { p });
    }
    Ok((p - 1.0) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    #[verified_engine::verified]
    fn test_radiation() {
        // Synchrotron
        // beta -> 1, gamma = 2. U_B = 1.
        // P = 4/3 sigma_T c * 1 * 4 * 1
        let u_b = 1.0;
        let gamma = 2.0;
        let beta = (1.0 - 1.0 / 4.0f64).sqrt(); // consistent beta
        let p = synchrotron_power(u_b, beta, gamma).expect("Failed to calc synchrotron power");
        let expected = (4.0 / 3.0) * SIGMA_T * C * beta * beta * 4.0;
        assert_relative_eq!(
            p,
            expected,
            epsilon = math_commons::registry::TOLERANCE_FAST
        );

        // Inverse Compton
        // p = 3 => alpha = (3-1)/2 = 1.
        let alpha = inverse_compton_spectral_index(3.0).expect("Failed to calc alpha");
        assert_relative_eq!(alpha, 1.0);
    }

    #[test]
    #[verified_engine::verified]
    fn test_errors() {
        assert!(synchrotron_power(-1.0, 0.9, 10.0).is_err());
        assert!(synchrotron_power(1.0, 0.9, 0.5).is_err());
        assert!(inverse_compton_spectral_index(0.5).is_err());
    }
}
