//! High Energy Physics module.
//!
//! This module implements key concepts in high-energy astrophysics including:
//! - Special Relativity and Four-Vectors
//! - Radiative Processes (Synchrotron, Inverse Compton)
//! - Relativistic Fluid Dynamics
//! - General Relativity (Schwarzschild metric)
//! - Statistics (Li & Ma significance)

pub mod constants;
pub mod error;
pub mod fluid_dynamics;
pub mod general_relativity;
pub mod observer;
pub mod radiation;
pub mod statistics;

// Re-export constants to match original API
pub use constants::{C, G, SIGMA_T, SOLAR_MASS};

// Re-export SchwarzschildBlackHole to match original API
pub use error::HighEnergyError;
pub use general_relativity::SchwarzschildBlackHole;
