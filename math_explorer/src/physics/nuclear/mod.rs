//! Nuclear physics module covering properties, models, and reactions.
//!
//! This module provides implementations for:
//! - Basic nuclear properties (Radius, Density).
//! - Liquid Drop Model (Semi-Empirical Mass Formula).
//! - Shell Model (Spin-Orbit coupling).
//! - Radioactive Decay (Activity, Gamow Factor).
//! - Nuclear Reactions (Q-Value, Breit-Wigner Resonance).

pub mod constants;
pub mod types;

pub use types::*;
// Explicitly not glob-importing constants to avoid polluting namespace and potential collisions.
// Users can access them via `nuclear::constants::...`.

use std::f64::consts::PI;

/// Basic nuclear properties.
pub mod properties {
    use super::*;

    /// Calculates the nuclear radius using the formula R = R0 * A^(1/3).
    ///
    /// # Arguments
    /// * `mass_number` - Mass number A.
    ///
    /// # Returns
    /// * `f64` - The radius in femtometers (fm).
    pub fn calculate_radius(mass_number: MassNumber) -> f64 {
        constants::property_constants::R0 * mass_number.as_f64().powf(1.0 / 3.0)
    }

    /// Calculates the nucleon density (nucleons per volume).
    ///
    /// # Arguments
    /// * `mass_number` - Mass number A.
    ///
    /// # Returns
    /// * `Result<f64, NuclearError>` - The density in nucleons/fm^3.
    pub fn calculate_nucleon_density(mass_number: MassNumber) -> Result<f64, NuclearError> {
        let radius = calculate_radius(mass_number);
        let volume = (4.0 / 3.0) * PI * radius.powi(3);
        if volume == 0.0 {
            return Err(NuclearError::VolumeZero);
        }
        Ok(mass_number.as_f64() / volume)
    }
}

/// The Liquid Drop Model (Semi-Empirical Mass Formula).
pub mod liquid_drop {
    use super::*;

    /// Calculates the Binding Energy B(Z, A) using the Weizsäcker formula.
    ///
    /// Formula: B = a_v A - a_s A^(2/3) - a_c Z(Z-1)/A^(1/3) - a_sym (A-2Z)^2/A + delta
    ///
    /// # Arguments
    /// * `atomic_number` - Atomic number Z.
    /// * `mass_number` - Mass number A.
    ///
    /// # Returns
    /// * `Result<f64, NuclearError>` - The binding energy in MeV.
    pub fn binding_energy(
        atomic_number: AtomicNumber,
        mass_number: MassNumber,
    ) -> Result<f64, NuclearError> {
        let z = atomic_number.as_f64();
        let a = mass_number.as_f64();

        if atomic_number.value() > mass_number.value() {
            return Err(NuclearError::InvalidAtomicNumber(
                "Z cannot be greater than A".to_string(),
            ));
        }

        let vol_term = constants::liquid_drop_constants::A_V * a;
        let surf_term = constants::liquid_drop_constants::A_S * a.powf(2.0 / 3.0);
        let coul_term = constants::liquid_drop_constants::A_C * (z * (z - 1.0)) / a.powf(1.0 / 3.0);
        let sym_term = constants::liquid_drop_constants::A_SYM * (a - 2.0 * z).powi(2) / a;

        // Pairing term delta
        let z_val = atomic_number.value();
        let n_val = mass_number.value() - z_val;

        let delta = if z_val.is_multiple_of(2) && n_val.is_multiple_of(2) {
            // Even Z, Even N
            constants::liquid_drop_constants::DELTA_COEFF * a.powf(-0.5)
        } else if !z_val.is_multiple_of(2) && !n_val.is_multiple_of(2) {
            // Odd Z, Odd N
            -constants::liquid_drop_constants::DELTA_COEFF * a.powf(-0.5)
        } else {
            // Odd A (one even, one odd)
            0.0
        };

        let b = vol_term - surf_term - coul_term - sym_term + delta;
        Ok(b)
    }

    /// Calculates the Binding Energy per nucleon.
    pub fn binding_energy_per_nucleon(
        atomic_number: AtomicNumber,
        mass_number: MassNumber,
    ) -> Result<f64, NuclearError> {
        let be = binding_energy(atomic_number, mass_number)?;
        Ok(be / mass_number.as_f64())
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
    /// * `s` - Spin angular momentum quantum number.
    /// * `j` - Total angular momentum quantum number.
    ///
    /// # Returns
    /// * `f64` - The energy shift factor.
    pub fn spin_orbit_coupling(l: f64, s: f64, j: f64) -> f64 {
        let hbar = constants::HBAR_C / constants::LIGHT_SPEED;
        let term = j * (j + 1.0) - l * (l + 1.0) - s * (s + 1.0);
        (hbar.powi(2) / 2.0) * term
    }
}

/// Radioactive Decay.
pub mod decay {
    use super::*;

    /// Calculates the remaining amount of a substance.
    ///
    /// Formula: N(t) = N0 * e^(-lambda * t)
    ///
    /// # Arguments
    /// * `initial_quantity` - Initial quantity (N0).
    /// * `half_life` - Half-life in seconds.
    /// * `time` - Time elapsed in seconds.
    pub fn calculate_remaining(
        initial_quantity: f64,
        half_life: f64,
        time: f64,
    ) -> Result<f64, NuclearError> {
        if half_life <= 0.0 {
            return Err(NuclearError::InvalidHalfLife);
        }
        let lambda = 2.0_f64.ln() / half_life;
        Ok(initial_quantity * (-lambda * time).exp())
    }

    /// Calculates the Gamow factor for alpha decay.
    ///
    /// # Arguments
    /// * `z_daughter` - Atomic number of the daughter nucleus.
    /// * `z_alpha` - Atomic number of the alpha particle (usually 2).
    /// * `velocity` - Velocity of the alpha particle in fm/s.
    pub fn gamow_factor(
        z_daughter: AtomicNumber,
        z_alpha: AtomicNumber,
        velocity: f64,
    ) -> Result<f64, NuclearError> {
        if velocity <= 0.0 {
            return Err(NuclearError::InvalidVelocity);
        }
        let hbar = constants::HBAR_C / constants::LIGHT_SPEED;
        let numerator = PI * z_alpha.as_f64() * z_daughter.as_f64() * constants::E_SQUARED;
        let denominator = hbar * velocity;
        Ok(numerator / denominator)
    }
}

/// Nuclear Reactions.
pub mod reactions {
    use super::*;

    /// Calculates the Q-value of a reaction.
    ///
    /// Formula: Q = (sum(m_in) - sum(m_out)) * c^2
    ///
    /// # Arguments
    /// * `input_masses` - Slice of input masses in MeV/c^2.
    /// * `output_masses` - Slice of output masses in MeV/c^2.
    pub fn q_value(input_masses: &[f64], output_masses: &[f64]) -> f64 {
        let sum_in: f64 = input_masses.iter().sum();
        let sum_out: f64 = output_masses.iter().sum();
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
    pub fn breit_wigner(
        energy: f64,
        resonance_energy: f64,
        gamma_width: f64,
    ) -> Result<f64, NuclearError> {
        if gamma_width <= 0.0 {
            return Err(NuclearError::InvalidGammaWidth);
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
        // Fe-56: Z=26.
        let be_fe = liquid_drop::binding_energy_per_nucleon(
            AtomicNumber::new(26),
            MassNumber::new(56).unwrap(),
        )
        .unwrap();

        // C-12: Z=6
        let be_c = liquid_drop::binding_energy_per_nucleon(
            AtomicNumber::new(6),
            MassNumber::new(12).unwrap(),
        )
        .unwrap();

        // U-238: Z=92
        let be_u = liquid_drop::binding_energy_per_nucleon(
            AtomicNumber::new(92),
            MassNumber::new(238).unwrap(),
        )
        .unwrap();

        assert!(be_fe > be_c, "Iron should have higher BE/A than Carbon");
        assert!(be_fe > be_u, "Iron should have higher BE/A than Uranium");
    }

    #[test]
    fn test_spin_orbit() {
        let val_j3_2 = shell_model::spin_orbit_coupling(1.0, 0.5, 1.5);
        let val_j1_2 = shell_model::spin_orbit_coupling(1.0, 0.5, 0.5);

        assert!(
            val_j3_2 > val_j1_2,
            "j=3/2 should have higher coupling value than j=1/2"
        );
    }

    #[test]
    fn test_resonance() {
        let e_res = 10.0;
        let gamma = 2.0;

        let max_val = reactions::breit_wigner(e_res, e_res, gamma).unwrap();
        let off_val = reactions::breit_wigner(e_res + 1.0, e_res, gamma).unwrap();

        assert!(max_val > off_val, "Resonance should be max at E_res");
        assert!((max_val - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_constants() {
        assert!(constants::PROTON_MASS > 900.0);
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
