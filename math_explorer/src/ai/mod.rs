//! # Artificial Intelligence
//!
//! This module provides implementations of modern AI architectures, rendering techniques,
//! and foundational theory.
//!
//! ## Domains
//!
//! ### 🧠 Architectures
//! - **`transformer`**: Full implementation of "Attention Is All You Need" (Encoder/Decoder, Multi-Head Attention).
//! - **`self_calibration`**: Logic for models to grade their own confidence (Soft Self-Consistency, Temperature Scaling).
//!
//! ### 🎨 Neural Rendering
//! - **`sds`**: Score Distillation Sampling (NeRF-Diffusion pipeline).
//! - **`gaussian_splatting`**: 3D Gaussian Splatting for real-time radiance field rendering.
//!
//! ### 📘 Theory & Primitives
//! - **`deep_learning_theory`**: Educational implementations of Backprop, Autograd, and Optimization (SGD).
//! - **`activations`**: Common activation functions (ReLU, Softmax).
//! - **`utils`**: Tensor operations and helpers.

pub mod activations;
pub mod deep_learning_theory;
pub mod gaussian_splatting;
pub mod sds;
pub mod self_calibration;
pub mod transformer;
pub mod utils;

// Re-exports for backward compatibility and ease of access
pub use transformer::{
    attention,
    feed_forward,
    positional_encoding,
};
