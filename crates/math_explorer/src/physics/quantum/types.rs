use nalgebra::{DMatrix, DVector};
use num_complex::Complex;

/// Represents the state of a quantum system as a vector in a Hilbert space.
///
/// In quantum mechanics, the state of a system is described by a state vector |\psi\rangle
/// belonging to a complex Hilbert space. The state vector contains all the information
/// about the system.
///
/// The probability of finding the system in a particular state is related to the
/// square of the absolute value of the vector's components (Born Rule).
#[derive(Clone, Debug)]
pub struct QuantumState {
    /// The complex vector representing the state |\psi\rangle.
    pub vector: DVector<Complex<f64>>,
}

impl QuantumState {
    /// Creates a new QuantumState from a vector of complex numbers.
    ///
    /// # Arguments
    /// * `vector` - A `DVector<Complex<f64>>` representing the state.
    pub fn new(vector: DVector<Complex<f64>>) -> Self {
        Self { vector }
    }

    /// Calculates the norm (magnitude) of the state vector.
    ///
    /// \sqrt{\langle\psi|\psi\rangle}
    pub fn norm(&self) -> f64 {
        self.vector.norm()
    }

    /// Normalizes the state vector so that its norm is 1.
    ///
    /// |\psi'\rangle = \frac{|\psi\rangle}{\sqrt{\langle\psi|\psi\rangle}}
    ///
    /// Returns a new QuantumState.
    pub fn normalize(&self) -> Self {
        let norm = self.norm();
        if norm == 0.0 {
            // Can't normalize zero vector, return as is or handle error.
            // Returning as is for now, but in QM this is invalid state.
            return self.clone();
        }
        Self {
            // Need to divide by complex or scale by 1/norm
            vector: self.vector.clone() / Complex::new(norm, 0.0),
        }
    }

    /// Calculates the inner product with another state (Bra-Ket notation: \langle\phi|\psi\rangle).
    ///
    /// This is equivalent to the dot product of the conjugate transpose of `other` (Bra)
    /// with `self` (Ket). Note: In physics notation <phi|psi>, phi is the bra.
    /// Here `self.inner_product(&other)` computes <self|other>.
    ///
    /// # Arguments
    /// * `other` - The Ket vector |\phi\rangle (argument).
    ///
    /// # Returns
    /// The complex scalar result of \langle\psi|\phi\rangle.
    pub fn inner_product(&self, other: &QuantumState) -> Complex<f64> {
        self.vector.dotc(&other.vector)
    }

    /// Returns the probability density of the state.
    ///
    /// For a discrete state vector, this returns a vector of real probabilities
    /// corresponding to each basis state: P_i = |\psi_i|^2.
    pub fn probability_density(&self) -> DVector<f64> {
        self.vector.map(|c| c.norm_sqr())
    }
}

/// Represents a linear operator acting on the Hilbert space.
///
/// In quantum mechanics, physical observables are represented by linear operators.
/// Hermitian operators correspond to measurable quantities.
#[derive(Clone, Debug)]
pub struct QuantumOperator {
    /// The complex matrix representing the operator \hat{A}.
    pub matrix: DMatrix<Complex<f64>>,
}

impl QuantumOperator {
    /// Creates a new QuantumOperator from a complex matrix.
    pub fn new(matrix: DMatrix<Complex<f64>>) -> Self {
        Self { matrix }
    }

    /// Checks if the operator is Hermitian (Self-adjoint).
    ///
    /// \hat{A} = \hat{A}^\dagger
    pub fn is_hermitian(&self, tolerance: f64) -> bool {
        let adjoint = self.matrix.adjoint();
        (self.matrix.clone() - adjoint).norm() < tolerance
    }

    /// Calculates the expectation value of the operator for a given state.
    ///
    /// \langle \hat{A} \rangle = \langle \psi | \hat{A} | \psi \rangle
    pub fn expectation_value(&self, state: &QuantumState) -> Complex<f64> {
        // <psi| (A |psi>)
        let bra = state.vector.adjoint();
        let ket_transformed = &self.matrix * &state.vector;
        (bra * ket_transformed)[(0, 0)]
    }

    /// Calculates the commutator with another operator.
    ///
    /// [\hat{A}, \hat{B}] = \hat{A}\hat{B} - \hat{B}\hat{A}
    pub fn commutator(&self, other: &QuantumOperator) -> QuantumOperator {
        let ab = &self.matrix * &other.matrix;
        let ba = &other.matrix * &self.matrix;
        QuantumOperator { matrix: ab - ba }
    }
}
