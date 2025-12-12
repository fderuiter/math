use nalgebra::{DMatrix, RowDVector};

/// Layer Normalization.
pub struct LayerNorm {
    /// Small constant for numerical stability.
    epsilon: f64,
    /// Learnable scale parameter.
    gamma: RowDVector<f64>,
    /// Learnable shift parameter.
    beta: RowDVector<f64>,
}

impl LayerNorm {
    /// Creates a new `LayerNorm` instance.
    ///
    /// # Arguments
    ///
    /// * `d_model`: The dimension of the model.
    ///
    /// # Returns
    ///
    /// A new `LayerNorm` instance.
    pub fn new(d_model: usize) -> Self {
        Self {
            epsilon: 1e-6,
            gamma: RowDVector::from_element(d_model, 1.0),
            beta: RowDVector::from_element(d_model, 0.0),
        }
    }

    /// Applies layer normalization to the input.
    ///
    /// # Arguments
    ///
    /// * `x`: The input matrix.
    ///
    /// # Returns
    ///
    /// The normalized matrix.
    pub fn forward(&self, x: &DMatrix<f64>) -> DMatrix<f64> {
        let mut output = DMatrix::zeros(x.nrows(), x.ncols());
        for r in 0..x.nrows() {
            let row = x.row(r);
            let mean = row.mean();

            let variance = row.variance();

            let inv_std = 1.0 / (variance + self.epsilon).sqrt();

            let mut normalized_row = row.clone_owned().add_scalar(-mean);
            normalized_row *= inv_std;

            let final_row = self.gamma.component_mul(&normalized_row) + &self.beta;
            output.set_row(r, &final_row);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_layer_norm() {
        let d_model = 4;
        let layer_norm = LayerNorm::new(d_model);
        let mut input = DMatrix::from_row_slice(1, 4, &[1.0, 2.0, 3.0, 4.0]);
        input.apply(|x| *x *= 10.0); // Scale up to make mean/variance more meaningful

        let output = layer_norm.forward(&input);

        // After normalization (before gamma/beta), mean should be ~0 and std dev ~1.
        // Since gamma=1 and beta=0 by default, the output should be normalized.
        let output_row = output.row(0);
        assert_relative_eq!(output_row.mean(), 0.0, epsilon = 1e-6);
        assert_relative_eq!(output_row.variance().sqrt(), 1.0, epsilon = 1e-6);
    }
}
