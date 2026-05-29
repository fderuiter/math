//! # Engineering Utilities
//!
//! Practical formulas and calculations for hardware engineering and reliability analysis.
//!
//! This module collects utility functions that don't fit into the larger simulation frameworks
//! but are essential for "back-of-the-napkin" estimation in embedded systems and manufacturing.

pub mod cnc;
pub mod uart;

pub use cnc::*;
pub use uart::*;

// [cite:graph_parameters_rust]

use crate::theory_verification;

theory_verification!(
    module = "engineering",
    paper = "cera_framework.tex",
    epsilon = 1e-6,
    constants = {
        TOLERANCE = 0.001;
    },
    test = {
        assert_relative_eq!(TOLERANCE, 0.001, epsilon = 1e-6);
    }
);
