//! # Artificial Intelligence
//!
//! Primitives and full systems for Modern AI, ranging from foundational Deep Learning theory
//! to state-of-the-art 3D reconstruction and generative models.
//!
//! ## Modules
//!
//! - **`transformer`**: The backbone of LLMs. Includes `Attention`, `Encoder`, `Decoder`.
//! - **`gaussian_splatting`**: Real-time 3D scene representation using rasterized Gaussians.
//! - **`sds`**: Score Distillation Sampling (NeRF-Diffusion hybrid) for 3D generation.
//! - **`deep_learning_theory`**: Educational implementations of Backprop, SGD, and Softmax from scratch.
//! - **`self_calibration`**: Logic for AI agents to self-assess and tune hyperparameters.
//! - **`activations`**: Common activation functions (ReLU, GeLU, Softmax).

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
