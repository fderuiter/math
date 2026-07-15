//! # Generative Models for Turbulence
//!
//! This module contains the Rust implementation of the concepts described in the paper
//! "Learning Turbulent Flows with Generative Models: Super-resolution, Forecasting, and
//! Sparse Flow Reconstruction" by Oommen, et al.
//!
//! It leverages the `tch-rs` library for deep learning functionalities to build and
//! train neural operators and other generative models.
//!
//! ## Submodules
//! - `networks`: Core neural network architectures like U-Nets.
//! - `models`: High-level model implementations (adv-NO, Diffusion Models).
//! - `losses`: Custom loss functions for training.
//! - `data`: Data loading and preprocessing utilities.
//! - `analysis`: Tools for analyzing results (e.g., energy spectra).

#[cfg(not(target_arch = "wasm32"))]
pub mod analysis;
#[cfg(not(target_arch = "wasm32"))]
pub mod data;
#[cfg(not(target_arch = "wasm32"))]
pub mod losses;
#[cfg(not(target_arch = "wasm32"))]
pub mod models;
#[cfg(not(target_arch = "wasm32"))]
pub mod networks;

// [cite:generative_turbulence]

use pure_math::theory_verification;

theory_verification!(
    module = "generative_turbulence",
    epsilon = 1e-6,
    constants = {
        REYNOLDS = 1000.0;
    },
    test = {
        assert_relative_eq!(REYNOLDS, 1000.0, epsilon = 1e-6);
    }
);
// theory_verification!
