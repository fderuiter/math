//! # Epidemiology
//!
//! Tools for modeling the spread of infectious diseases using deterministic and stochastic methods.
//!
//! ## The SIR Model
//!
//! The most fundamental compartmental model divides the population into three classes:
//!
//! ```mermaid
//! graph LR
//!     S(Susceptible) -->|Infection (beta)| I(Infectious)
//!     I -->|Recovery (gamma)| R(Recovered)
//!
//!     style I fill:#f96,stroke:#333,stroke-width:2px
//! ```
//!
//! *   **Susceptible ($S$)**: Individuals who can catch the disease.
//! *   **Infectious ($I$)**: Individuals who have the disease and can spread it.
//! *   **Recovered ($R$)**: Individuals who have recovered and are immune.
//!
//! ## 🚀 Quick Start: Simulating an Outbreak
//!
//! ```rust
//! use math_explorer::epidemiology::SIRModel;
//!
//! fn main() {
//!     // 1. Define Initial Conditions
//!     let total_pop = 1000.0;
//!     let initial_infected = 1.0;
//!
//!     // 2. Configure Model
//!     // Beta (Infection Rate) = 0.3
//!     // Gamma (Recovery Rate) = 0.1 (Avg recovery time = 10 days)
//!     // R0 = Beta / Gamma = 3.0 (Epidemic threshold > 1)
//!     let mut model = SIRModel::new(total_pop, initial_infected, 0.3, 0.1);
//!
//!     // 3. Run Simulation
//!     // Simulate for 100 days
//!     for day in 0..100 {
//!         // println!("Day {}: S={:.1} I={:.1} R={:.1}", day, model.state.s, model.state.i, model.state.r);
//!         model.step(1.0); // Step forward by 1 day
//!     }
//!
//!     // 4. Analyze Results
//!     // By day 100, the epidemic should have burned through most of the population.
//!     println!("Final Recovered: {:.1}", model.state.r);
//!     assert!(model.state.r > 900.0);
//! }
//! ```
//!
//! ## Mathematical Background
//!
//! The **Threshold Theorem** states that an epidemic occurs if and only if the basic reproduction
//! number $R_0 > 1$.
//!
//! $R_0$ is defined as the expected number of secondary infections produced by a single infected
//! individual in a completely susceptible population.

pub mod analytics;
pub mod compartmental;
pub mod error;
pub mod matrix_dynamics;
pub mod networks;
pub mod stochastic;

pub use compartmental::{SEIRModel, SEIRState, SIRModel, SIRState};
pub use error::EpidemiologyError;
