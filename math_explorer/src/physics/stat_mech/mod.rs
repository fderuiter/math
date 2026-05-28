//! Statistical Mechanics module.
//!
//! This module serves as the mathematical bridge between quantum/classical micro-physics
//! and macroscopic thermodynamics, covering Ensemble Theory, Quantum Statistics,
//! Phase Transitions, and Non-Equilibrium dynamics.

/// Boltzmann Constant in J/K.
pub const KB: f64 = 1.380649e-23;

pub mod dynamics;
pub mod ensembles;
pub mod error;
pub mod ising;
pub mod quantum_stats;

// [cite:graph_parameters_rust]
