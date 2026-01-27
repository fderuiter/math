//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

pub mod kinetics;
pub mod state;
pub mod system;

pub use kinetics::{ReactionKinetics, SchnakenbergKinetics};
pub use state::TuringState;
pub use system::TuringSystem;
