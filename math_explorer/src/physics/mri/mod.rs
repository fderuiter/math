//! MRI Physics Simulation Module
//!
//! This module provides a rigorous simulation of Magnetic Resonance Imaging (MRI) physics,
//! covering Quantum Foundations, Classical Dynamics (Bloch Equations), Spatial Encoding,
//! and Image Reconstruction.
//!
//! # Domains
//!
//! 1. **Quantum Foundations**: Proton properties, Larmor frequency, and Boltzmann statistics.
//! 2. **Classical Dynamics**: Bloch equation simulation for magnetization vectors.
//! 3. **Spatial Encoding**: Gradient fields and k-space trajectory calculations.
//! 4. **Image Reconstruction**: Signal generation and Inverse Fourier Transform.

pub mod proton;
pub mod bloch;
pub mod scanner;
pub mod reconstruction;

pub use bloch::BlochSimulator;
