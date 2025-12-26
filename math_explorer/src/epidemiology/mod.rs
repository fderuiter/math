//! # Epidemiology: Modeling Disease Spread
//!
//! This module provides a suite of tools for simulating infectious disease dynamics, ranging from
//! classical deterministic ODEs to stochastic network models.
//!
//! ## Core Models
//!
//! 1.  **Deterministic Compartmental Models (SIR/SEIR)**:
//!     Uses ordinary differential equations (ODEs) to model large populations where individuals are mixed homogeneously.
//! 2.  **Stochastic Dynamics**:
//!     Uses the Gillespie algorithm for small populations where chance events matter (e.g., extinction).
//! 3.  **Network Epidemiology**:
//!     Models spread on graphs to account for heterogeneous contact patterns (e.g., Superspreaders).
//!
//! ## The SIR Model
//!
//! The classic SIR model divides the population into three compartments:
//!
//! ```mermaid
//! graph LR
//!     S((Susceptible)) -->|Infection Rate beta| I((Infected))
//!     I -->|Recovery Rate gamma| R((Recovered))
//!
//!     style I fill:#f96,stroke:#333,stroke-width:2px,color:black
//! ```
//!
//! $$
//! \frac{dS}{dt} = -\beta S I, \quad \frac{dI}{dt} = \beta S I - \gamma I, \quad \frac{dR}{dt} = \gamma I
//! $$
//!
//! ## Example: Simulating an Outbreak
//!
//! ```rust
//! use math_explorer::epidemiology::compartmental::{SIRModel, SIRState};
//!
//! fn main() {
//!     // 1. Define initial parameters
//!     let n = 1000.0;
//!     let i0 = 10.0;
//!     let beta = 0.3;
//!     let gamma = 0.1;
//!
//!     // 2. Configure model
//!     // SIRModel::new(N, I0, Beta, Gamma)
//!     let mut model = SIRModel::new(n, i0, beta, gamma);
//!
//!     // 3. Step forward in time (Euler method default)
//!     let dt = 0.1;
//!     model.step(dt);
//!
//!     println!("New Infected Count: {:.2}", model.state.i);
//! }
//! ```
//!
//! ## Key Concepts
//!
//! - **$R_0$ (Basic Reproduction Number)**: $\beta / \gamma$. If $R_0 > 1$, the disease spreads.
//! - **Herd Immunity**: The threshold $1 - 1/R_0$ where the susceptible population is too low to sustain an epidemic.

pub mod compartmental;
pub mod analytics;
pub mod matrix_dynamics;
pub mod networks;
pub mod stochastic;
