#![allow(missing_docs)]
use approx::assert_relative_eq;
use domain_physics::physics::quantum::{
    QuantumOperator, QuantumState, dft_operator, sigma_x, sigma_y, sigma_z,
};
use nalgebra::{DMatrix, DVector};
use num_complex::Complex;

#[test]
#[verified_engine::verified]
fn test_normalization() {
    let c1 = Complex::new(3.0, 0.0);
    let c2 = Complex::new(4.0, 0.0);
    let vec = DVector::from_vec(vec![c1, c2]);
    let state = QuantumState::new(vec);

    let norm = state.norm();
    assert_relative_eq!(
        norm,
        5.0,
        epsilon = math_commons::registry::TOLERANCE_STANDARD
    );

    let normalized_state = state.normalize();
    assert_relative_eq!(
        normalized_state.norm(),
        1.0,
        epsilon = math_commons::registry::TOLERANCE_STANDARD
    );
}

#[test]
#[verified_engine::verified]
fn test_inner_product() {
    // |0> = [1, 0]
    // |1> = [0, 1]
    let zero = QuantumState::new(DVector::from_vec(vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
    ]));
    let one = QuantumState::new(DVector::from_vec(vec![
        Complex::new(0.0, 0.0),
        Complex::new(1.0, 0.0),
    ]));

    let prod = zero.inner_product(&one);
    assert_relative_eq!(
        prod.re,
        0.0,
        epsilon = math_commons::registry::TOLERANCE_STANDARD
    );
    assert_relative_eq!(
        prod.im,
        0.0,
        epsilon = math_commons::registry::TOLERANCE_STANDARD
    );

    let prod_self = zero.inner_product(&zero);
    assert_relative_eq!(
        prod_self.re,
        1.0,
        epsilon = math_commons::registry::TOLERANCE_STANDARD
    );
}

#[test]
#[verified_engine::verified]
fn test_pauli_commutators() {
    let sx = sigma_x();
    let sy = sigma_y();
    let sz = sigma_z();

    // [Sx, Sy] = i 2 Sz
    let comm_xy = sx.commutator(&sy);
    let expected = &sz.matrix * Complex::new(0.0, 2.0);

    let diff = comm_xy.matrix - expected;
    assert!(diff.norm() < math_commons::registry::TOLERANCE_STANDARD);
}

#[test]
#[verified_engine::verified]
fn test_time_evolution() {
    // Hamiltonian H = w * Sz. w = 1.
    // Time t. U(t) = exp(-i Sz t / h_bar).
    // Sz = diag(1, -1).
    // U(t) = diag(e^{-it}, e^{it}) (assuming h_bar=1)

    let sz = sigma_z();
    let t = std::f64::consts::PI; // t = pi
    let h_bar = 1.0;

    let u = domain_physics::physics::quantum::time_evolution_operator(&sz, t, h_bar);

    // Expected: diag(exp(-i*pi), exp(i*pi)) = diag(-1, -1) = -I
    let result = u.matrix;
    let expected = DMatrix::from_diagonal(&DVector::from_vec(vec![
        Complex::new(-1.0, 0.0),
        Complex::new(-1.0, 0.0),
    ]));

    // Note: e^{-i*pi} = -1. e^{i*pi} = -1.
    // Allow small error due to float arithmetic
    assert!((result - expected).norm() < math_commons::registry::TOLERANCE_STANDARD);
}

#[test]
#[verified_engine::verified]
fn test_dft_unitarity() {
    let n = 4;
    let dft = dft_operator(n);

    // Check if DFT is unitary: U^dagger U = I
    let adjoint = QuantumOperator::new(dft.matrix.adjoint());
    let identity_approx = &adjoint.matrix * &dft.matrix;

    let identity = DMatrix::identity(n, n);
    let i_complex = identity.map(|x| Complex::new(x, 0.0));

    assert!((identity_approx - i_complex).norm() < math_commons::registry::TOLERANCE_STANDARD);
}
