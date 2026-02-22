//! # Schrödinger Equation
//!
//! This module provides tools for solving the time-dependent Schrödinger equation,
//! which describes how the quantum state of a physical system changes over time.
//!
//! $$ i\hbar \frac{\partial}{\partial t}|\psi(t)\rangle = \hat{H}|\psi(t)\rangle $$
//!
//! For a time-independent Hamiltonian $\hat{H}$, the solution is given by the unitary time-evolution operator:
//!
//! $$ |\psi(t)\rangle = e^{-i\hat{H}t/\hbar} |\psi(0)\rangle $$
//!
//! ## Usage
//!
//! The core function `evolve_state` computes the state at time $t$ given an initial state and a Hamiltonian.

use crate::physics::quantum::types::{QuantumOperator, QuantumState};
use num_complex::Complex;

/// Calculates the time evolution operator $U(t) = e^{-i\hat{H}t/\hbar}$.
///
/// This implementation assumes the Hamiltonian $\hat{H}$ is time-independent.
/// The operator is computed using matrix exponentiation.
///
/// $$ U(t) = \exp\left(-\frac{i}{\hbar} \hat{H} t\right) $$
///
/// # Arguments
/// * `hamiltonian` - The Hamiltonian operator $\hat{H}$ (must be a square matrix).
/// * `t` - The time duration $t$ to evolve the system.
/// * `h_bar` - The reduced Planck constant $\hbar$.
///
/// # Returns
/// The unitary operator $U(t)$ as a `QuantumOperator`.
pub fn time_evolution_operator(
    hamiltonian: &QuantumOperator,
    t: f64,
    h_bar: f64,
) -> QuantumOperator {
    // U(t) = exp(-i * H * t / h_bar)
    let i = Complex::new(0.0, 1.0);
    // Avoid division by zero if h_bar is 0 (though physical h_bar is non-zero)
    let factor = -i * t / h_bar;

    // Scale the Hamiltonian matrix by -it/hbar
    let scaled_h = &hamiltonian.matrix * factor;

    // Compute the matrix exponential: e^A
    // nalgebra's exp() is for square matrices.
    let u_matrix = scaled_h.exp();

    QuantumOperator::new(u_matrix)
}

/// Evolves a quantum state over time using the Schrödinger equation.
///
/// Computes the new state vector $|\psi(t)\rangle$ by applying the time evolution operator to the initial state $|\psi(0)\rangle$.
///
/// $$ |\psi(t)\rangle = e^{-i\hat{H}t/\hbar} |\psi(0)\rangle $$
///
/// # Arguments
/// * `state` - The initial quantum state $|\psi(0)\rangle$.
/// * `hamiltonian` - The time-independent Hamiltonian operator $\hat{H}$.
/// * `t` - The time duration $t$.
/// * `h_bar` - The reduced Planck constant $\hbar$.
///
/// # Returns
/// The evolved quantum state $|\psi(t)\rangle$.
///
/// # Example
///
/// Simulating a 2-level system (Qubit) undergoing Rabi oscillations.
///
/// ```
/// use math_explorer::physics::quantum::schrodinger::evolve_state;
/// use math_explorer::physics::quantum::types::{QuantumState, QuantumOperator};
/// use nalgebra::{DMatrix, DVector};
/// use num_complex::Complex;
///
/// // 1. Define the Hamiltonian for a spin in a magnetic field (Pauli X)
/// // H = \hbar * omega * sigma_x / 2
/// let h_bar = 1.0;
/// let omega = 2.0;
/// let factor = Complex::new(h_bar * omega / 2.0, 0.0);
/// let hamiltonian_matrix = DMatrix::from_row_slice(2, 2, &[
///     Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), // 0  1
///     Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), // 1  0
/// ]) * factor;
/// let hamiltonian = QuantumOperator::new(hamiltonian_matrix);
///
/// // 2. Initialize state in |0> (Up)
/// let initial_vector = DVector::from_vec(vec![
///     Complex::new(1.0, 0.0),
///     Complex::new(0.0, 0.0)
/// ]);
/// let initial_state = QuantumState::new(initial_vector);
///
/// // 3. Evolve for time t = pi / omega (Should flip to |1>)
/// let t = std::f64::consts::PI / omega;
/// let final_state = evolve_state(&initial_state, &hamiltonian, t, h_bar);
///
/// // 4. Check probability of being in |1> (Down)
/// let prob_down = final_state.probability_density()[1];
/// assert!((prob_down - 1.0).abs() < 1e-4);
/// ```
pub fn evolve_state(
    state: &QuantumState,
    hamiltonian: &QuantumOperator,
    t: f64,
    h_bar: f64,
) -> QuantumState {
    let u = time_evolution_operator(hamiltonian, t, h_bar);
    QuantumState::new(&u.matrix * &state.vector)
}
