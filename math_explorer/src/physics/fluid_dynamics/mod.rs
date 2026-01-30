//! # Fluid Dynamics
//!
//! > **"Panta Rhei" (Everything Flows)** — Heraclitus
//!
//! This module implements the fundamental laws governing the motion of fluids, from
//! the microscopic conservation laws to macroscopic flow regimes.
//!
//! ## The Physics of Flow
//!
//! Fluid dynamics is built upon three conservation laws which form the Navier-Stokes equations:
//!
//! ```mermaid
//! graph TD
//!     subgraph "Conservation Laws"
//!     Mass[Conservation of Mass]
//!     Momentum[Conservation of Momentum]
//!     Energy[Conservation of Energy]
//!     end
//!
//!     subgraph "Governing Equations"
//!     NSE[Navier-Stokes Equations]
//!     Euler[Euler Equations]
//!     end
//!
//!     subgraph "Flow Regimes"
//!     Re{Reynolds Number}
//!     Laminar[Laminar Flow]
//!     Turbulent[Turbulence]
//!     end
//!
//!     Mass --> NSE
//!     Momentum --> NSE
//!     Energy --> NSE
//!
//!     NSE -->|Viscosity = 0| Euler
//!     NSE --> Re
//!
//!     Re -->|< 2000| Laminar
//!     Re -->|> 4000| Turbulent
//! ```
//!
//! ## 🚀 Quick Start: Flow Regime Classification
//!
//! Determine if a flow is laminar or turbulent based on its Reynolds number.
//!
//! ```rust
//! use math_explorer::physics::fluid_dynamics::types::FluidProperties;
//! use math_explorer::physics::fluid_dynamics::regimes::{PipeFlowClassifier, FlowClassifier};
//!
//! // 1. Define Fluid Properties (Water at 20°C)
//! let water = FluidProperties::water();
//!
//! // 2. Define Flow Conditions
//! let velocity = 2.0;       // Flow velocity (m/s)
//! let diameter = 0.05;      // Pipe diameter (5 cm)
//!
//! // 3. Calculate Reynolds Number: Re = (rho * v * L) / mu
//! let re = (water.density * velocity * diameter) / water.dynamic_viscosity;
//! println!("Reynolds Number: {:.2}", re);
//!
//! // 4. Classify Regime
//! let classifier = PipeFlowClassifier;
//! let regime = classifier.classify(re);
//!
//! println!("Flow Regime: {:?}", regime);
//! ```
//!
//! ## Modules
//!
//! - **[`conservation`]**: Implementation of Mass and Momentum conservation (Navier-Stokes).
//! - **[`regimes`]**: Strategy pattern for classifying flow (Laminar vs. Turbulent).
//! - **[`turbulence`]**: RANS (Reynolds-Averaged Navier-Stokes) modeling components.
//! - **[`types`]**: Core structs like `FluidProperties` and `FlowState`.

pub mod analysis;
pub mod conservation;
pub mod regimes;
pub mod turbulence;
pub mod types;

#[cfg(test)]
mod tests;
