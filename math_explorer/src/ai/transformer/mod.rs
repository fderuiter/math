#![doc = include_str!("README.md")]

pub mod attention;
pub mod decoder;
pub mod encoder;
pub mod feed_forward;
pub mod layer_norm;
pub mod model;
pub mod positional_encoding;
pub mod tokenization;
pub mod traits;

pub use attention::MultiHeadAttention;
pub use decoder::{Decoder, DecoderLayer};
pub use encoder::{Encoder, EncoderLayer};
pub use feed_forward::FeedForward;
pub use layer_norm::LayerNorm;
pub use model::Transformer;
pub use traits::{AttentionMechanism, FeedForwardNetwork, NormalizationLayer};
