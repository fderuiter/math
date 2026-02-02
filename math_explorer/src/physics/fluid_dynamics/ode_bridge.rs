//! Bridge between Fluid Dynamics and ODE Solvers.
//!
//! This module adapts the `MomentumEquation` strategies to the `OdeSystem` trait,
//! allowing standard integrators (like Runge-Kutta) to be used for time-stepping
//! fluid velocities.

use crate::physics::fluid_dynamics::conservation::MomentumEquation;
use crate::physics::fluid_dynamics::types::{FlowState, FluidProperties, SpatialGradients};
use crate::pure_math::analysis::ode::OdeSystem;
use nalgebra::Vector3;

/// A bridge to use explicit ODE solvers (like Runge-Kutta) for Fluid Dynamics.
///
/// This struct treats the velocity at a single point as the state to be integrated.
///
/// # Assumptions
/// * Spatial gradients ($\nabla \mathbf{u}, \nabla p, \nabla^2 \mathbf{u}$) are provided externally.
/// * Gradients are treated as **constant** for the duration of the integration step (Split Operator / Explicit approach).
/// * Pressure is not evolved by this solver (use a Poisson solver for that).
pub struct PointFlowSolver<'a> {
    /// Fluid physical properties (density, viscosity).
    pub properties: &'a FluidProperties,
    /// Spatial gradients at the current location.
    pub gradients: &'a SpatialGradients,
    /// The momentum equation strategy (Navier-Stokes, Euler).
    pub strategy: &'a dyn MomentumEquation,
    /// External body forces (e.g., gravity).
    pub body_force: Vector3<f64>,
}

impl<'a> OdeSystem<Vector3<f64>> for PointFlowSolver<'a> {
    fn derivative(&self, _t: f64, state: &Vector3<f64>) -> Vector3<f64> {
        // Construct a temporary FlowState from the current velocity guess.
        // Pressure is set to 0.0 because the acceleration depends on pressure GRADIENT,
        // which is stored in `self.gradients`. Absolute pressure doesn't affect momentum
        // change in incompressible flow (only gradient does).
        let flow_state = FlowState::new(*state, 0.0);

        self.strategy.acceleration(
            self.properties,
            &flow_state,
            self.gradients,
            self.body_force,
        )
    }
}
