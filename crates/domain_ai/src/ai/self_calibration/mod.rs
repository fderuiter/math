//! Self-calibration module for AI models.
//!
//! This module contains functionality for:
//! - Defining response types.
//! - Calculating soft self-consistency scores.
//! - Temperature scaling based on answer entropy.
//! - Training via KL divergence loss.

#[allow(missing_docs)]
pub mod scoring;
#[allow(missing_docs)]
pub mod temperature;
#[allow(missing_docs)]
pub mod training;
pub mod types;

// [cite:self_calibration_paper]
