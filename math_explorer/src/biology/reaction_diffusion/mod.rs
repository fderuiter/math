//! # Generic Reaction-Diffusion System
//!
//! This module provides a flexible framework for simulating N-species reaction-diffusion systems.
//!
//! ## Why does this exist?
//!
//! While specific implementations like `TuringSystem` are useful, biological modeling often requires
//! testing novel hypotheses. This module exists to decouple the core mechanics of a simulation
//! (the mathematical integration and spatial grids) from the domain-specific biology (the chemical reactions).
//! By enforcing the Strategy Pattern, researchers can swap solvers, dimensions (1D vs 2D), or kinetics
//! without touching the underlying engine.
//!
//! ## Architecture
//!
//! ```mermaid
//! classDiagram
//!     class ReactionDiffusionSystem {
//!         +model: ReactionDiffusionModel
//!         +state: ChemicalState
//!         +solver: Solver
//!         +step(dt)
//!     }
//!
//!     class ReactionDiffusionModel {
//!         +reaction: ReactionModel
//!         +diffusion: DiffusionModel
//!         +diffusion_coeffs: Vec~f64~
//!     }
//!
//!     class ReactionModel {
//!         <<Trait>>
//!         +reaction()
//!     }
//!
//!     class DiffusionModel {
//!         <<Trait>>
//!         +apply()
//!     }
//!
//!     ReactionDiffusionSystem o-- ReactionDiffusionModel
//!     ReactionDiffusionModel o-- ReactionModel
//!     ReactionDiffusionModel o-- DiffusionModel
//! ```
//!
//! ## Example
//!
//! Implementing a simple decay system using the generic framework:
//!
//! ```rust
//! use math_explorer::biology::reaction_diffusion::{ReactionDiffusionSystem, ReactionModel, ChemicalState};
//! use math_explorer::biology::diffusion::FiniteDifference1D;
//!
//! // 1. Define custom kinetics (e.g., simple exponential decay)
//! struct DecayKinetics { rate: f64 }
//! impl ReactionModel for DecayKinetics {
//!     fn reaction(&self, concs: &[f64], rates: &mut [f64]) {
//!         rates[0] = -self.rate * concs[0];
//!     }
//! }
//!
//! // 2. Setup the generic system (1 species, 10 grid points)
//! let kinetics = DecayKinetics { rate: 0.1 };
//! let diffusion = FiniteDifference1D::new(1.0);
//! let mut system = ReactionDiffusionSystem::new(1, 10, kinetics, diffusion, vec![0.5]);
//!
//! // 3. Initialize state and run
//! system.state.species_mut(0)[5] = 100.0; // Spike in the middle
//! system.step(0.1);
//!
//! assert!(system.state.species(0)[5] < 100.0); // Concentration decayed
//! ```

pub mod algorithms;
pub mod model;

#[cfg(test)]
mod tests;

pub use algorithms::ReactionDiffusionSystem;
pub use model::{ChemicalState, DiffusionModel, ReactionDiffusionModel, ReactionModel};
