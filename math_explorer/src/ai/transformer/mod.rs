// Main Transformer model implementation, including Encoder and Decoder.

pub mod encoder;
pub mod decoder;

pub use encoder::{Encoder, EncoderLayer};
pub use decoder::{Decoder, DecoderLayer};
pub use crate::ai::normalization::LayerNorm;

/// The full Transformer model.
pub struct Transformer {
    /// The encoder component of the transformer.
    pub encoder: Encoder,
    /// The decoder component of the transformer.
    pub decoder: Decoder,
    // Note: In a full implementation, this would also include embedding layers,
    // positional encoding addition, and a final linear layer + softmax.
}
