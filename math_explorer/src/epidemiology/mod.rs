//! Epidemiology module for modeling disease spread.
//!
//! This module covers:
//! 1. Deterministic Compartmental Models (SIR, SEIR).
//! 2. Analytical solutions (Final Size).
//! 3. Matrix Algebra for R0 (Next Generation Matrix).
//! 4. Network Epidemiology (Heterogeneity).
//! 5. Stochastic Dynamics (Extinction, Gillespie).
//!
//! # Mathematical Background
//!
//! The **Threshold Theorem** states that an epidemic occurs if and only if the basic reproduction
//! number $R_0 > 1$.
//!
//! $R_0$ is defined as the expected number of secondary infections produced by a single infected
//! individual in a completely susceptible population.
//!
//! ## Compartmental Models
//!
//! We model the population flow between mutually exclusive compartments.
//!
//! ```mermaid
//! graph LR
//!     S((Susceptible)) -->|βSI/N| I((Infectious))
//!     I -->|γI| R((Recovered))
//!
//!     style S fill:#eee,stroke:#333
//!     style I fill:#f99,stroke:#333
//!     style R fill:#9f9,stroke:#333
//! ```
//!
//! ## Example: SIR Simulation
//!
//! ```rust
//! use math_explorer::epidemiology::compartmental::{SIRModel, basic_reproduction_number};
//!
//! // 1. Setup parameters
//! let N = 1000.0;     // Total population
//! let I0 = 1.0;       // Patient Zero
//! let beta = 0.4;     // Infection rate
//! let gamma = 0.1;    // Recovery rate (10 days duration)
//!
//! // 2. Check theoretical threshold
//! let r0 = basic_reproduction_number(beta, gamma);
//! assert_eq!(r0, 4.0); // An epidemic will occur
//!
//! // 3. Initialize Model
//! let mut model = SIRModel::new(N, I0, beta, gamma);
//!
//! // 4. Simulate
//! for _day in 0..100 {
//!     model.step(1.0); // Step forward by 1 day
//! }
//!
//! // The epidemic should have peaked and burned out
//! println!("Final Infected: {:.2}", model.state.i);
//! println!("Final Recovered: {:.2}", model.state.r);
//! ```

pub mod compartmental;
pub mod analytics;
pub mod matrix_dynamics;
pub mod networks;
pub mod stochastic;
