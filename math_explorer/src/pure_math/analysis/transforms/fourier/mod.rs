//! Fourier Analysis and Transforms.
//!
//! This module provides both continuous Fourier Analysis (Series and Transform via integration)
//! and discrete Fast Fourier Transform (FFT) implementations.

pub mod continuous;
pub mod discrete;
pub mod traits;

// Re-export continuous functions for backward compatibility.
pub use continuous::*;

// Re-export discrete transform and traits for convenience.
pub use discrete::FastFourierTransform;
pub use traits::{SpectralTransform, TransformError};
