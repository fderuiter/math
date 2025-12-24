//! # Score Distillation Sampling (SDS)
//!
//! This module implements the NeRF-Diffusion pipeline for 3D generation.
//! It combines Neural Radiance Fields (NeRF) with pre-trained 2D Diffusion Models
//! to distill 3D geometry from 2D priors.
//!
//! ## Pipeline Architecture
//!
//! The SDS process is a 5-stage loop that optimizes the 3D representation.
//!
//! ```mermaid
//! graph TD
//!     subgraph "Forward Pass (Rendering)"
//!     Camera[Camera Pose] --> Rays[Ray Generation]
//!     Rays --> NeRF[Differentiable NeRF]
//!     NeRF --> Image[Rendered 2D Image]
//!     end
//!
//!     subgraph "Score Distillation"
//!     Image --> Noise[Add Noise t]
//!     Noise --> UNet[Diffusion U-Net]
//!     UNet --> Score[Predict Noise / Score]
//!     Score --> Grad[Calculate Gradient]
//!     end
//!
//!     subgraph "Backward Pass (Optimization)"
//!     Grad --> Backprop[Backpropagate to NeRF]
//!     Backprop --> Update[Update Weights/Grid]
//!     end
//! ```
//!
//! ## Submodules
//!
//! - **rendering**: Differentiable volume rendering (ray marching, density accumulation).
//! - **diffusion**: Noise injection and timestep management.
//! - **score**: Classifier-Free Guidance (CFG) and score estimation.
//! - **gradient**: Calculation of SDS gradients (residuals).
//! - **training**: Optimization loop (Adam optimizer).

pub mod rendering;
pub mod diffusion;
pub mod score;
pub mod gradient;
pub mod training;
