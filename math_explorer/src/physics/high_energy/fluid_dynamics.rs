use super::constants::C;

/// Calculates the density compression ratio r for a strong relativistic shock.
/// Formula: r = (gamma_hat + 1) / (gamma_hat - 1)
///
/// # Arguments
/// * `adiabatic_index` - The adiabatic index (gamma_hat).
pub fn shock_compression_ratio(adiabatic_index: f64) -> Result<f64, String> {
    if adiabatic_index <= 1.0 {
        return Err("Adiabatic index must be > 1".to_string());
    }
    let r = (adiabatic_index + 1.0) / (adiabatic_index - 1.0);
    Ok(r)
}

/// Calculates the specific enthalpy h.
/// Formula: h = 1 + (gamma_hat / (gamma_hat - 1)) * (P / (rho * c^2))
pub fn specific_enthalpy(adiabatic_index: f64, pressure: f64, density: f64) -> Result<f64, String> {
    if adiabatic_index <= 1.0 {
        return Err("Adiabatic index must be > 1".to_string());
    }
    if density <= 0.0 {
        return Err("Density must be positive".to_string());
    }
    if pressure < 0.0 {
        return Err("Pressure cannot be negative".to_string());
    }

    let term = (adiabatic_index / (adiabatic_index - 1.0)) * (pressure / (density * C.powi(2)));
    Ok(1.0 + term)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_fluid_dynamics() {
        // Compression ratio, gamma_hat = 4/3.
        // r = (4/3 + 1) / (4/3 - 1) = (7/3) / (1/3) = 7.
        let r = shock_compression_ratio(4.0/3.0).unwrap();
        assert_relative_eq!(r, 7.0);
    }
}
