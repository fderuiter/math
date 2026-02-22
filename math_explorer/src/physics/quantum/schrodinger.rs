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
///
/// # Examples
///
/// Calculate the time evolution operator for a spin-1/2 system with Hamiltonian H = \sigma_z.
///
/// ```
/// use math_explorer::physics::quantum::{time_evolution_operator, sigma_z};
/// use std::f64::consts::PI;
/// use num_complex::Complex;
///
/// // 1. Define Hamiltonian H = sigma_z (Spin-1/2 Z-operator)
/// // \sigma_z = |0><0| - |1><1| = diag(1, -1)
/// let h_hat = sigma_z();
///
/// // 2. Calculate U(t) for t = PI, h_bar = 1.0
/// // U(t) = exp(-i * H * t / h_bar)
/// // U(PI) = exp(-i * diag(1, -1) * PI) = diag(exp(-i*PI), exp(i*PI))
/// //       = diag(-1, -1) = -I
/// let t = PI;
/// let h_bar = 1.0;
/// let u_op = time_evolution_operator(&h_hat, t, h_bar);
///
/// // 3. Verify result is approximately -I
/// let u_matrix = u_op.matrix;
/// let expected_diag = Complex::new(-1.0, 0.0);
///
/// // Check diagonal elements
/// assert!((u_matrix[(0, 0)] - expected_diag).norm() < 1e-9, "U(0,0) incorrect");
/// assert!((u_matrix[(1, 1)] - expected_diag).norm() < 1e-9, "U(1,1) incorrect");
///
/// // Check off-diagonal elements are zero
/// assert!(u_matrix[(0, 1)].norm() < 1e-9, "U(0,1) should be 0");
/// assert!(u_matrix[(1, 0)].norm() < 1e-9, "U(1,0) should be 0");
/// ```
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
