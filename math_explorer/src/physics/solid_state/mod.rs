//! # Solid State Physics
//!
//! This module provides implementations for core concepts in solid state physics,
//! facilitating the transition from single-particle band theory to Many-Body Physics.
//!
//! It covers:
//! 1. Second Quantization (Operators and Fock States)
//! 2. Screening (Thomas-Fermi and Yukawa)
//! 3. Lattice Structures (SC, BCC, FCC)
//! 4. Lattice Dynamics (Phonons)
//! 5. Magnetism (Heisenberg Model)
//! 6. Superconductivity (BCS)
//! 7. Interactions (Electron-Phonon)
//! 8. Types and Errors (Strong types and error handling)

pub mod bcs;
pub mod error;
pub mod interactions;
pub mod lattice;
pub mod magnetism;
pub mod phonons;
pub mod screening;
pub mod second_quantization;
pub mod types;
