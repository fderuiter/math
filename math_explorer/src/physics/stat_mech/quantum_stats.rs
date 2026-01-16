use super::KB;

/// Type of particle for statistical distribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParticleType {
    Boson,
    Fermion,
    Classical,
}

// --- Strategy Pattern Implementation ---

/// Strategy for calculating particle occupancy probability.
pub trait StatisticalDistribution {
    /// Calculates the occupancy probability / average occupation number.
    fn occupancy(
        &self,
        energy: f64,
        chemical_potential: f64,
        temperature: f64,
    ) -> Result<f64, String>;
}

/// Fermi-Dirac Statistics (Fermions).
#[derive(Debug, Clone, Copy, Default)]
pub struct FermiDirac;

impl StatisticalDistribution for FermiDirac {
    fn occupancy(
        &self,
        energy: f64,
        chemical_potential: f64,
        temperature: f64,
    ) -> Result<f64, String> {
        if temperature <= 0.0 {
            if energy < chemical_potential {
                return Ok(1.0);
            } else {
                return Ok(0.0);
            }
        }
        let beta = 1.0 / (KB * temperature);
        let exponent = beta * (energy - chemical_potential);
        let denom = exponent.exp() + 1.0;
        Ok(1.0 / denom)
    }
}

/// Bose-Einstein Statistics (Bosons).
#[derive(Debug, Clone, Copy, Default)]
pub struct BoseEinstein;

impl StatisticalDistribution for BoseEinstein {
    fn occupancy(
        &self,
        energy: f64,
        chemical_potential: f64,
        temperature: f64,
    ) -> Result<f64, String> {
        if temperature <= 0.0 {
            if chemical_potential > energy {
                return Err("Chemical potential cannot exceed energy for Bosons at T=0".to_string());
            }
            // If mu == E, divergence (condensate).
            return Ok(0.0); // Assume ground state condensation handled elsewhere? Or infinity.
        }
        if chemical_potential > energy {
            return Err("Chemical potential cannot be greater than energy for Bosons".to_string());
        }
        let beta = 1.0 / (KB * temperature);
        let exponent = beta * (energy - chemical_potential);
        // Check for singularity if exponent is 0 (E=mu)
        if exponent.abs() < 1e-9 {
            return Ok(f64::INFINITY);
        }
        let denom = exponent.exp() - 1.0;
        Ok(1.0 / denom)
    }
}

/// Maxwell-Boltzmann Statistics (Classical).
#[derive(Debug, Clone, Copy, Default)]
pub struct MaxwellBoltzmann;

impl StatisticalDistribution for MaxwellBoltzmann {
    fn occupancy(
        &self,
        energy: f64,
        chemical_potential: f64,
        temperature: f64,
    ) -> Result<f64, String> {
        if temperature <= 0.0 {
            // e^-inf or e^inf
            if energy < chemical_potential {
                return Ok(f64::INFINITY);
            } else {
                return Ok(0.0);
            }
        }
        let beta = 1.0 / (KB * temperature);
        let exponent = beta * (energy - chemical_potential);
        Ok((-exponent).exp())
    }
}

/// Generic function to calculate occupancy using a specific strategy.
pub fn calculate_occupancy<D: StatisticalDistribution>(
    distribution: D,
    energy: f64,
    chemical_potential: f64,
    temperature: f64,
) -> Result<f64, String> {
    distribution.occupancy(energy, chemical_potential, temperature)
}

// --- Legacy Wrapper ---

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
    match particle_type {
        ParticleType::Fermion => {
            calculate_occupancy(FermiDirac, energy, chemical_potential, temperature)
        }
        ParticleType::Boson => {
            calculate_occupancy(BoseEinstein, energy, chemical_potential, temperature)
        }
        ParticleType::Classical => {
            calculate_occupancy(MaxwellBoltzmann, energy, chemical_potential, temperature)
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

        let prob_below = occupancy_probability(ParticleType::Fermion, e_below, mu_j, t).unwrap();

        let prob_above = occupancy_probability(ParticleType::Fermion, e_above, mu_j, t).unwrap();

        assert!(
            (prob_below - 1.0).abs() < 1e-6,
            "Fermion below mu should be occupied at T~0"
        );
        assert!(
            prob_above.abs() < 1e-6,
            "Fermion above mu should be empty at T~0"
        );
    }

    #[test]
    fn test_strategy_usage() {
        // Test using the Strategy API directly
        let t = 300.0;
        let e = 1.0 * KB * t; // Energy = kT
        let mu = 0.0;

        // Fermi-Dirac: 1 / (e^1 + 1)
        let fd = FermiDirac;
        let prob_fd = calculate_occupancy(fd, e, mu, t).unwrap();
        let expected_fd = 1.0 / (std::f64::consts::E + 1.0);
        assert!((prob_fd - expected_fd).abs() < 1e-9);

        // Maxwell-Boltzmann: e^-1
        let mb = MaxwellBoltzmann;
        let prob_mb = calculate_occupancy(mb, e, mu, t).unwrap();
        let expected_mb = (-1.0_f64).exp();
        assert!((prob_mb - expected_mb).abs() < 1e-9);
    }
}
