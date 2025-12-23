//! # Score Distillation Sampling (SDS)
//!
//! This module implements the **Score Distillation Sampling** (SDS) algorithm, typically used for
//! **Text-to-3D** generation.
//!
//! ## 📖 Theory
//!
//! SDS bridges the gap between 2D diffusion models (like Stable Diffusion) and 3D representations (like NeRFs).
//! Instead of training on 3D data (which is scarce), we optimize a 3D representation such that its
//! 2D renderings "look like" valid images according to a frozen 2D diffusion model.
//!
//! ## 🔄 The Pipeline
//!
//! 1.  **Render**: A differentiable renderer (NeRF) produces an image from a random view.
//! 2.  **Noise**: We add noise to this image.
//! 3.  **Denoise**: The frozen 2D diffusion model predicts the noise (the "score").
//! 4.  **Gradient**: The difference between added and predicted noise becomes the gradient.
//! 5.  **Update**: This gradient is backpropagated through the renderer to update the 3D model.
//!
//! ## 🧩 Modules
//!
//! *   [`rendering`]: A differentiable NeRF renderer (Ray generation -> Color).
//! *   [`diffusion`]: Simulates the noise injection process.
//! *   [`score`]: Interactions with the (mock) diffusion model to get score estimates.
//! *   [`gradient`]: Calculation of the SDS gradient residuals.
//! *   [`training`]: The optimization loop.

pub mod rendering;
pub mod diffusion;
pub mod score;
pub mod gradient;
pub mod training;
