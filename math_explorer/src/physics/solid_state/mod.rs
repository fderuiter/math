//! # Solid State Physics
//!
//! This module provides implementations for core concepts in solid state physics,
//! facilitating the transition from single-particle band theory to Many-Body Physics.
//!
//! It covers:
//! 1. Second Quantization (Operators and Fock States)
//! 2. Screening (Thomas-Fermi and Yukawa)
//! 3. Lattice Dynamics (Phonons)
//! 4. Magnetism (Heisenberg Model)
//! 5. Superconductivity (BCS)
//! 6. Interactions (Electron-Phonon)
//! 7. Types and Errors (Strong types and error handling)

pub mod bcs;
pub mod error;
pub mod interactions;
pub mod magnetism;
pub mod phonons;
pub mod screening;
pub mod second_quantization;
pub mod types;
