//! # Solid State Physics
//!
//! This module provides implementations for core concepts in solid state physics,
//! facilitating the transition from single-particle band theory to Many-Body Physics.
//!
//! It covers:
//! 1. Second Quantization (Operators and Fock States)
//! 2. Screening (Thomas-Fermi and Yukawa)
//! 3. Lattice Dynamics (Phonons)
//! 4. Magnetism (Heisenberg Model)
//! 5. Superconductivity (BCS)
//! 6. Interactions (Electron-Phonon)

pub mod second_quantization;
pub mod screening;
pub mod phonons;
pub mod magnetism;
pub mod bcs;
pub mod interactions;

#[cfg(test)]
mod tests {
    use super::*;
    use second_quantization::{FockState, ParticleType, QuantumOperatorType, Operator, check_commutation};
    use screening::thomas_fermi_dielectric;
    use bcs::coherence_factors;

    #[test]
    fn test_fermion_exclusion() {
        let mut state = FockState::new(2);
        // First addition should succeed
        assert!(state.create_particle(0, ParticleType::Fermion).is_ok());
        // Second addition to same state must fail
        assert!(state.create_particle(0, ParticleType::Fermion).is_err());
    }

    #[test]
    fn test_commutation_logic() {
        let c_k = Operator::new(QuantumOperatorType::Annihilation, 1);
        let c_k_dag = Operator::new(QuantumOperatorType::Creation, 1);
        let c_q = Operator::new(QuantumOperatorType::Annihilation, 2);

        // Fermion: {c_k, c_k^dag} = 1
        let val = check_commutation(&c_k, &c_k_dag, ParticleType::Fermion);
        assert!((val - 1.0).abs() < 1e-9);

        // Fermion: {c_k, c_q} = 0
        let val2 = check_commutation(&c_k, &c_q, ParticleType::Fermion);
        assert!(val2.abs() < 1e-9);

        // Boson: [a_k, a_k^dag] = 1
        let val3 = check_commutation(&c_k, &c_k_dag, ParticleType::Boson);
        assert!((val3 - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_screening_large_q() {
        let k_tf = 0.5;
        let large_q = 1000.0;
        let epsilon = thomas_fermi_dielectric(large_q, k_tf);
        // As q -> infinity, epsilon -> 1.0
        assert!((epsilon - 1.0).abs() < 1e-4);
    }

    #[test]
    fn test_bcs_probability_conservation() {
        let xi = 1.5;
        let delta = 0.2;
        let (u, v) = coherence_factors(xi, delta);
        let prob = u*u + v*v;
        assert!((prob - 1.0).abs() < 1e-9);
    }
}
