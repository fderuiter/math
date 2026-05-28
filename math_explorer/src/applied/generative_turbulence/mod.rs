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
//! - `training`: Training loops and optimization logic.
//! - `data`: Data loading and preprocessing utilities.
//! - `analysis`: Tools for analyzing results (e.g., energy spectra).

pub mod analysis;
pub mod data;
pub mod losses;
pub mod models;
pub mod networks;
pub mod training;

// [cite:generative_turbulence]
