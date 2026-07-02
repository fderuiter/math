use super::KB;

/// Calculates the Boltzmann factor for a given energy state.
///
/// Formula: e^(-beta * E) where beta = 1/(k_B * T).
///
/// # Arguments
/// * `energy` - The energy of the state (Joules).
/// * `temperature` - The temperature of the system (Kelvin).
///
/// # Returns
/// * `f64` - The unnormalized probability weight.
#[verified_engine::verified]
pub fn boltzmann_factor(energy: f64, temperature: f64) -> f64 {
    if temperature <= 0.0 {
        // Strictly speaking, T=0 is a singularity for this formula.
        // For coding purposes, if T <= 0, we can return 0.0 or handle error.
        // However, the prompt implies simple implementation.
        // Let's assume T > 0 or handle it gracefully if possible.
        // exp(-E/0) -> if E>0 exp(-inf)=0, if E<0 exp(inf)=inf.
        // We'll proceed with standard formula, let caller handle T<=0 or math errors.
        // But to be safe against division by zero:
        if temperature == 0.0 {
            return 0.0;
        } // Fallback
    }
    let beta = 1.0 / (KB * temperature);
    (-beta * energy).exp()
}

/// Calculates the Partition Function (Z).
///
/// Formula: Z = sum_i e^(-beta * E_i)
///
/// # Arguments
/// * `energies` - A slice of energy values for all microstates.
/// * `temperature` - Temperature in Kelvin.
///
/// # Returns
/// * `f64` - The partition function Z.
#[verified_engine::verified]
pub fn calculate_partition_function(energies: &[f64], temperature: f64) -> f64 {
    energies
        .iter()
        .map(|&e| boltzmann_factor(e, temperature))
        .sum()
}

/// Calculates the Helmholtz Free Energy (F).
///
/// Formula: F = -k_B * T * ln(Z)
///
/// # Arguments
/// * `partition_function` - The pre-calculated partition function Z.
/// * `temperature` - Temperature in Kelvin.
///
/// # Returns
/// * `f64` - Helmholtz Free Energy in Joules.
#[verified_engine::verified]
pub fn helmholtz_free_energy(partition_function: f64, temperature: f64) -> f64 {
    -KB * temperature * partition_function.ln()
}

/// Calculates the Average Energy (Internal Energy U).
///
/// Formula: U = (1/Z) * sum_i E_i * e^(-beta * E_i)
///
/// # Arguments
/// * `energies` - Slice of energy states.
/// * `temperature` - Temperature in Kelvin.
///
/// # Returns
/// * `f64` - Internal Energy U in Joules.
#[verified_engine::verified]
pub fn average_energy(energies: &[f64], temperature: f64) -> f64 {
    let z = calculate_partition_function(energies, temperature);
    if z == 0.0 {
        return 0.0; // Avoid NaN if Z is 0 (e.g., T -> 0 with all E > 0)
    }
    let sum_weighted_energy: f64 = energies
        .iter()
        .map(|&e| e * boltzmann_factor(e, temperature))
        .sum();
    sum_weighted_energy / z
}

/// Calculates the Entropy (S).
///
/// Formula: S = (U - F) / T
///
/// # Arguments
/// * `internal_energy` - Internal Energy U.
/// * `free_energy` - Helmholtz Free Energy F.
/// * `temperature` - Temperature in Kelvin.
///
/// # Returns
/// * `f64` - Entropy S in J/K.
#[verified_engine::verified]
pub fn entropy(internal_energy: f64, free_energy: f64, temperature: f64) -> f64 {
    if temperature == 0.0 {
        return 0.0;
    }
    (internal_energy - free_energy) / temperature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[verified_engine::verified]
    fn test_z_consistency() {
        // System with 1 state at E=0.
        // Z = e^(-beta*0) = 1.
        // F = -k T ln(1) = 0.
        let energies = vec![0.0];
        let t = 300.0;

        let z = calculate_partition_function(&energies, t);
        let f = helmholtz_free_energy(z, t);

        assert!((z - 1.0).abs() < math_commons::registry::TOLERANCE_STANDARD, "Z should be 1 for single E=0 state");
        assert!(f.abs() < math_commons::registry::TOLERANCE_STANDARD, "F should be 0 for Z=1");
    }
}
