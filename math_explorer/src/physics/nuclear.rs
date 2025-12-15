//! Nuclear physics module covering properties, models, and reactions.
//!
//! This module provides implementations for:
//! - Basic nuclear properties (Radius, Density).
//! - Liquid Drop Model (Semi-Empirical Mass Formula).
//! - Shell Model (Spin-Orbit coupling).
//! - Radioactive Decay (Activity, Gamow Factor).
//! - Nuclear Reactions (Q-Value, Breit-Wigner Resonance).

// Constants
/// Proton mass in MeV/c^2.
pub const PROTON_MASS: f64 = 938.272;
/// Neutron mass in MeV/c^2.
pub const NEUTRON_MASS: f64 = 939.565;
/// Reduced Planck constant times speed of light (hbar * c) in MeV fm.
pub const HBAR_C: f64 = 197.3;
/// Speed of light in fm/s (approx 3.0e23).
pub const LIGHT_SPEED: f64 = 2.99792458e23;
/// Squared elementary charge (e^2) in MeV fm.
/// Derived from fine-structure constant alpha = e^2 / (hbar c) ~ 1/137.036.
pub const E_SQUARED: f64 = 1.439976;

/// Basic nuclear properties.
pub mod properties {
    use std::f64::consts::PI;

    /// Calculates the nuclear radius using the formula R = R0 * A^(1/3).
    ///
    /// # Arguments
    /// * `mass_number` - Mass number A (number of nucleons).
    ///
    /// # Returns
    /// * `Result<f64, String>` - The radius in femtometers (fm), or an error if A is negative.
    pub fn calculate_radius(mass_number: f64) -> Result<f64, String> {
        if mass_number < 0.0 {
            return Err("Mass number A cannot be negative".to_string());
        }
        let r0 = 1.2; // fm
        Ok(r0 * mass_number.powf(1.0 / 3.0))
    }

    /// Calculates the nucleon density (nucleons per volume).
    ///
    /// The density is approximately constant for all nuclei.
    ///
    /// # Arguments
    /// * `mass_number` - Mass number A.
    ///
    /// # Returns
    /// * `Result<f64, String>` - The density in nucleons/fm^3.
    pub fn calculate_nucleon_density(mass_number: f64) -> Result<f64, String> {
        if mass_number <= 0.0 {
            return Err("Mass number A must be positive".to_string());
        }
        let radius = calculate_radius(mass_number)?;
        let volume = (4.0 / 3.0) * PI * radius.powi(3);
        if volume == 0.0 {
             return Err("Volume is zero".to_string());
        }
        Ok(mass_number / volume)
    }
}

/// The Liquid Drop Model (Semi-Empirical Mass Formula).
pub mod liquid_drop {
    /// Calculates the Binding Energy B(Z, A) using the Weizsäcker formula.
    ///
    /// Formula: B = a_v A - a_s A^(2/3) - a_c Z(Z-1)/A^(1/3) - a_sym (A-2Z)^2/A + delta
    ///
    /// # Arguments
    /// * `atomic_number` - Atomic number Z (number of protons).
    /// * `mass_number` - Mass number A (total nucleons).
    ///
    /// # Returns
    /// * `Result<f64, String>` - The binding energy in MeV.
    pub fn binding_energy(atomic_number: f64, mass_number: f64) -> Result<f64, String> {
        if mass_number <= 0.0 {
            return Err("Mass number A must be positive".to_string());
        }
        if atomic_number < 0.0 {
            return Err("Atomic number Z cannot be negative".to_string());
        }
        if atomic_number > mass_number {
            return Err("Atomic number Z cannot be greater than mass number A".to_string());
        }

        let a_v = 15.75;
        let a_s = 17.8;
        let a_c = 0.711;
        let a_sym = 23.7;

        let vol_term = a_v * mass_number;
        let surf_term = a_s * mass_number.powf(2.0 / 3.0);
        let coul_term = a_c * (atomic_number * (atomic_number - 1.0)) / mass_number.powf(1.0 / 3.0);
        let sym_term = a_sym * (mass_number - 2.0 * atomic_number).powi(2) / mass_number;

        // Pairing term delta
        let neutron_number = mass_number - atomic_number;
        // Determine parity using integer casting.
        // We use a small epsilon for float comparison safety if inputs are not perfect integers,
        // but assuming inputs are effectively integers.
        let z_int = atomic_number.round() as i64;
        let n_int = neutron_number.round() as i64;

        let delta = if z_int % 2 == 0 && n_int % 2 == 0 {
            // Even Z, Even N
            11.18 * mass_number.powf(-0.5)
        } else if z_int % 2 != 0 && n_int % 2 != 0 {
            // Odd Z, Odd N
            -11.18 * mass_number.powf(-0.5)
        } else {
            // Odd A (one even, one odd)
            0.0
        };

        let b = vol_term - surf_term - coul_term - sym_term + delta;
        Ok(b)
    }

    /// Calculates the Binding Energy per nucleon.
    pub fn binding_energy_per_nucleon(atomic_number: f64, mass_number: f64) -> Result<f64, String> {
        if mass_number <= 0.0 {
            return Err("Mass number A must be positive".to_string());
        }
        let be = binding_energy(atomic_number, mass_number)?;
        Ok(be / mass_number)
    }
}

/// The Shell Model (Spin-Orbit coupling).
pub mod shell_model {
    use super::*;

    /// Calculates the spin-orbit expectation value <L.S>.
    ///
    /// Formula: <L.S> = (hbar^2 / 2) * (j(j+1) - l(l+1) - s(s+1))
    ///
    /// # Arguments
    /// * `l` - Orbital angular momentum quantum number.
    /// * `s` - Spin angular momentum quantum number (usually 0.5).
    /// * `j` - Total angular momentum quantum number.
    ///
    /// # Returns
    /// * `f64` - The energy shift factor (in units of hbar^2).
    pub fn spin_orbit_coupling(l: f64, s: f64, j: f64) -> f64 {
        // Calculate hbar from constants.
        // HBAR_C = hbar * c (MeV fm)
        // c = LIGHT_SPEED (fm/s)
        // hbar = HBAR_C / LIGHT_SPEED (MeV s)
        let hbar = HBAR_C / LIGHT_SPEED;
        let term = j * (j + 1.0) - l * (l + 1.0) - s * (s + 1.0);
        (hbar.powi(2) / 2.0) * term
    }
}

/// Radioactive Decay.
pub mod decay {
    use super::*;
    use std::f64::consts::PI;

    /// Calculates the remaining amount of a substance.
    ///
    /// Formula: N(t) = N0 * e^(-lambda * t)
    /// where lambda = ln(2) / half_life
    ///
    /// # Arguments
    /// * `initial_quantity` - Initial quantity (N0).
    /// * `half_life` - Half-life in seconds.
    /// * `time` - Time elapsed in seconds.
    pub fn calculate_remaining(initial_quantity: f64, half_life: f64, time: f64) -> Result<f64, String> {
        if half_life <= 0.0 {
            return Err("Half-life must be positive".to_string());
        }
        let lambda = 2.0_f64.ln() / half_life;
        Ok(initial_quantity * (-lambda * time).exp())
    }

    /// Calculates the Gamow factor for alpha decay.
    ///
    /// Formula: G = (pi * Z_alpha * Z_d * e^2) / (hbar * v)
    ///
    /// # Arguments
    /// * `z_daughter` - Atomic number of the daughter nucleus.
    /// * `z_alpha` - Atomic number of the alpha particle (usually 2).
    /// * `velocity` - Velocity of the alpha particle in fm/s.
    pub fn gamow_factor(z_daughter: f64, z_alpha: f64, velocity: f64) -> Result<f64, String> {
        if velocity <= 0.0 {
            return Err("Velocity must be positive".to_string());
        }
        // hbar = HBAR_C / c_fm_s
        let hbar = HBAR_C / LIGHT_SPEED;
        let numerator = PI * z_alpha * z_daughter * E_SQUARED;
        let denominator = hbar * velocity;
        Ok(numerator / denominator)
    }
}

/// Nuclear Reactions.
pub mod reactions {
    // No imports from super needed for calculation, just logic. But maybe constants? No.

    /// Calculates the Q-value of a reaction.
    ///
    /// Formula: Q = (sum(m_in) - sum(m_out)) * c^2
    ///
    /// Note: If masses are provided in MeV/c^2 (energy units), the c^2 factor is implicit (i.e. 1).
    /// If masses are in kg, c^2 is needed.
    /// Based on module constants (PROTON_MASS ~ 938), inputs are expected in MeV/c^2.
    /// Thus, we return the mass difference directly as energy.
    ///
    /// # Arguments
    /// * `input_masses` - Slice of input masses in MeV/c^2.
    /// * `output_masses` - Slice of output masses in MeV/c^2.
    pub fn q_value(input_masses: &[f64], output_masses: &[f64]) -> f64 {
        let sum_in: f64 = input_masses.iter().sum();
        let sum_out: f64 = output_masses.iter().sum();
        // Since units are MeV/c^2, multiplying by c^2 gives MeV.
        // Effectively (m_in - m_out).
        sum_in - sum_out
    }

    /// Calculates the Breit-Wigner cross-section shape.
    ///
    /// Formula: sigma(E) ~ Gamma^2 / ((E - E_res)^2 + Gamma^2/4)
    ///
    /// # Arguments
    /// * `energy` - Energy E in MeV.
    /// * `resonance_energy` - Resonance energy E_res in MeV.
    /// * `gamma_width` - Decay width Gamma in MeV.
    pub fn breit_wigner(energy: f64, resonance_energy: f64, gamma_width: f64) -> Result<f64, String> {
        if gamma_width <= 0.0 {
            return Err("Gamma width must be positive".to_string());
        }
        let numerator = gamma_width.powi(2);
        let denominator = (energy - resonance_energy).powi(2) + (gamma_width.powi(2) / 4.0);
        Ok(numerator / denominator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iron_peak() {
        // Verify Binding Energy per nucleon peaks around A=56.
        // We'll check A=56 (Fe-56) vs A=10 and A=200.
        // Fe-56: Z=26.
        let be_fe = liquid_drop::binding_energy_per_nucleon(26.0, 56.0).unwrap();

        // C-12: Z=6
        let be_c = liquid_drop::binding_energy_per_nucleon(6.0, 12.0).unwrap();

        // U-238: Z=92
        let be_u = liquid_drop::binding_energy_per_nucleon(92.0, 238.0).unwrap();

        assert!(be_fe > be_c, "Iron should have higher BE/A than Carbon");
        assert!(be_fe > be_u, "Iron should have higher BE/A than Uranium");
    }

    #[test]
    fn test_spin_orbit() {
        // Verify j=3/2 has higher 'energy' (coupling value) than j=1/2 for l=1, s=1/2.
        // For l=1, s=1/2:
        // j=3/2: <L.S> > 0
        // j=1/2: <L.S> < 0
        let val_j3_2 = shell_model::spin_orbit_coupling(1.0, 0.5, 1.5);
        let val_j1_2 = shell_model::spin_orbit_coupling(1.0, 0.5, 0.5);

        assert!(val_j3_2 > val_j1_2, "j=3/2 should have higher coupling value than j=1/2");
    }

    #[test]
    fn test_resonance() {
        // Verify Breit-Wigner is max at E = E_res.
        let e_res = 10.0;
        let gamma = 2.0;

        let max_val = reactions::breit_wigner(e_res, e_res, gamma).unwrap();
        let off_val = reactions::breit_wigner(e_res + 1.0, e_res, gamma).unwrap();

        assert!(max_val > off_val, "Resonance should be max at E_res");
        // Check actual value at peak: Gamma^2 / (0 + Gamma^2/4) = 4.
        assert!((max_val - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_constants() {
        // Check proton mass
        assert!(PROTON_MASS > 900.0);
    }

    #[test]
    fn test_decay() {
        let half_life = 10.0;
        let n0 = 100.0;
        let remaining = decay::calculate_remaining(n0, half_life, 10.0).unwrap();
        // At t=half_life, should be 50.
        assert!((remaining - 50.0).abs() < 1e-6);
    }
}
