//! # Physics
//!
//! This module implements physical laws and simulations across various scales of the universe.
//! It uses the **Hub and Spoke** documentation pattern, categorizing submodules by field.
//!
//! ## Structure
//!
//! ```mermaid
//! graph TD
//!     Physics --> Quantum[Quantum & Standard Model]
//!     Physics --> Astro[Astrophysics & Cosmology]
//!     Physics --> Materials[Material Science]
//!     Physics --> Medical[Medical Physics]
//!     Physics --> Chaos[Chaos & Dynamics]
//!
//!     Quantum --> Q[quantum]
//!     Quantum --> SM[standard_model]
//!     Quantum --> Nuc[nuclear]
//!     Quantum --> HE[high_energy]
//!
//!     Astro --> A[astrophysics]
//!
//!     Materials --> SS[solid_state]
//!     Materials --> FD[fluid_dynamics]
//!     Materials --> Stat[stat_mech]
//!
//!     Medical --> MRI[mri]
//!     Medical --> Med[medical]
//!
//!     Chaos --> C[chaos]
//! ```
//!
//! ## Submodules
//!
//! ### Quantum & Standard Model
//! * **`quantum`**: Quantum mechanics primitives (states, operators, angular momentum).
//! * **`standard_model`**: Particle physics (Higgs, Gauge Bosons, QCD).
//! * **`nuclear`**: Nuclear properties, decay modes, and reaction models.
//! * **`high_energy`**: Special and General Relativity, radiation, and fluid dynamics in high energy contexts.
//!
//! ### Astrophysics & Cosmology
//! * **`astrophysics`**: Galaxy properties and orbital mechanics.
//!
//! ### Material Science
//! * **`solid_state`**: Many-body physics, phonons, BCS theory, and magnetism.
//! * **`fluid_dynamics`**: Conservation laws, turbulence (RANS), and flow analysis.
//! * **`stat_mech`**: Statistical ensembles (Canonical) and Ising models.
//!
//! ### Medical Physics
//! * **`mri`**: Magnetic Resonance Imaging simulation (Bloch equations, sequences).
//! * **`medical`**: Radiation therapy planning, dosimetry, and calibration.
//!
//! ### Chaos
//! * **`chaos`**: Deterministic chaos, strange attractors (Lorenz), and fractals.
//!
//! ## Usage Example
//!
//! ```rust
//! // Example: Simulating the Butterfly Effect with the Lorenz System
//! use math_explorer::physics::chaos::lorenz::{LorenzSystem, LorenzState};
//!
//! // Initialize the system with the standard chaotic parameters
//! // sigma=10, rho=28, beta=8/3
//! let initial_state = LorenzState::new(1.0, 1.0, 1.0);
//! let mut system = LorenzSystem::default_chaotic(initial_state);
//!
//! // Advance the simulation
//! let dt = 0.01;
//! system.step(dt);
//!
//! println!("New state: {:?}", system.state);
//! ```

pub mod quantum;
pub mod astrophysics;
pub mod high_energy;
pub mod fluid_dynamics;
pub mod nuclear;
pub mod standard_model;
pub mod solid_state;
pub mod mri;
pub mod stat_mech;
pub mod medical;
pub mod chaos;
