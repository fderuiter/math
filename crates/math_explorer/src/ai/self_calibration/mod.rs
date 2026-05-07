//! Self-calibration module for AI models.
//!
//! This module contains functionality for:
//! - Defining response types.
//! - Calculating soft self-consistency scores.
//! - Temperature scaling based on answer entropy.
//! - Training via KL divergence loss.

pub mod scoring;
pub mod temperature;
pub mod training;
pub mod types;
