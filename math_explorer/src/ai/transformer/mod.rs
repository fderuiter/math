//! # Transformer Architecture
//!
//! This module implements the Transformer architecture as described in the paper
//! "Attention Is All You Need".
//!
//! ## Architecture Overview
//!
//! The Transformer follows an Encoder-Decoder structure using stacked self-attention
//! and point-wise, fully connected layers.
//!
//! ```mermaid
//! graph TD
//!     subgraph Encoder
//!     Input[Input] --> EncLayer1[Encoder Layer 1]
//!     EncLayer1 --> EncLayer2[Encoder Layer 2]
//!     EncLayer2 --> EncOutput[Encoder Output]
//!     end
//!
//!     subgraph Decoder
//!     Target[Target] --> DecLayer1[Decoder Layer 1]
//!     EncOutput --> DecLayer1
//!     DecLayer1 --> DecLayer2[Decoder Layer 2]
//!     EncOutput --> DecLayer2
//!     DecLayer2 --> Output[Output]
//!     end
//! ```
//!
//! ## Components
//!
//! - **Attention**: Scaled Dot-Product Attention and Multi-Head Attention.
//! - **Feed Forward**: Position-wise Feed-Forward Networks.
//! - **Layer Norm**: Layer Normalization.
//! - **Encoder**: Stack of `EncoderLayer`s.
//! - **Decoder**: Stack of `DecoderLayer`s.

pub mod attention;
pub mod decoder;
pub mod encoder;
pub mod feed_forward;
pub mod layer_norm;
pub mod model;
pub mod positional_encoding;
pub mod traits;

pub use attention::MultiHeadAttention;
pub use decoder::{Decoder, DecoderLayer};
pub use encoder::{Encoder, EncoderLayer};
pub use feed_forward::FeedForward;
pub use layer_norm::LayerNorm;
pub use model::Transformer;
pub use traits::{AttentionMechanism, FeedForwardNetwork, NormalizationLayer};
