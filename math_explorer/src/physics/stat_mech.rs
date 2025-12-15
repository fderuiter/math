//! Statistical Mechanics module.
//!
//! This module serves as the mathematical bridge between quantum/classical micro-physics
//! and macroscopic thermodynamics, covering Ensemble Theory, Quantum Statistics,
//! Phase Transitions, and Non-Equilibrium dynamics.

/// Boltzmann Constant in J/K.
pub const KB: f64 = 1.380649e-23;

/// Ensemble Theory (The Canonical Ensemble).
pub mod ensembles {
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
    pub fn boltzmann_factor(energy: f64, temperature: f64) -> f64 {
        if temperature <= 0.0 {
            // Strictly speaking, T=0 is a singularity for this formula.
            // For coding purposes, if T <= 0, we can return 0.0 or handle error.
            // However, the prompt implies simple implementation.
            // Let's assume T > 0 or handle it gracefully if possible.
            // exp(-E/0) -> if E>0 exp(-inf)=0, if E<0 exp(inf)=inf.
            // We'll proceed with standard formula, let caller handle T<=0 or math errors.
            // But to be safe against division by zero:
             if temperature == 0.0 { return 0.0; } // Fallback
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
    pub fn calculate_partition_function(energies: &[f64], temperature: f64) -> f64 {
        energies.iter().map(|&e| boltzmann_factor(e, temperature)).sum()
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
    pub fn entropy(internal_energy: f64, free_energy: f64, temperature: f64) -> f64 {
        if temperature == 0.0 {
            return 0.0;
        }
        (internal_energy - free_energy) / temperature
    }
}

/// Quantum Statistics (Distributions).
pub mod quantum_stats {
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
}

/// Phase Transitions (Ising Model).
pub mod ising {
    use super::KB;
    use rand::Rng;

    /// A 2D Spin Lattice for the Ising Model.
    pub struct SpinLattice {
        pub width: usize,
        pub height: usize,
        pub spins: Vec<i8>, // Flattened 2D grid
    }

    impl SpinLattice {
        /// Creates a new random spin lattice.
        pub fn new(width: usize, height: usize) -> Self {
            let mut rng = rand::thread_rng();
            let count = width * height;
            let spins = (0..count)
                .map(|_| if rng.gen_bool(0.5) { 1 } else { -1 })
                .collect();
            SpinLattice {
                width,
                height,
                spins,
            }
        }

        /// Gets the spin at (x, y).
        pub fn get(&self, x: usize, y: usize) -> i8 {
            self.spins[y * self.width + x]
        }

        /// Sets the spin at (x, y).
        pub fn set(&mut self, x: usize, y: usize, val: i8) {
            self.spins[y * self.width + x] = val;
        }

        /// Calculates the total energy (Hamiltonian) of the lattice.
        ///
        /// Formula: H = -J * sum_<i,j> s_i s_j - h * sum_i s_i
        /// Uses Periodic Boundary Conditions.
        pub fn hamiltonian(&self, j_coupling: f64, h_field: f64) -> f64 {
            let mut interaction_sum = 0.0;
            let mut field_sum = 0.0;

            for y in 0..self.height {
                for x in 0..self.width {
                    let s = self.get(x, y);
                    field_sum += s as f64;

                    // Neighbors (Right and Down only to avoid double counting)
                    let right_x = (x + 1) % self.width;
                    let right_s = self.get(right_x, y);

                    let down_y = (y + 1) % self.height;
                    let down_s = self.get(x, down_y);

                    interaction_sum += (s * right_s) as f64;
                    interaction_sum += (s * down_s) as f64;
                }
            }

            -j_coupling * interaction_sum - h_field * field_sum
        }

        /// Calculates the total Magnetization.
        ///
        /// Formula: M = sum_i s_i
        pub fn magnetization(&self) -> i64 {
            self.spins.iter().map(|&s| s as i64).sum()
        }

        /// Performs one Metropolis algorithm step (one attempt).
        ///
        /// 1. Select a random site.
        /// 2. Calculate energy cost to flip Delta E.
        /// 3. Accept flip if Delta E < 0 OR with probability e^(-beta * Delta E).
        pub fn metropolis_step(&mut self, temperature: f64, j_coupling: f64, h_field: f64) {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            let s = self.get(x, y);

            // Neighbors for energy difference (all 4 neighbors needed for Delta E)
            let left_x = (x + self.width - 1) % self.width;
            let right_x = (x + 1) % self.width;
            let up_y = (y + self.height - 1) % self.height;
            let down_y = (y + 1) % self.height;

            let sum_neighbors = self.get(left_x, y) as f64
                + self.get(right_x, y) as f64
                + self.get(x, up_y) as f64
                + self.get(x, down_y) as f64;

            // Delta E = E_new - E_old
            // E_old_local = -J * s * sum_neighbors - h * s
            // E_new_local = -J * (-s) * sum_neighbors - h * (-s)
            // Delta E = 2 * J * s * sum_neighbors + 2 * h * s
            let delta_e = 2.0 * s as f64 * (j_coupling * sum_neighbors + h_field);

            let should_flip = if delta_e < 0.0 {
                true
            } else {
                let beta = 1.0 / (KB * temperature);
                let prob = (-beta * delta_e).exp();
                rng.r#gen::<f64>() < prob
            };

            if should_flip {
                self.set(x, y, -s);
            }
        }
    }
}

/// Non-Equilibrium (Fluctuations).
pub mod dynamics {
    use rand::Rng;

    /// Simulates a 1D Random Walk.
    ///
    /// # Arguments
    /// * `steps` - Number of steps N.
    ///
    /// # Returns
    /// * `f64` - Final position.
    pub fn random_walk_1d(steps: usize) -> f64 {
        let mut rng = rand::thread_rng();
        let mut position = 0.0;
        for _ in 0..steps {
            if rng.gen_bool(0.5) {
                position += 1.0;
            } else {
                position -= 1.0;
            }
        }
        position
    }

    /// Estimates the Diffusion Coefficient D.
    ///
    /// Formula: D ~ <x^2> / (2t)
    ///
    /// # Arguments
    /// * `num_walks` - Number of walks M to average over.
    /// * `time_steps` - Duration t of each walk.
    ///
    /// # Returns
    /// * `f64` - Estimated Diffusion Coefficient D.
    pub fn estimate_diffusion_coefficient(num_walks: usize, time_steps: usize) -> f64 {
        let mut sum_sq_displacement = 0.0;

        for _ in 0..num_walks {
            let final_pos = random_walk_1d(time_steps);
            sum_sq_displacement += final_pos * final_pos;
        }

        let msd = sum_sq_displacement / num_walks as f64;
        // Time t corresponds to number of steps if we assume dt=1.
        msd / (2.0 * time_steps as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_consistency() {
        // System with 1 state at E=0.
        // Z = e^(-beta*0) = 1.
        // F = -k T ln(1) = 0.
        let energies = vec![0.0];
        let t = 300.0;

        let z = ensembles::calculate_partition_function(&energies, t);
        let f = ensembles::helmholtz_free_energy(z, t);

        assert!((z - 1.0).abs() < 1e-9, "Z should be 1 for single E=0 state");
        assert!(f.abs() < 1e-9, "F should be 0 for Z=1");
    }

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

        let prob_below = quantum_stats::occupancy_probability(
            quantum_stats::ParticleType::Fermion,
            e_below,
            mu_j,
            t
        ).unwrap();

        let prob_above = quantum_stats::occupancy_probability(
            quantum_stats::ParticleType::Fermion,
            e_above,
            mu_j,
            t
        ).unwrap();

        assert!((prob_below - 1.0).abs() < 1e-6, "Fermion below mu should be occupied at T~0");
        assert!(prob_above.abs() < 1e-6, "Fermion above mu should be empty at T~0");
    }

    #[test]
    fn test_ising_disordered() {
        // High temperature limit. T >> J/k_B.
        // Magnetization should be near 0 on average.
        // J = 1.0 Joules (normalized).
        // T = very high. e.g. KB*T = 1000 * J.
        // T = 1000 * J / KB.
        let j_val = 1.0; // effectively unitless if we scale T
        let h_val = 0.0;
        let t_high = 100.0 * j_val / KB; // High Temp

        let width = 20;
        let height = 20;
        let mut lattice = ising::SpinLattice::new(width, height);

        // Run many steps to equilibrate
        let steps = 10_000;
        for _ in 0..steps {
            lattice.metropolis_step(t_high, j_val, h_val);
        }

        let m = lattice.magnetization();
        let max_m = (width * height) as f64;
        let avg_m_per_spin = m as f64 / max_m;

        // In disordered phase, magnetization should be small (random fluctuations).
        // It won't be exactly 0, but should be close to 0 compared to 1.
        // <M> ~ 0.
        assert!(avg_m_per_spin.abs() < 0.3, "High T Ising should be disordered (M ~ 0). Got {}", avg_m_per_spin);
    }

    #[test]
    fn test_diffusion() {
        // For Random Walk D ~ 0.5 (since dx=1, dt=1).
        let d = dynamics::estimate_diffusion_coefficient(1000, 100);
        assert!((d - 0.5).abs() < 0.1, "Diffusion coefficient should be approx 0.5. Got {}", d);
    }
}
