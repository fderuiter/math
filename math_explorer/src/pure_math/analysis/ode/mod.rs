//! Ordinary Differential Equation (ODE) solvers.
//!
//! This module provides a generic `OdeSystem` trait for defining ODEs and solvers
//! like `RungeKutta4` to integrate them numerically.
//!
//! The design relies on the `VectorOperations` trait to allow the solvers to work
//! with any vector-like type (e.g., `Vec<f64>`, `nalgebra::Vector3<f64>`, etc.),
//! avoiding heap allocations when fixed-size arrays are sufficient.

pub mod traits;
pub mod state;
pub mod solvers;
pub mod stepper;

pub use traits::{OdeSystem, Solver, VectorOperations};
pub use state::{VecState, ArrayState};
pub use solvers::{Euler, RungeKutta4};
pub use stepper::TimeStepper;
