//! # Standard Model of Particle Physics
//!
//! This module implements the mathematical formalism of the Standard Model,
//! covering the Gauge Principle, Spontaneous Symmetry Breaking, Flavor Physics,
//! Quantum Chromodynamics (QCD), and Neutrino Oscillations.
//!
//! The Standard Model is a quantum field theory based on the gauge group
//! SU(3)_C x SU(2)_L x U(1)_Y, describing the strong, weak, and electromagnetic
//! interactions.

pub mod error;
pub mod flavor;
pub mod gauge;
pub mod higgs;
pub mod qcd;

pub use error::StandardModelError;
pub mod neutrinos;
