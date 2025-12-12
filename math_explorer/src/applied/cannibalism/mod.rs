//! # Cannibalism
//!
//! Mathematical models of cannibalism.

pub mod death_rate;
pub mod juvenile_adult_dynamics;
pub mod mckendrick_von_foerster;
pub mod two_dimensional_ode;

pub use death_rate::*;
pub use juvenile_adult_dynamics::*;
pub use mckendrick_von_foerster::*;
pub use two_dimensional_ode::*;
