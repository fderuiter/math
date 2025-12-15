use super::KB;

/// Type of particle for statistical distribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Boson,
    Fermion,
    Classical,
}

/// Calculates the occupancy probability / average occupation number.
///
/// Formulas:
/// - Maxwell-Boltzmann: <n> = e^(-beta(epsilon - mu))
/// - Fermi-Dirac: <n> = 1 / (e^(beta(epsilon - mu)) + 1)
/// - Bose-Einstein: <n> = 1 / (e^(beta(epsilon - mu)) - 1)
///
/// # Arguments
/// * `particle_type` - The type of particle (Boson, Fermion, Classical).
/// * `energy` - Energy level epsilon.
/// * `chemical_potential` - Chemical potential mu.
/// * `temperature` - Temperature T in Kelvin.
///
/// # Returns
/// * `Result<f64, String>` - The average occupancy, or error if invalid (e.g. Bosons with mu > epsilon).
pub fn occupancy_probability(
    particle_type: ParticleType,
    energy: f64,
    chemical_potential: f64,
    temperature: f64,
) -> Result<f64, String> {
    if temperature <= 0.0 {
        // T -> 0 limits.
        // For Fermions: if E < mu, 1.0, else 0.0.
        // For Bosons: if E < mu, undefined/negative? E > mu required.
        // Let's approximate small T by using a very small epsilon or handling directly?
        // The prompt asks for verification of Fermion limit T->0.
        // If T is exactly 0, we can't divide by 0 in beta.
        // We'll treat 0 as "very small positive" or handle explicitly.
        // Let's handle explicitly.
         match particle_type {
            ParticleType::Fermion => {
                if energy < chemical_potential {
                    return Ok(1.0);
                } else {
                    return Ok(0.0);
                }
            }
            ParticleType::Boson => {
                 if chemical_potential > energy {
                     return Err("Chemical potential cannot exceed energy for Bosons at T=0".to_string());
                 }
                 // If mu == E, divergence (condensate).
                 return Ok(0.0); // Assume ground state condensation handled elsewhere? Or infinity.
            }
            ParticleType::Classical => {
                 // e^-inf or e^inf
                 if energy < chemical_potential {
                     return Ok(f64::INFINITY);
                 } else {
                     return Ok(0.0);
                 }
            }
        }
    }

    let beta = 1.0 / (KB * temperature);
    let exponent = beta * (energy - chemical_potential);

    match particle_type {
        ParticleType::Classical => {
            // Maxwell-Boltzmann
            // Note: The formula provided in prompt is e^(-beta(epsilon - mu)).
            // Usually MB is e^(-beta(E - mu)) or just A * e^(-beta E).
            // We use the prompt's formula.
            Ok((-exponent).exp())
        }
        ParticleType::Fermion => {
            // Fermi-Dirac
            let denom = exponent.exp() + 1.0;
            Ok(1.0 / denom)
        }
        ParticleType::Boson => {
            // Bose-Einstein
            if chemical_potential > energy {
                 return Err("Chemical potential cannot be greater than energy for Bosons".to_string());
            }
            // Check for singularity if exponent is 0 (E=mu)
            if exponent.abs() < 1e-9 {
                return Ok(f64::INFINITY);
            }
            let denom = exponent.exp() - 1.0;
            // Since E >= mu, exponent >= 0. exponent.exp() >= 1.
            // denom >= 0.
            Ok(1.0 / denom)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fermion_limit() {
        // T -> 0 (use very small T)
        // E < mu -> prob 1
        // E > mu -> prob 0
        let t = 1e-9; // effectively 0
        // Actually, formulas use raw Joules.
        // Let's use E in Joules.
        let mu_j = 1.0e-20;
        let e_below = 0.5e-20;
        let e_above = 1.5e-20;

        let prob_below = occupancy_probability(
            ParticleType::Fermion,
            e_below,
            mu_j,
            t
        ).unwrap();

        let prob_above = occupancy_probability(
            ParticleType::Fermion,
            e_above,
            mu_j,
            t
        ).unwrap();

        assert!((prob_below - 1.0).abs() < 1e-6, "Fermion below mu should be occupied at T~0");
        assert!(prob_above.abs() < 1e-6, "Fermion above mu should be empty at T~0");
    }
}
