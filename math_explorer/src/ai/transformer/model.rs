use super::encoder::Encoder;
use super::decoder::Decoder;

/// The full Transformer model.
pub struct Transformer {
    /// The encoder component of the transformer.
    pub encoder: Encoder,
    /// The decoder component of the transformer.
    pub decoder: Decoder,
    // Note: In a full implementation, this would also include embedding layers,
    // positional encoding addition, and a final linear layer + softmax.
}
