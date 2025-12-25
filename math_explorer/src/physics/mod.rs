//! # Physics
//!
//! This module implements physical laws and simulations across scales, from the subatomic
//! to the cosmological.
//!
//! ## Domains
//!
//! ### ⚛️ Quantum & Standard Model
//! - **`quantum`**: Core quantum mechanics (Clebsch-Gordan, Schrödinger evolution).
//! - **`standard_model`**: Gauge theories, Higgs Mechanism, and QCD.
//! - **`nuclear`**: Liquid Drop Model, Shell Model, and Decay kinetics.
//! - **`stat_mech`**: Ensembles (Canonical/Grand Canonical) and Quantum Statistics.
//!
//! ### 🌌 Astrophysics & Cosmology
//! - **`astrophysics`**: Galactic properties and stellar dynamics.
//! - **`high_energy`**: Relativistic physics, Black Holes (Schwarzschild), and Radiation.
//!
//! ### 🧪 Material Science
//! - **`solid_state`**: Many-Body Physics, Phonons, and Superconductivity (BCS).
//! - **`fluid_dynamics`**: Navier-Stokes terms, Turbulence modeling, and Conservation laws.
//!
//! ### 🏥 Medical Physics
//! - **`medical`**: Radiation Therapy Treatment Planning (Dose Calculation, Optimization).
//! - **`mri`**: Magnetic Resonance Imaging simulation (Bloch Equations, Signal Reconstruction).
//!
//! ### 🌀 Chaos & Complexity
//! - **`chaos`**: Deterministic chaos, Strange Attractors (Lorenz), and Fractal Dimensions.

/// Astrophysics (Galaxies, Stellar Dynamics).
pub mod astrophysics;

/// Chaos Theory (Lorenz System, Lyapunov Exponents, Fractals).
pub mod chaos;

/// Fluid Dynamics (Conservation Laws, Turbulence).
pub mod fluid_dynamics;

/// High Energy Physics (Relativity, Black Holes, Radiation).
pub mod high_energy;

/// Medical Physics (Radiation Therapy, Dose Calculation).
pub mod medical;

/// MRI Physics (Bloch Equations, Signal Processing).
pub mod mri;

/// Nuclear Physics (Liquid Drop Model, Decay, Reactions).
pub mod nuclear;

/// Quantum Mechanics (States, Operators, Time Evolution).
pub mod quantum;

/// Solid State Physics (Phonons, BCS Theory, Magnetism).
pub mod solid_state;

/// The Standard Model (Gauge Theory, Higgs, QCD).
pub mod standard_model;

/// Statistical Mechanics (Ensembles, Ising Model, Quantum Stats).
pub mod stat_mech;
