//! # Solid State Physics
//!
//! This module provides implementations for core concepts in solid state physics,
//! facilitating the transition from single-particle band theory to Many-Body Physics.
//!
//! <div class="warning">
//!
//! **Mermaid Diagram: BCS Superconductivity**
//!
//! ```mermaid
//! graph TD
//!     Lattice[Crystal Lattice] -->|Vibration| Phonons[Phonons]
//!     Electrons[Electrons] -->|Interaction| Coupling[Electron-Phonon Coupling]
//!     Phonons -->|Mediate| Coupling
//!     Coupling -->|Forms| Cooper[Cooper Pairs]
//!     Cooper -->|Condensate| SC[Superconductivity]
//!     SC -->|Gap| EnergyGap[Energy Gap \Delta]
//!
//!     style Cooper fill:#aaffaa,stroke:#333
//!     style SC fill:#aaffaa,stroke:#333
//! ```
//! </div>
//!
//! ## ⚡ Quick Start: BCS Gap Equation
//!
//! Calculate the superconducting energy gap ($\Delta$) resulting from attractive electron-phonon interactions.
//!
//! ```rust
//! use math_explorer::physics::solid_state::bcs::solve_gap_equation;
//!
//! fn main() {
//!     // 1. Define Material Properties (e.g., Aluminum-like parameters)
//!     // Debye energy (cutoff frequency) in arbitrary energy units (meV)
//!     let debye_energy = 10.0;
//!     // Interaction strength V * N(0)
//!     // If this is too small, no gap opens (Delta -> 0).
//!     let v_potential = 0.3;
//!
//!     // 2. Discretize the Density of States near the Fermi Level
//!     // We create a band of electron states ranging from -10 to +10 meV
//!     let energies: Vec<f64> = (0..200)
//!         .map(|i| (i as f64 - 100.0) * 0.1)
//!         .collect();
//!
//!     // 3. Solve the Self-Consistent Gap Equation
//!     // $\Delta = - \sum V \frac{\Delta}{2E}$
//!     let iterations = 100;
//!     let delta = solve_gap_equation(&energies, v_potential, debye_energy, iterations)
//!         .expect("Failed to converge");
//!
//!     println!("Superconducting Gap Δ: {:.4} meV", delta);
//!
//!     // 4. Verification
//!     // For finite V, we expect a non-zero gap
//!     assert!(delta > 0.0);
//!
//!     // Analytical BCS approximation: Delta ~ 2 * Debye * exp(-1 / (N(0)V))
//!     // Here we just check it exists.
//! }
//! ```
//!
//! ## Modules
//!
//! 1. **Second Quantization**: `second_quantization` - Operators and Fock States.
//! 2. **Screening**: `screening` - Thomas-Fermi and Yukawa potentials.
//! 3. **Lattice Dynamics**: `phonons` - Dispersion relations.
//! 4. **Magnetism**: `magnetism` - Heisenberg Model.
//! 5. **Superconductivity**: `bcs` - Cooper pairing and Gap equation.
//! 6. **Interactions**: `interactions` - Electron-Phonon coupling.

pub mod bcs;
pub mod interactions;
pub mod magnetism;
pub mod phonons;
pub mod screening;
pub mod second_quantization;
