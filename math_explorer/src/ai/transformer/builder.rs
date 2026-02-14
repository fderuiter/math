use crate::ai::error::AIError;
use super::model::Transformer;
use super::encoder::Encoder;
use super::decoder::Decoder;
use super::attention::MultiHeadAttention;
use super::feed_forward::FeedForward;
use super::layer_norm::LayerNorm;

/// Builder for constructing a standard Transformer model.
///
/// This builder ensures that the model configuration is consistent (e.g., d_model is divisible by heads)
/// and prevents invalid states.
pub struct TransformerBuilder {
    d_model: Option<usize>,
    heads: Option<usize>,
    d_ff: Option<usize>,
    layers: Option<usize>,
}

impl Default for TransformerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformerBuilder {
    /// Creates a new `TransformerBuilder` with no configuration set.
    pub fn new() -> Self {
        Self {
            d_model: None,
            heads: None,
            d_ff: None,
            layers: None,
        }
    }

    /// Sets the model dimension (d_model).
    pub fn d_model(mut self, d_model: usize) -> Self {
        self.d_model = Some(d_model);
        self
    }

    /// Sets the number of attention heads (h).
    pub fn heads(mut self, heads: usize) -> Self {
        self.heads = Some(heads);
        self
    }

    /// Sets the feed-forward dimension (d_ff).
    pub fn d_ff(mut self, d_ff: usize) -> Self {
        self.d_ff = Some(d_ff);
        self
    }

    /// Sets the number of layers (N) for both encoder and decoder.
    pub fn layers(mut self, layers: usize) -> Self {
        self.layers = Some(layers);
        self
    }

    /// Builds the `Transformer` instance.
    ///
    /// # Errors
    /// Returns `AIError` if:
    /// * Any required parameter is missing.
    /// * `d_model` is not divisible by `heads`.
    /// * `heads` is 0.
    /// * `layers` is 0.
    pub fn build(self) -> Result<Transformer<MultiHeadAttention, FeedForward, LayerNorm>, AIError> {
        let d_model = self.d_model.ok_or_else(|| AIError::MissingParameter {
            name: "d_model".to_string(),
        })?;
        let heads = self.heads.ok_or_else(|| AIError::MissingParameter {
            name: "heads".to_string(),
        })?;
        let d_ff = self.d_ff.ok_or_else(|| AIError::MissingParameter {
            name: "d_ff".to_string(),
        })?;
        let layers = self.layers.ok_or_else(|| AIError::MissingParameter {
            name: "layers".to_string(),
        })?;

        // Basic validation
        if heads == 0 {
            return Err(AIError::InvalidParameter {
                name: "heads".to_string(),
                value: 0.0,
            });
        }
        if layers == 0 {
            return Err(AIError::InvalidParameter {
                name: "layers".to_string(),
                value: 0.0,
            });
        }
        #[allow(clippy::manual_is_multiple_of)]
        if d_model % heads != 0 {
             return Err(AIError::DimensionMismatch {
                expected: format!("multiple of {}", heads),
                got: d_model.to_string(),
            });
        }

        // Construct components
        let encoder = Encoder::new(layers, d_model, heads, d_ff)?;
        let decoder = Decoder::new(layers, d_model, heads, d_ff)?;

        Ok(Transformer {
            encoder,
            decoder,
        })
    }
}
