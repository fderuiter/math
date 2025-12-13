/// The AI module contains implementations of various machine learning models and concepts.
///
/// It currently includes:
/// - Attention mechanisms (Scaled Dot-Product and Multi-Head Attention).
/// - Position-wise Feed-Forward Networks.
/// - Positional Encodings.
/// - A full Transformer implementation.
/// - Score Distillation Sampling (SDS) and NeRF.
pub mod activations;
pub mod transformer;
pub mod sds;
pub mod self_calibration;
pub mod utils;

// Re-export transformer components for backward compatibility.
pub use transformer::attention;
pub use transformer::feed_forward;
pub use transformer::positional_encoding;
