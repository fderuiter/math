//! # Statistical Mechanics
//!
//! > **"Ludwig Boltzmann, who spent much of his life studying statistical mechanics, died in 1906, by his own hand. Paul Ehrenfest, carrying on the work, died similarly in 1933. Now it is our turn."** — David L. Goodstein
//!
//! This module bridges the gap between the microscopic world of quantum mechanics and the macroscopic world of thermodynamics.
//!
//! ## The Ising Model
//!
//! The Ising model is the simplest model of a phase transition. It describes ferromagnetism using discrete spins on a lattice.
//!
//! ```mermaid
//! graph TD
//!     subgraph "Microscopic State"
//!     Spins[Spin Lattice (+1/-1)]
//!     Interaction[Neighbor Interaction J]
//!     Temp[Temperature T]
//!     end
//!
//!     subgraph "Metropolis Dynamics"
//!     Flip{Flip Spin?}
//!     Energy[Delta E]
//!     Prob[Boltzmann Probability]
//!     end
//!
//!     subgraph "Macroscopic Phase"
//!     Order[Ferromagnetic (Ordered)]
//!     Critical[Critical Point Tc]
//!     Disorder[Paramagnetic (Disordered)]
//!     end
//!
//!     Spins --> Interaction
//!     Interaction --> Flip
//!     Temp --> Prob
//!
//!     Flip -->|Delta E < 0| Update
//!     Flip -->|Random < Prob| Update
//!
//!     Temp --> Critical
//!     Critical -->|T < Tc| Order
//!     Critical -->|T > Tc| Disorder
//! ```
//!
//! ## 🚀 Quick Start: Simulating Ferromagnetism
//!
//! Simulate a 2D Ising model and observe the magnetization.
//!
//! ```rust
//! use math_explorer::physics::stat_mech::ising::SpinLattice;
//! use math_explorer::physics::stat_mech::KB;
//!
//! // 1. Setup Lattice (50x50 spins)
//! let width = 50;
//! let height = 50;
//! let mut lattice = SpinLattice::new(width, height);
//!
//! // 2. Define Physics Parameters
//! let j_coupling = 1.0; // Interaction strength
//! let h_field = 0.0;    // No external magnetic field
//!
//! // 3. Set Temperature (T < Tc implies Order)
//! // Tc approx 2.269 * J / KB
//! let temp = 1.5 * j_coupling / KB;
//!
//! // 4. Evolve System (Monte Carlo Simulation)
//! // 1000 steps per spin (MCS)
//! let steps = 1000 * width * height;
//! lattice.evolve(steps, temp, j_coupling, h_field);
//!
//! // 5. Measure Magnetization
//! let m = lattice.magnetization();
//! println!("Final Magnetization: {}", m);
//! ```
//!
//! ## Modules
//!
//! - **[`ising`]**: 2D Ising Model simulation using Metropolis-Hastings.
//! - **[`ensembles`]**: Canonical and Grand Canonical ensemble properties.
//! - **[`dynamics`]**: Time-evolution and equilibration logic.

/// Boltzmann Constant in J/K.
pub const KB: f64 = 1.380649e-23;

pub mod dynamics;
pub mod ensembles;
pub mod error;
pub mod ising;
pub mod quantum_stats;
