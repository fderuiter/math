use super::encoder::Encoder;
use super::decoder::Decoder;

/// The full Transformer model container.
///
/// This struct currently acts as a container for the Encoder and Decoder components.
/// In a production-ready implementation, it would likely also manage:
/// - Input and Output Embeddings
/// - Positional Encodings (applied before the Encoder/Decoder)
/// - Final Linear Projection and Softmax
///
/// # Usage
///
/// Since this struct does not currently implement a `new` method or a `forward` pass
/// coordinating both components, users are encouraged to instantiate `Encoder`
/// and `Decoder` separately or construct this struct manually if needed.
///
/// ```rust
/// use math_explorer::ai::transformer::{Transformer, Encoder, Decoder};
///
/// let encoder = Encoder::new(6, 512, 8, 2048);
/// let decoder = Decoder::new(6, 512, 8, 2048);
///
/// let model = Transformer {
///     encoder,
///     decoder,
/// };
/// ```
pub struct Transformer {
    /// The encoder component of the transformer.
    pub encoder: Encoder,
    /// The decoder component of the transformer.
    pub decoder: Decoder,
    // Note: In a full implementation, this would also include embedding layers,
    // positional encoding addition, and a final linear layer + softmax.
}
