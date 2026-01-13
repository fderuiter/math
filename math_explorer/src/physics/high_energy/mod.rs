//! High Energy Physics module.
//!
//! This module implements key concepts in high-energy astrophysics including:
//! - Special Relativity and Four-Vectors
//! - Radiative Processes (Synchrotron, Inverse Compton)
//! - Relativistic Fluid Dynamics
//! - General Relativity (Schwarzschild metric)
//! - Statistics (Li & Ma significance)

pub mod constants;
pub mod observer;
pub mod radiation;
pub mod fluid_dynamics;
pub mod general_relativity;
pub mod statistics;
pub mod error;

// Re-export constants to match original API
pub use constants::{C, G, SOLAR_MASS, SIGMA_T};

// Re-export SchwarzschildBlackHole to match original API
pub use general_relativity::SchwarzschildBlackHole;

// Re-export Error type
pub use error::HighEnergyError;
