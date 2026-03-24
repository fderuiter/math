//! # Engineering Utilities
//!
//! Practical formulas and calculations for hardware engineering and reliability analysis.
//!
//! This module collects utility functions that don't fit into the larger simulation frameworks
//! but are essential for "back-of-the-napkin" estimation in embedded systems and manufacturing.

pub mod cnc;
pub mod error;
pub mod uart;

pub use cnc::*;
pub use error::*;
pub use uart::*;
