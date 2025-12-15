use super::constants::{C, SIGMA_T};

/// Calculates the total synchrotron power radiated by a single electron.
/// Formula: P = (4/3) * sigma_T * c * beta^2 * gamma^2 * U_B
///
/// # Arguments
/// * `u_b` - Magnetic energy density.
/// * `beta` - Electron velocity / c.
/// * `gamma` - Lorentz factor.
pub fn synchrotron_power(u_b: f64, beta: f64, gamma: f64) -> Result<f64, String> {
    if u_b < 0.0 {
        return Err("Energy density cannot be negative".to_string());
    }
    if gamma < 1.0 {
        return Err("Lorentz factor must be >= 1".to_string());
    }

    // P = 4/3 sigma_T c beta^2 gamma^2 U_B
    let power = (4.0 / 3.0) * SIGMA_T * C * beta.powi(2) * gamma.powi(2) * u_b;
    Ok(power)
}

/// Calculates the observed spectral index alpha from an electron power-law distribution p.
/// Formula: alpha = (p - 1) / 2
pub fn inverse_compton_spectral_index(p: f64) -> Result<f64, String> {
    if p <= 1.0 {
        return Err("Power law index p must be > 1 for convergence".to_string());
    }
    Ok((p - 1.0) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_radiation() {
        // Synchrotron
        // beta -> 1, gamma = 2. U_B = 1.
        // P = 4/3 sigma_T c * 1 * 4 * 1
        let u_b = 1.0;
        let gamma = 2.0;
        let beta = (1.0 - 1.0/4.0f64).sqrt(); // consistent beta
        let p = synchrotron_power(u_b, beta, gamma).unwrap();
        let expected = (4.0/3.0) * SIGMA_T * C * beta * beta * 4.0;
        assert_relative_eq!(p, expected, epsilon = 1e-6);

        // Inverse Compton
        // p = 3 => alpha = (3-1)/2 = 1.
        let alpha = inverse_compton_spectral_index(3.0).unwrap();
        assert_relative_eq!(alpha, 1.0);
    }
}
