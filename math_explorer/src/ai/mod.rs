//! # Artificial Intelligence & Machine Learning
//!
//! This module serves as a laboratory for AI algorithms, ranging from foundational theory to modern rendering techniques.
//! It is organized into three primary categories: **Architectures**, **Rendering**, and **Theory**.
//!
//! ## 🏗️ Architectures
//!
//! *   [`transformer`]: The backbone of modern NLP. Implements the "Attention Is All You Need" paper, including Encoders, Decoders, and Multi-Head Attention.
//! *   [`self_calibration`]: An experimental framework for models to self-adjust their confidence (temperature scaling) based on validation performance.
//!
//! ## 🎨 Neural Rendering & 3D
//!
//! *   [`gaussian_splatting`]: **3D Gaussian Splatting (3DGS)**. A rasterization technique that represents scenes as 3D Gaussians for real-time rendering.
//! *   [`sds`]: **Score Distillation Sampling**. A method for generating 3D assets from 2D diffusion models (text-to-3D), often used with NeRFs.
//!
//! ## 📚 Theory & Foundations
//!
//! *   [`deep_learning_theory`]: A "from scratch" implementation of the mathematics behind deep learning.
//!     *   Builds a neural network using raw Calculus (Backpropagation), Linear Algebra (Matrix Ops), and Probability (MLE/Softmax).
//!     *   **Educational Purpose**: Read this to understand *how* PyTorch works under the hood.
//!
//! ## 🛠️ Primitives
//!
//! *   [`activations`]: Common activation functions (ReLU, Softmax).
//! *   [`utils`]: Helper traits and tensor operations.

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
