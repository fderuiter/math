//! # Cannibalism (Theoretical)
//!
//! Mathematical models of intraspecific predation, focusing on population dynamics.
//!
//! > **Note:** This module primarily contains **placeholder implementations** and theoretical
//! > structures for solving the **McKendrick-von Foerster** partial differential equations.
//! > It serves as a structural template for future numerical solvers.
//!
//! ## Theoretical Basis
//!
//! The core model relies on the McKendrick-von Foerster equation:
//!
//! $$ \frac{\partial n}{\partial t} + \frac{\partial n}{\partial a} = -\mu(t, a) n(t, a) $$
//!
//! Where:
//! - $n(t, a)$ is the population density of age $a$ at time $t$.
//! - $\mu(t, a)$ is the mortality rate (which includes cannibalism terms).
//!
//! ## Submodules
//!
//! - `mckendrick_von_foerster`: Core PDE definitions.
//! - `juvenile_adult_dynamics`: Age-structured interaction logic.
//! - `two_dimensional_ode`: Simplified dynamics.

pub mod death_rate;
pub mod juvenile_adult_dynamics;
pub mod mckendrick_von_foerster;
pub mod two_dimensional_ode;

pub use death_rate::*;
pub use juvenile_adult_dynamics::*;
pub use mckendrick_von_foerster::*;
pub use two_dimensional_ode::*;
