//! # Computational Fluid Dynamics (CFD)
//!
//! > **"Water is the driving force of all nature."** — Leonardo da Vinci
//!
//! This module provides the foundational equations for simulating fluid motion, from
//! laminar pipe flow to turbulent chaos. We implement the **Navier-Stokes** and **Euler**
//! equations using a rigorous vector calculus approach.
//!
//! ## 🌊 The Ecosystem
//!
//! We decompose fluid dynamics into State, Properties, and Conservation Laws:
//!
//! ```mermaid
//! graph TD
//!     subgraph "Data Structures"
//!     State[FlowState<br/>(Velocity, Pressure)]
//!     Props[FluidProperties<br/>(Density, Viscosity)]
//!     end
//!
//!     subgraph "Analysis"
//!     Re[Reynolds Number]
//!     Regime{Flow Regime<br/>Laminar/Turbulent}
//!     Stress[Reynolds Stress]
//!     end
//!
//!     subgraph "Conservation Laws (PDEs)"
//!     NS[Navier-Stokes Eq<br/>Viscous Flow]
//!     Euler[Euler Eq<br/>Inviscid Flow]
//!     end
//!
//!     State --> Re
//!     Props --> Re
//!     Re --> Regime
//!     Regime -->|If Turbulent| Stress
//!
//!     State --> NS
//!     Props --> NS
//!     State --> Euler
//! ```
//!
//! ## 🚀 Quick Start: Navier-Stokes Time Step
//!
//! Calculate the acceleration of a fluid element (Water) under pressure and viscous forces.
//!
//! ```rust
//! use math_explorer::physics::fluid_dynamics::types::{FluidProperties, FlowState};
//! use math_explorer::physics::fluid_dynamics::conservation::navier_stokes_time_derivative;
//! use math_explorer::physics::fluid_dynamics::analysis::reynolds_number;
//! use nalgebra::{Vector3, Matrix3};
//!
//! // 1. Define Fluid (Water at 20°C)
//! let water = FluidProperties::water();
//!
//! // 2. Define State at a point (Moving Fast)
//! // Velocity = [10.0, 0.0, 0.0] m/s
//! // Pressure = 101325 Pa (1 atm)
//! let state = FlowState::new(Vector3::new(10.0, 0.0, 0.0), 101325.0);
//!
//! // 3. Analysis: Check Regime
//! // Characteristic Length L = 0.5m (e.g., pipe diameter)
//! let re = reynolds_number(&water, state.velocity.norm(), 0.5);
//! // Re approx 4.99 x 10^6 -> Turbulent
//!
//! // 4. Define Spatial Gradients (The "Mesh" part)
//! // In a real simulation, these come from finite difference/volume neighbors.
//!
//! // Velocity Gradient (Jacobian): Slowing down in x-direction (compression)
//! let vel_grad = Matrix3::from_diagonal(&Vector3::new(-0.1, 0.0, 0.0));
//!
//! // Pressure Gradient: High pressure ahead opposes flow (+x gradient)
//! let press_grad = Vector3::new(100.0, 0.0, 0.0);
//!
//! // Laplacian (Viscous Diffusion): Smoothing out velocity spikes
//! let laplacian = Vector3::zeros();
//!
//! // Gravity (acting down in z)
//! let gravity = Vector3::new(0.0, 0.0, -9.81);
//!
//! // 5. Solve Momentum Equation
//! // Returns du/dt (Acceleration)
//! let acceleration = navier_stokes_time_derivative(
//!     &water,
//!     &state,
//!     &vel_grad,
//!     press_grad,
//!     laplacian,
//!     gravity
//! );
//!
//! // Expect deceleration due to pressure gradient and convection
//! println!("Acceleration: {:.4} m/s²", acceleration);
//! ```

pub mod analysis;
pub mod conservation;
pub mod regimes;
pub mod turbulence;
pub mod types;

#[cfg(test)]
mod tests;
