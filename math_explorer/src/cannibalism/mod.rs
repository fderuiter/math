//! # Cannibalism
//!
//! Mathematical models of cannibalism.

pub mod mckendrick_von_foerster;
pub mod death_rate;
pub mod juvenile_adult_dynamics;
pub mod two_dimensional_ode;

pub use mckendrick_von_foerster::*;
pub use death_rate::*;
pub use juvenile_adult_dynamics::*;
pub use two_dimensional_ode::*;
