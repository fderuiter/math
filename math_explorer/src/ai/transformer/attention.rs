// Implementation of Scaled Dot-Product Attention and Multi-Head Attention.

use crate::ai::activations::softmax_row_wise;
use nalgebra::DMatrix;

/// Computes the Scaled Dot-Product Attention.
///
/// This function implements the core attention mechanism:
/// `Attention(Q, K, V) = softmax( (Q * K^T) / sqrt(d_k) ) * V`
///
/// # Arguments
/// * `q`: Queries matrix of shape (sequence_length, d_k)
/// * `k`: Keys matrix of shape (sequence_length, d_k)
/// * `v`: Values matrix of shape (sequence_length, d_v)
/// * `mask`: Optional mask to apply to the scores before softmax.
///
/// # Returns
/// A tuple containing the output matrix of shape (sequence_length, d_v) and the
/// attention weights matrix of shape (sequence_length, sequence_length).
pub fn scaled_dot_product_attention(
    q: &DMatrix<f64>,
    k: &DMatrix<f64>,
    v: &DMatrix<f64>,
    mask: Option<&DMatrix<f64>>,
) -> (DMatrix<f64>, DMatrix<f64>) {
    let d_k = q.ncols() as f64;

    // 1. Calculate scores: Q * K^T
    let mut scores = q * k.transpose();

    // 2. Scale scores
    // Prevent division by zero if d_k is 0 (which would cause NaNs).
    // If d_k is 0, we skip scaling, leaving scores as 0 (valid for dot product of empty vectors).
    if d_k > 0.0 {
        scores /= d_k.sqrt();
    }

    // 3. Apply mask if provided (e.g., for padding or preventing future token peeking)
    if let Some(m) = mask {
        // The mask is added to the scores. For positions to be ignored,
        // the mask should contain a very large negative number.
        scores += m;
    }

    // 4. Apply softmax to get attention weights
    let attention_weights = softmax_row_wise(&scores);

    // 5. Multiply weights by V to get the final output
    let output = &attention_weights * v;

    (output, attention_weights)
}

/// Multi-Head Attention mechanism.
///
/// This struct holds the learnable weight matrices for the projections.
pub struct MultiHeadAttention {
    d_model: usize,
    h: usize,
    d_k: usize,

    /// Weight matrices for query projections for each head
    pub w_q: Vec<DMatrix<f64>>,
    /// Weight matrices for key projections for each head
    pub w_k: Vec<DMatrix<f64>>,
    /// Weight matrices for value projections for each head
    pub w_v: Vec<DMatrix<f64>>,
    /// Final output weight matrix
    pub w_o: DMatrix<f64>,
}

impl MultiHeadAttention {
    /// Creates a new `MultiHeadAttention` instance.
    ///
    /// In a real model, the weight matrices would be initialized randomly.
    /// Here, they are initialized as zeros for simplicity.
    pub fn new(d_model: usize, h: usize) -> Self {
        assert!(
            d_model.is_multiple_of(h),
            "d_model must be divisible by the number of heads h"
        );
        let d_k = d_model / h;

        Self {
            d_model,
            h,
            d_k,
            w_q: (0..h).map(|_| DMatrix::zeros(d_model, d_k)).collect(),
            w_k: (0..h).map(|_| DMatrix::zeros(d_model, d_k)).collect(),
            w_v: (0..h).map(|_| DMatrix::zeros(d_model, d_k)).collect(),
            w_o: DMatrix::zeros(d_model, d_model),
        }
    }

    /// Performs the forward pass for the Multi-Head Attention layer.
    pub fn forward(
        &self,
        q: &DMatrix<f64>,
        k: &DMatrix<f64>,
        v: &DMatrix<f64>,
        mask: Option<&DMatrix<f64>>,
    ) -> DMatrix<f64> {
        let mut head_outputs = Vec::with_capacity(self.h);

        // 1. Linearly project Q, K, V for each head and apply attention
        for i in 0..self.h {
            let q_proj = q * &self.w_q[i];
            let k_proj = k * &self.w_k[i];
            let v_proj = v * &self.w_v[i];

            let (head_output, _attention_weights) =
                scaled_dot_product_attention(&q_proj, &k_proj, &v_proj, mask);
            head_outputs.push(head_output);
        }

        // 2. Concatenate the outputs of the heads
        let sequence_length = q.nrows();
        let mut concatenated_output = DMatrix::zeros(sequence_length, self.d_model);
        for (i, head_output) in head_outputs.iter().enumerate() {
            let start_col = i * self.d_k;
            concatenated_output
                .view_mut((0, start_col), (sequence_length, self.d_k))
                .copy_from(head_output);
        }

        // 3. Apply the final linear projection
        concatenated_output * &self.w_o
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scaled_dot_product_attention_dims() {
        let seq_len = 10;
        let d_k = 64;
        let d_v = 64;
        let q = DMatrix::zeros(seq_len, d_k);
        let k = DMatrix::zeros(seq_len, d_k);
        let v = DMatrix::zeros(seq_len, d_v);

        let (output, attn_weights) = scaled_dot_product_attention(&q, &k, &v, None);

        assert_eq!(output.nrows(), seq_len);
        assert_eq!(output.ncols(), d_v);
        assert_eq!(attn_weights.nrows(), seq_len);
        assert_eq!(attn_weights.ncols(), seq_len);
    }

    #[test]
    fn test_multi_head_attention_dims() {
        let seq_len = 10;
        let d_model = 512;
        let h = 8;

        let q = DMatrix::zeros(seq_len, d_model);
        let k = DMatrix::zeros(seq_len, d_model);
        let v = DMatrix::zeros(seq_len, d_model);

        let mha = MultiHeadAttention::new(d_model, h);
        let output = mha.forward(&q, &k, &v, None);

        assert_eq!(output.nrows(), seq_len);
        assert_eq!(output.ncols(), d_model);
    }

    #[test]
    fn test_scaled_dot_product_attention_zero_dim() {
        use approx::assert_relative_eq;
        let seq_len = 5;
        let d_k = 0; // Trigger potential division by zero
        let d_v = 10;

        let q = DMatrix::zeros(seq_len, d_k);
        let k = DMatrix::zeros(seq_len, d_k);
        let v = DMatrix::zeros(seq_len, d_v);

        let (_output, weights) = scaled_dot_product_attention(&q, &k, &v, None);

        // With d_k=0, scores should be 0. Softmax should be uniform (1/seq_len = 0.2).
        let expected_weight = 1.0 / (seq_len as f64);

        // Check that we don't have NaNs and weights are uniform
        assert!(!weights[(0, 0)].is_nan());
        assert_relative_eq!(weights[(0, 0)], expected_weight, epsilon = 1e-6);
    }
}
