//! Epidemiology module for modeling disease spread.
//!
//! This module covers:
//! 1. Deterministic Compartmental Models (SIR, SEIR).
//! 2. Analytical solutions (Final Size).
//! 3. Matrix Algebra for R0 (Next Generation Matrix).
//! 4. Network Epidemiology (Heterogeneity).
//! 5. Stochastic Dynamics (Extinction, Gillespie).
//!
//! # Mathematical Background
//!
//! The **Threshold Theorem** states that an epidemic occurs if and only if the basic reproduction
//! number $R_0 > 1$.
//!
//! $R_0$ is defined as the expected number of secondary infections produced by a single infected
//! individual in a completely susceptible population.

pub mod analytics;
pub mod compartmental;
pub mod error;
pub mod matrix_dynamics;
pub mod networks;
pub mod stochastic;

pub use error::EpidemiologyError;
