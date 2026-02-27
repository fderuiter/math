//! Ordinary Differential Equation (ODE) solvers.
//!
//! This module provides a generic `OdeSystem` trait for defining ODEs and solvers
//! like `RungeKutta4` to integrate them numerically.
//!
//! The design relies on the `VectorOperations` trait to allow the solvers to work
//! with any vector-like type (e.g., `Vec<f64>`, `nalgebra::Vector3<f64>`, etc.),
//! avoiding heap allocations when fixed-size arrays are sufficient.

pub mod model;
pub mod solvers;
pub mod state;
pub mod stepper;
pub mod traits;

pub use model::OdeModel;
pub use solvers::{Euler, RungeKutta4};
pub use state::{ArrayState, VecState};
pub use stepper::TimeStepper;
pub use traits::{OdeSystem, Solver, VectorOperations};
