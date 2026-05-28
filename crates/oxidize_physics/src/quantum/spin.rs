use crate::quantum::types::QuantumOperator;
use nalgebra::DMatrix;
use num_complex::Complex;

/// Returns the Pauli matrix \sigma_x.
///
/// \sigma_x = [[0, 1], [1, 0]]
pub fn sigma_x() -> QuantumOperator {
    let zero = Complex::new(0.0, 0.0);
    let one = Complex::new(1.0, 0.0);
    let elements = vec![zero, one, one, zero];
    QuantumOperator::new(DMatrix::from_vec(2, 2, elements))
}

/// Returns the Pauli matrix \sigma_y.
///
/// \sigma_y = [[0, -i], [i, 0]]
pub fn sigma_y() -> QuantumOperator {
    let zero = Complex::new(0.0, 0.0);
    let i = Complex::new(0.0, 1.0);
    // Note: DMatrix::from_vec fills column by column.
    // Matrix is [[0, -i], [i, 0]]
    // Col 1: 0, i
    // Col 2: -i, 0
    let elements = vec![zero, i, -i, zero];
    QuantumOperator::new(DMatrix::from_vec(2, 2, elements))
}

/// Returns the Pauli matrix \sigma_z.
///
/// \sigma_z = [[1, 0], [0, -1]]
pub fn sigma_z() -> QuantumOperator {
    let zero = Complex::new(0.0, 0.0);
    let one = Complex::new(1.0, 0.0);
    let neg_one = Complex::new(-1.0, 0.0);
    // Col 1: 1, 0
    // Col 2: 0, -1
    let elements = vec![one, zero, zero, neg_one];
    QuantumOperator::new(DMatrix::from_vec(2, 2, elements))
}
