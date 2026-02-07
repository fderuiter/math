use super::attention::MultiHeadAttention;
use super::feed_forward::FeedForward;
use super::layer_norm::LayerNorm;
use crate::ai::transformer::traits::{AttentionMechanism, FeedForwardNetwork, NormalizationLayer};
use nalgebra::DMatrix;

/// A single Encoder layer, containing self-attention and a feed-forward network,
/// with residual connections and layer normalization.
pub struct EncoderLayer {
    /// The multi-head self-attention mechanism.
    pub self_attn: Box<dyn AttentionMechanism>,
    /// The position-wise feed-forward network.
    pub feed_forward: Box<dyn FeedForwardNetwork>,
    /// Layer normalization applied after the attention mechanism.
    pub norm1: Box<dyn NormalizationLayer>,
    /// Layer normalization applied after the feed-forward network.
    pub norm2: Box<dyn NormalizationLayer>,
}

impl EncoderLayer {
    /// Creates a new `EncoderLayer` instance.
    ///
    /// # Arguments
    ///
    /// * `d_model`: The dimension of the model's embeddings and hidden states.
    /// * `h`: The number of attention heads.
    /// * `d_ff`: The dimension of the inner layer of the feed-forward network.
    ///
    /// # Returns
    ///
    /// A new `EncoderLayer` instance initialized with the given parameters.
    pub fn new(d_model: usize, h: usize, d_ff: usize) -> Self {
        Self {
            self_attn: Box::new(MultiHeadAttention::new(d_model, h)),
            feed_forward: Box::new(FeedForward::new(d_model, d_ff)),
            norm1: Box::new(LayerNorm::new(d_model)),
            norm2: Box::new(LayerNorm::new(d_model)),
        }
    }

    /// Creates a new `EncoderLayer` with custom components.
    pub fn new_with_components(
        self_attn: Box<dyn AttentionMechanism>,
        feed_forward: Box<dyn FeedForwardNetwork>,
        norm1: Box<dyn NormalizationLayer>,
        norm2: Box<dyn NormalizationLayer>,
    ) -> Self {
        Self {
            self_attn,
            feed_forward,
            norm1,
            norm2,
        }
    }

    /// Performs the forward pass of the encoder layer.
    ///
    /// # Arguments
    ///
    /// * `x`: The input matrix of shape (sequence_length, d_model).
    /// * `mask`: Optional mask for the self-attention mechanism.
    ///
    /// # Returns
    ///
    /// The output matrix of shape (sequence_length, d_model).
    pub fn forward(&self, x: &DMatrix<f64>, mask: Option<&DMatrix<f64>>) -> DMatrix<f64> {
        let attn_output = self.self_attn.forward(x, x, x, mask);
        let x_plus_attn = x + attn_output;
        let normed_attn = self.norm1.forward(&x_plus_attn);

        let ff_output = self.feed_forward.forward(&normed_attn);
        let normed_attn_plus_ff = &normed_attn + ff_output;
        self.norm2.forward(&normed_attn_plus_ff)
    }
}

/// The full Encoder, composed of a stack of identical EncoderLayers.
pub struct Encoder {
    /// A vector of `EncoderLayer` instances.
    pub layers: Vec<EncoderLayer>,
}

impl Encoder {
    /// Creates a new `Encoder` instance.
    ///
    /// # Arguments
    ///
    /// * `num_layers`: The number of encoder layers to stack.
    /// * `d_model`: The dimension of the model.
    /// * `h`: The number of attention heads.
    /// * `d_ff`: The dimension of the feed-forward network.
    ///
    /// # Returns
    ///
    /// A new `Encoder` instance.
    pub fn new(num_layers: usize, d_model: usize, h: usize, d_ff: usize) -> Self {
        Self {
            layers: (0..num_layers)
                .map(|_| EncoderLayer::new(d_model, h, d_ff))
                .collect(),
        }
    }

    /// Performs the forward pass through all encoder layers.
    ///
    /// # Arguments
    ///
    /// * `x`: The input matrix.
    /// * `mask`: Optional mask for attention.
    ///
    /// # Returns
    ///
    /// The output matrix after passing through all layers.
    pub fn forward(&self, mut x: DMatrix<f64>, mask: Option<&DMatrix<f64>>) -> DMatrix<f64> {
        for layer in &self.layers {
            x = layer.forward(&x, mask);
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
