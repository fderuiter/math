/// The AI module contains implementations of various machine learning models and concepts.
///
/// It currently includes:
/// - Attention mechanisms (Scaled Dot-Product and Multi-Head Attention).
/// - Position-wise Feed-Forward Networks.
/// - Positional Encodings.
/// - A full Transformer implementation.
/// - Score Distillation Sampling (SDS) and NeRF.
/// - 3D Gaussian Splatting (3DGS).
/// - Foundational Mathematics of Deep Learning (Linear Algebra, Calculus, Probability, Optimization).
pub mod activations;
pub mod transformer;
pub mod sds;
pub mod self_calibration;
pub mod utils;
pub mod gaussian_splatting;
pub mod deep_learning_theory;

// Re-export transformer components for backward compatibility.
pub use transformer::attention;
pub use transformer::feed_forward;
pub use transformer::positional_encoding;
