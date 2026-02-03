use nalgebra::{DMatrix, DVector};
use thiserror::Error;

/// Errors related to tensor operations.
#[derive(Error, Debug)]
pub enum TensorError {
    #[error("Dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    #[error("Singular metric tensor, cannot invert")]
    SingularMetric,
    #[error("Index out of bounds")]
    IndexOutOfBounds,
}

/// A contravariant vector ($A^\mu$).
/// Components transform like coordinate differentials.
#[derive(Debug, Clone, PartialEq)]
pub struct ContravariantVector(pub DVector<f64>);

impl ContravariantVector {
    pub fn new(data: DVector<f64>) -> Self {
        Self(data)
    }

    pub fn dim(&self) -> usize {
        self.0.len()
    }
}

/// A covariant vector ($A_\mu$).
/// Components transform inversely to contravariant vectors.
#[derive(Debug, Clone, PartialEq)]
pub struct CovariantVector(pub DVector<f64>);

impl CovariantVector {
    pub fn new(data: DVector<f64>) -> Self {
        Self(data)
    }

    pub fn dim(&self) -> usize {
        self.0.len()
    }
}

/// A rank-2 tensor (can be covariant, contravariant, or mixed).
/// For now, we represent it as a matrix.
#[derive(Debug, Clone, PartialEq)]
pub struct Rank2Tensor(pub DMatrix<f64>);

impl Rank2Tensor {
    pub fn new(data: DMatrix<f64>) -> Self {
        Self(data)
    }
}
