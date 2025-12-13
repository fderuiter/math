use crate::physics::quantum::types::{QuantumOperator, QuantumState};
use num_complex::Complex;

/// Calculates the time evolution operator U(t) = e^{-iHt/\hbar}.
///
/// This assumes the Hamiltonian H is time-independent.
///
/// # Arguments
/// * `hamiltonian` - The Hamiltonian operator \hat{H}.
/// * `t` - The time duration t.
/// * `h_bar` - The reduced Planck constant \hbar.
///
/// # Returns
/// The unitary operator U(t).
pub fn time_evolution_operator(
    hamiltonian: &QuantumOperator,
    t: f64,
    h_bar: f64,
) -> QuantumOperator {
    // U(t) = exp(-i * H * t / h_bar)
    // Argument of exp is a matrix.
    let i = Complex::new(0.0, 1.0);
    let factor = -i * t / h_bar;

    // nalgebra DMatrix has an exp() method.
    // We scale the matrix by the factor first.
    // Note: nalgebra's exp() is for square matrices.

    let scaled_h = &hamiltonian.matrix * factor;
    let u_matrix = scaled_h.exp();

    QuantumOperator::new(u_matrix)
}

/// Evolves a quantum state over time using the Schrödinger equation.
///
/// |\psi(t)\rangle = e^{-iHt/\hbar} |\psi(0)\rangle
pub fn evolve_state(
    state: &QuantumState,
    hamiltonian: &QuantumOperator,
    t: f64,
    h_bar: f64,
) -> QuantumState {
    let u = time_evolution_operator(hamiltonian, t, h_bar);
    QuantumState::new(&u.matrix * &state.vector)
}
