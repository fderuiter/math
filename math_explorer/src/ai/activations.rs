// Implementation of various activation functions using Strategy Pattern.
use nalgebra::{DMatrix, RealField};

/// Trait defining an activation function.
///
/// This allows for different activation functions to be used interchangeably
/// and genericized over the scalar type (e.g., f32, f64).
pub trait ActivationFunction<T: RealField + Copy> {
    /// Applies the activation function in-place.
    fn apply(&self, x: &mut DMatrix<T>);

    /// Applies the activation function and returns a new matrix.
    fn forward(&self, x: &DMatrix<T>) -> DMatrix<T> {
        let mut y = x.clone();
        self.apply(&mut y);
        y
    }
}

/// Rectified Linear Unit (ReLU).
/// ReLU(x) = max(0, x)
#[derive(Debug, Clone, Copy, Default)]
pub struct ReLU;

impl<T: RealField + Copy> ActivationFunction<T> for ReLU {
    fn apply(&self, x: &mut DMatrix<T>) {
        x.apply(|v| *v = v.max(T::zero()));
    }
}

/// Leaky ReLU.
/// LeakyReLU(x) = x if x >= 0, else alpha * x
#[derive(Debug, Clone, Copy)]
pub struct LeakyReLU<T> {
    pub alpha: T,
}

impl<T: RealField + Copy> ActivationFunction<T> for LeakyReLU<T> {
    fn apply(&self, x: &mut DMatrix<T>) {
        let alpha = self.alpha;
        x.apply(|v| {
            if *v < T::zero() {
                *v *= alpha;
            }
        });
    }
}

/// Softmax function.
/// Applies softmax row-wise.
#[derive(Debug, Clone, Copy, Default)]
pub struct Softmax;

impl<T: RealField + Copy> ActivationFunction<T> for Softmax {
    fn apply(&self, x: &mut DMatrix<T>) {
         // Softmax depends on the whole row, so we iterate rows.
         for r in 0..x.nrows() {
            let mut row = x.row_mut(r);
            let max_val = row.max();

            // Map row to exps
            let exps = row.map(|v| (v - max_val).exp());
            let sum_exps = exps.sum();

            if sum_exps > T::zero() {
                let softmax_row = exps.map(|v| v / sum_exps);
                row.copy_from(&softmax_row);
            }
         }
    }
}

// --- Legacy Wrappers ---

/// Applies the Rectified Linear Unit (ReLU) activation function element-wise, in-place.
#[deprecated(since = "0.2.0", note = "Use ReLU struct instead")]
pub fn relu<T: RealField + Copy>(matrix: &mut DMatrix<T>) {
    ReLU.apply(&mut *matrix);
}

/// Applies the softmax function to each row of a matrix.
#[deprecated(since = "0.2.0", note = "Use Softmax struct instead")]
pub fn softmax_row_wise<T: RealField + Copy>(matrix: &DMatrix<T>) -> DMatrix<T> {
    Softmax.forward(matrix)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_relu() {
        let mut matrix = DMatrix::<f64>::from_row_slice(1, 4, &[-1.0, 0.0, 1.0, -2.0]);
        relu(&mut matrix);
        let expected = DMatrix::from_row_slice(1, 4, &[0.0, 0.0, 1.0, 0.0]);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_leaky_relu() {
         let mut matrix = DMatrix::<f32>::from_row_slice(1, 2, &[-1.0, 2.0]);
         let activation = LeakyReLU { alpha: 0.1 };
         activation.apply(&mut matrix);
         let expected = DMatrix::from_row_slice(1, 2, &[-0.1, 2.0]);
         // f32 comparison
         assert!((matrix[(0,0)] - expected[(0,0)]).abs() < 1e-6);
    }

    #[test]
    fn test_softmax_row_wise_sum() {
        let matrix = DMatrix::<f64>::from_row_slice(2, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let softmax_result = softmax_row_wise(&matrix);

        // Each row should sum to 1.0
        assert_relative_eq!(softmax_result.row(0).sum(), 1.0, epsilon = 1e-6);
        assert_relative_eq!(softmax_result.row(1).sum(), 1.0, epsilon = 1e-6);
    }
}
