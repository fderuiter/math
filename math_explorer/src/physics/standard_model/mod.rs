//! # Standard Model of Particle Physics
//!
//! This module implements the mathematical formalism of the Standard Model,
//! covering the Gauge Principle, Spontaneous Symmetry Breaking, Flavor Physics,
//! Quantum Chromodynamics (QCD), and Neutrino Oscillations.
//!
//! The Standard Model is a quantum field theory based on the gauge group
//! SU(3)_C x SU(2)_L x U(1)_Y, describing the strong, weak, and electromagnetic
//! interactions.

pub mod gauge;
pub mod higgs;
pub mod flavor;
pub mod qcd;
pub mod neutrinos;
pub mod error;

pub use error::StandardModelError;
