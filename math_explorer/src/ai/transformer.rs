// Main Transformer model implementation, including Encoder and Decoder.

use super::attention::MultiHeadAttention;
use super::feed_forward::FeedForward;
use nalgebra::{DMatrix, RowDVector};

// --- EncoderLayer and Encoder implementations from before ---

/// A single Encoder layer, containing self-attention and a feed-forward network,
/// with residual connections and layer normalization.
pub struct EncoderLayer {
    pub self_attn: MultiHeadAttention,
    pub feed_forward: FeedForward,
    pub norm1: LayerNorm,
    pub norm2: LayerNorm,
}

impl EncoderLayer {
    pub fn new(d_model: usize, h: usize, d_ff: usize) -> Self {
        Self {
            self_attn: MultiHeadAttention::new(d_model, h),
            feed_forward: FeedForward::new(d_model, d_ff),
            norm1: LayerNorm::new(d_model),
            norm2: LayerNorm::new(d_model),
        }
    }

    pub fn forward(&self, x: &DMatrix<f64>, mask: Option<&DMatrix<f64>>) -> DMatrix<f64> {
        let attn_output = self.self_attn.forward(x, x, x, mask);
        let x_plus_attn = x + attn_output;
        let normed_attn = self.norm1.forward(&x_plus_attn);

        let ff_output = self.feed_forward.forward(&normed_attn);
        let normed_attn_plus_ff = &normed_attn + ff_output;
        let output = self.norm2.forward(&normed_attn_plus_ff);

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

    #[test]
    fn test_encoder_layer_dims() {
        let seq_len = 10;
        let d_model = 512;
        let h = 8;
        let d_ff = 2048;

        let x = DMatrix::zeros(seq_len, d_model);
        let encoder_layer = EncoderLayer::new(d_model, h, d_ff);
        let output = encoder_layer.forward(&x, None);

        assert_eq!(output.nrows(), seq_len);
        assert_eq!(output.ncols(), d_model);
    }

    #[test]
    fn test_decoder_layer_dims() {
        let seq_len = 10;
        let d_model = 512;
        let h = 8;
        let d_ff = 2048;

        let x = DMatrix::zeros(seq_len, d_model);
        let enc_output = DMatrix::zeros(seq_len, d_model);
        let decoder_layer = DecoderLayer::new(d_model, h, d_ff);
        let output = decoder_layer.forward(&x, &enc_output, None, None);

        assert_eq!(output.nrows(), seq_len);
        assert_eq!(output.ncols(), d_model);
    }
}

/// The full Encoder, composed of a stack of identical EncoderLayers.
pub struct Encoder {
    pub layers: Vec<EncoderLayer>,
}

impl Encoder {
    pub fn new(num_layers: usize, d_model: usize, h: usize, d_ff: usize) -> Self {
        Self {
            layers: (0..num_layers)
                .map(|_| EncoderLayer::new(d_model, h, d_ff))
                .collect(),
        }
    }

    pub fn forward(&self, mut x: DMatrix<f64>, mask: Option<&DMatrix<f64>>) -> DMatrix<f64> {
        for layer in &self.layers {
            x = layer.forward(&x, mask);
        }
        x
    }
}


// --- DecoderLayer and Decoder implementations ---

/// A single Decoder layer.
pub struct DecoderLayer {
    pub self_attn: MultiHeadAttention,
    pub cross_attn: MultiHeadAttention,
    pub feed_forward: FeedForward,
    pub norm1: LayerNorm,
    pub norm2: LayerNorm,
    pub norm3: LayerNorm,
}

impl DecoderLayer {
    pub fn new(d_model: usize, h: usize, d_ff: usize) -> Self {
        Self {
            self_attn: MultiHeadAttention::new(d_model, h),
            cross_attn: MultiHeadAttention::new(d_model, h),
            feed_forward: FeedForward::new(d_model, d_ff),
            norm1: LayerNorm::new(d_model),
            norm2: LayerNorm::new(d_model),
            norm3: LayerNorm::new(d_model),
        }
    }

    pub fn forward(
        &self,
        x: &DMatrix<f64>,
        enc_output: &DMatrix<f64>,
        self_attn_mask: Option<&DMatrix<f64>>,
        cross_attn_mask: Option<&DMatrix<f64>>,
    ) -> DMatrix<f64> {
        // 1. Masked self-attention
        let self_attn_output = self.self_attn.forward(x, x, x, self_attn_mask);
        let x_plus_self_attn = x + self_attn_output;
        let normed_self_attn = self.norm1.forward(&x_plus_self_attn);

        // 2. Cross-attention (Encoder-Decoder attention)
        let cross_attn_output = self
            .cross_attn
            .forward(&normed_self_attn, enc_output, enc_output, cross_attn_mask);
        let normed_self_attn_plus_cross = &normed_self_attn + cross_attn_output;
        let normed_cross_attn = self.norm2.forward(&normed_self_attn_plus_cross);

        // 3. Feed-forward network
        let ff_output = self.feed_forward.forward(&normed_cross_attn);
        let normed_cross_attn_plus_ff = &normed_cross_attn + ff_output;
        let output = self.norm3.forward(&normed_cross_attn_plus_ff);

        output
    }
}

/// The full Decoder, composed of a stack of identical DecoderLayers.
pub struct Decoder {
    pub layers: Vec<DecoderLayer>,
}

impl Decoder {
    pub fn new(num_layers: usize, d_model: usize, h: usize, d_ff: usize) -> Self {
        Self {
            layers: (0..num_layers)
                .map(|_| DecoderLayer::new(d_model, h, d_ff))
                .collect(),
        }
    }

    pub fn forward(
        &self,
        mut x: DMatrix<f64>,
        enc_output: &DMatrix<f64>,
        self_attn_mask: Option<&DMatrix<f64>>,
        cross_attn_mask: Option<&DMatrix<f64>>,
    ) -> DMatrix<f64> {
        for layer in &self.layers {
            x = layer.forward(&x, enc_output, self_attn_mask, cross_attn_mask);
        }
        x
    }
}

// --- Full Transformer and LayerNorm ---

/// The full Transformer model.
pub struct Transformer {
    pub encoder: Encoder,
    pub decoder: Decoder,
    // Note: In a full implementation, this would also include embedding layers,
    // positional encoding addition, and a final linear layer + softmax.
}

/// Layer Normalization.
pub struct LayerNorm {
    epsilon: f64,
    gamma: RowDVector<f64>,
    beta: RowDVector<f64>,
}

impl LayerNorm {
    pub fn new(d_model: usize) -> Self {
        Self {
            epsilon: 1e-6,
            gamma: RowDVector::from_element(d_model, 1.0),
            beta: RowDVector::from_element(d_model, 0.0),
        }
    }

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
