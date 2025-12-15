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

use std::f64::consts::PI;
use num_complex::Complex;
use nalgebra::Vector3;

/// 1. Second Quantization
///
/// The framework for Many-Body systems where states are defined by occupation numbers
/// rather than individual particle coordinates.
pub mod second_quantization {
    /// Particle statistics type.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ParticleType {
        /// Fermions follow Fermi-Dirac statistics and Pauli Exclusion Principle.
        /// {c_i, c_j^\dagger} = \delta_{ij}
        Fermion,
        /// Bosons follow Bose-Einstein statistics.
        /// [a_i, a_j^\dagger] = \delta_{ij}
        Boson,
    }

    /// Type of Quantum Operator.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum QuantumOperatorType {
        /// Creates a particle in a state (raising operator).
        Creation,
        /// Annihilates a particle from a state (lowering operator).
        Annihilation,
    }

    /// A quantum operator acting on a specific state index (k-vector or site).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Operator {
        pub op_type: QuantumOperatorType,
        pub index: usize,
    }

    impl Operator {
        pub fn new(op_type: QuantumOperatorType, index: usize) -> Self {
            Self { op_type, index }
        }
    }

    /// Fock State representation using occupation numbers.
    /// |n_1, n_2, ..., n_M>
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FockState {
        /// Occupation numbers for each mode/site.
        /// For Fermions, valid values are 0 or 1.
        /// For Bosons, values can be any non-negative integer (limited by u8 here).
        pub occupations: Vec<u8>,
    }

    impl FockState {
        /// Creates a new vacuum state |0, 0, ...> with given size.
        pub fn new(size: usize) -> Self {
            Self {
                occupations: vec![0; size],
            }
        }

        /// Sets the occupation number of a specific state directly.
        pub fn set_occupation(&mut self, index: usize, count: u8, p_type: ParticleType) -> Result<(), String> {
            if index >= self.occupations.len() {
                return Err(format!("Index {} out of bounds", index));
            }
            if p_type == ParticleType::Fermion && count > 1 {
                return Err("Pauli Exclusion Principle: Fermions cannot occupy same state > 1".to_string());
            }
            self.occupations[index] = count;
            Ok(())
        }

        /// Tries to add a particle to the state (Apply creation operator).
        pub fn create_particle(&mut self, index: usize, p_type: ParticleType) -> Result<(), String> {
            if index >= self.occupations.len() {
                return Err(format!("Index {} out of bounds", index));
            }
            let current = self.occupations[index];
            match p_type {
                ParticleType::Fermion => {
                    if current >= 1 {
                        return Err("Pauli Exclusion: State already occupied".to_string());
                    }
                    self.occupations[index] = 1;
                },
                ParticleType::Boson => {
                    if current == u8::MAX {
                        return Err("Boson saturation (u8 max)".to_string());
                    }
                    self.occupations[index] += 1;
                }
            }
            Ok(())
        }
    }

    /// Checks the canonical commutation (Boson) or anti-commutation (Fermion) relations.
    ///
    /// Returns the value of:
    /// * `{op1, op2}` for Fermions. Expected to be `delta_{ij}` for {c, c^\dagger}.
    /// * `[op1, op2]` for Bosons. Expected to be `delta_{ij}` for [a, a^\dagger].
    pub fn check_commutation(op1: &Operator, op2: &Operator, p_type: ParticleType) -> f64 {
        match p_type {
            ParticleType::Fermion => {
                // Fermions: Anti-commutator {A, B} = AB + BA
                // {c_i, c_j^\dagger} = delta_{ij}
                match (op1.op_type, op2.op_type) {
                    (QuantumOperatorType::Annihilation, QuantumOperatorType::Creation) |
                    (QuantumOperatorType::Creation, QuantumOperatorType::Annihilation) => {
                        if op1.index == op2.index { 1.0 } else { 0.0 }
                    }
                    _ => 0.0, // {c, c} = 0, {c^\dagger, c^\dagger} = 0
                }
            }
            ParticleType::Boson => {
                // Bosons: Commutator [A, B] = AB - BA
                // [a_i, a_j^\dagger] = delta_{ij}
                // [a_i^\dagger, a_j] = -delta_{ij}
                match (op1.op_type, op2.op_type) {
                    (QuantumOperatorType::Annihilation, QuantumOperatorType::Creation) => {
                         if op1.index == op2.index { 1.0 } else { 0.0 }
                    },
                    (QuantumOperatorType::Creation, QuantumOperatorType::Annihilation) => {
                         if op1.index == op2.index { -1.0 } else { 0.0 }
                    },
                    _ => 0.0, // [a, a] = 0, [a^\dagger, a^\dagger] = 0
                }
            }
        }
    }
}

/// 2. Electron-Electron Screening
///
/// Describes how electric fields are modified by the presence of mobile charge carriers.
pub mod screening {
    // use super::*; // Not strictly needed as thomas_fermi_dielectric uses no external deps, but good practice

    /// Calculates the Thomas-Fermi dielectric function \epsilon(q).
    ///
    /// \epsilon(q) = 1 + k_{TF}^2 / q^2
    ///
    /// This approximation is valid for static fields and small wavevectors (q -> 0)
    /// in a free electron gas.
    pub fn thomas_fermi_dielectric(q: f64, k_tf: f64) -> f64 {
        if q.abs() < 1e-10 {
            // Divergence at q=0 implies infinite screening length for constant potential (perfect shielding).
            return 1e10;
        }
        1.0 + (k_tf.powi(2) / q.powi(2))
    }

    /// Calculates the screened potential in real space (Yukawa Potential).
    ///
    /// V(r) \propto (e^{-k_{TF} r}) / r
    ///
    /// This represents the potential of a point charge screened by the electron gas.
    pub fn yukawa_potential(r: f64, k_tf: f64) -> f64 {
        if r <= 1e-12 {
            return f64::INFINITY;
        }
        (-k_tf * r).exp() / r
    }
}

/// 3. Lattice Dynamics (Phonons)
///
/// Describes the collective excitations (vibrations) of the crystal lattice.
pub mod phonons {
    use super::*;

    /// Acoustic phonon dispersion relation (Debye Approximation).
    ///
    /// \omega(k) = v_s * k
    /// Linearly dependent on wavevector k for small k.
    pub fn acoustic_dispersion(k: f64, v_s: f64) -> f64 {
        v_s * k
    }

    /// Optical phonon dispersion relation (Einstein Model).
    ///
    /// \omega(k) = \omega_E (constant)
    /// Assumes independent oscillators.
    pub fn optical_dispersion(_k: f64, w_e: f64) -> f64 {
        w_e
    }

    /// Debye Heat Capacity Cv at low temperatures.
    ///
    /// C_v \propto T^3
    /// Formula: (12 \pi^4 / 5) * N * k_B * (T / \Theta_D)^3
    pub fn debye_heat_capacity_low_temp(t: f64, theta_d: f64, n_atoms: f64, k_b: f64) -> f64 {
        let prefactor = (12.0 * PI.powi(4)) / 5.0;
        prefactor * n_atoms * k_b * (t / theta_d).powi(3)
    }
}

/// 4. Magnetism (Heisenberg Model)
///
/// Models magnetic ordering via exchange interactions between spins.
pub mod magnetism {
    use super::*;

    /// Calculates the energy of a spin configuration under the Heisenberg Hamiltonian.
    ///
    /// H = -J \sum_{<i,j>} S_i \cdot S_j
    ///
    /// * J > 0: Ferromagnetic
    /// * J < 0: Antiferromagnetic
    pub fn calculate_heisenberg_energy(j: f64, spins: &[Vector3<f64>], neighbors: &[(usize, usize)]) -> f64 {
        let mut sum_dot_products = 0.0;
        for &(idx1, idx2) in neighbors {
            if idx1 < spins.len() && idx2 < spins.len() {
                sum_dot_products += spins[idx1].dot(&spins[idx2]);
            }
        }
        -j * sum_dot_products
    }

    /// Magnon dispersion relation for a 3D ferromagnet (cubic lattice, low k).
    ///
    /// E(k) = 2 J S a^2 k^2
    ///
    /// Represents the energy cost of long-wavelength spin waves.
    pub fn magnon_dispersion(k: f64, j: f64, s: f64, a: f64) -> f64 {
        2.0 * j * s * a.powi(2) * k.powi(2)
    }
}

/// 5. Superconductivity (BCS Theory)
///
/// Describes the pairing of electrons into Cooper pairs via phonon mediation.
pub mod bcs {
    // use super::*;

    /// Solves the BCS Gap Equation iteratively.
    ///
    /// \Delta_k = - \sum_{k'} V_{kk'} \frac{\Delta_{k'}}{2 E_{k'}}
    /// where E_k = \sqrt{\xi_k^2 + \Delta_k^2}
    ///
    /// Assumes an attractive potential -V exists for energies within the Debye cutoff.
    pub fn solve_gap_equation(
        energies_xi: &[f64],
        potential_v_magnitude: f64,
        debye_energy: f64,
        iterations: usize
    ) -> Result<f64, String> {
        // Initial guess for the gap parameter Delta
        let mut delta = 0.01 * debye_energy;

        for _ in 0..iterations {
            let mut summation = 0.0;
            // Sum over all states k'
            for &xi in energies_xi {
                // Interaction acts only within the Debye window
                if xi.abs() <= debye_energy {
                    let e_k = (xi.powi(2) + delta.powi(2)).sqrt();
                    if e_k > 1e-12 {
                        summation += 1.0 / (2.0 * e_k);
                    }
                }
            }

            // New Delta from Gap Equation:
            // Delta = V * Delta * Sum(1/2E)
            // (Assuming V is attractive constant -V_0, equation becomes positive)
            let new_delta = potential_v_magnitude * delta * summation;

            // Simple mixing to stabilize convergence
            delta = 0.5 * delta + 0.5 * new_delta;
        }

        Ok(delta)
    }

    /// Calculates the Bogoliubov coherence factors (u_k, v_k).
    ///
    /// v_k^2 = 1/2 (1 - \xi_k / E_k) : Probability of pair occupation
    /// u_k^2 = 1 - v_k^2             : Probability of emptiness
    pub fn coherence_factors(xi_k: f64, delta: f64) -> (f64, f64) {
        let e_k = (xi_k.powi(2) + delta.powi(2)).sqrt();
        let v_sq = 0.5 * (1.0 - xi_k / e_k);
        // Ensure within [0, 1] for numerical safety
        let v_sq = v_sq.clamp(0.0, 1.0);
        let u_sq = 1.0 - v_sq;

        (u_sq.sqrt(), v_sq.sqrt())
    }
}

/// 6. Electron-Phonon Interaction
///
/// Describes how electrons scatter off lattice vibrations.
pub mod interactions {
    use super::*;

    /// Fröhlich Vertex for electron-phonon interaction.
    /// Describes the coupling of electrons to longitudinal optical (LO) phonons in polar crystals.
    pub struct FrohlichVertex {
        /// Fröhlich coupling constant (dimensionless).
        pub alpha: f64,
        /// LO Phonon frequency (unused in simple vertex calc but theoretically important).
        pub omega_lo: f64,
    }

    impl FrohlichVertex {
        pub fn new(alpha: f64, omega_lo: f64) -> Self {
            Self { alpha, omega_lo }
        }

        /// Returns the scattering amplitude M(q).
        /// M_q \propto 1 / q
        pub fn amplitude(&self, q: f64) -> Complex<f64> {
            if q.abs() < 1e-12 {
                return Complex::new(0.0, 0.0);
            }
            // M_q is typically purely imaginary in Fröhlich Hamiltonian formulation
            // |M_q|^2 ~ alpha / q^2
            let val = self.alpha.sqrt() / q;
            Complex::new(0.0, val)
        }
    }
}

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
