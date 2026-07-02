//! Solvers for Fluid Dynamics.
//!
//! This module adapts fluid dynamics equations (PDEs) into ODE systems that can be
//! solved using standard integrators like Euler or Runge-Kutta.

use super::conservation::MomentumEquation;
use super::types::{FlowState, FluidProperties, SpatialGradients};
use nalgebra::Vector3;
use pure_math::pure_math::analysis::ode::{OdeSystem, TimeStepper};

/// A Lagrangian particle in a flow field.
///
/// Tracks the state of a fluid element as it moves through space and time.
/// The `OdeSystem` implementation computes the acceleration (change in velocity)
/// based on the provided momentum equation and spatial gradients.
///
/// **Note:** This is a simplified model where spatial gradients are provided externally
/// or assumed constant during the time step. In a full simulation, gradients would be
/// re-evaluated at each step based on the particle's new position relative to neighbors.
pub struct FluidParticleSystem<'a, M: MomentumEquation> {
    /// Fluid properties (density, viscosity).
    pub properties: &'a FluidProperties,
    /// The momentum equation strategy (Navier-Stokes, Euler).
    pub momentum_equation: M,
    /// Spatial gradients at the particle's location.
    pub gradients: SpatialGradients,
    /// External body force acceleration (e.g., gravity).
    pub body_force: Vector3<f64>,
    /// Current state.
    pub state: FlowState,
}

impl<'a, M: MomentumEquation> FluidParticleSystem<'a, M> {
    /// Creates a new fluid particle system.
    #[verified_engine::verified]
    pub fn new(
        properties: &'a FluidProperties,
        momentum_equation: M,
        gradients: SpatialGradients,
        body_force: Vector3<f64>,
        initial_state: FlowState,
    ) -> Self {
        Self {
            properties,
            momentum_equation,
            gradients,
            body_force,
            state: initial_state,
        }
    }
}

impl<'a, M: MomentumEquation> OdeSystem<FlowState> for FluidParticleSystem<'a, M> {
    #[verified_engine::verified]
    fn derivative(&self, _t: f64, state: &FlowState) -> FlowState {
        // Compute acceleration d(velocity)/dt
        let acceleration = self.momentum_equation.acceleration(
            self.properties,
            state,
            &self.gradients,
            self.body_force,
        );

        // Pressure evolution is not modeled here (requires Poisson solver).
        // We return 0 for pressure change, treating it as constant for the ODE step.
        FlowState {
            velocity: acceleration,
            pressure: 0.0,
        }
    }
}

impl<'a, M: MomentumEquation> TimeStepper<FlowState> for FluidParticleSystem<'a, M> {
    #[verified_engine::verified]
    fn get_state(&self) -> &FlowState {
        &self.state
    }

    #[verified_engine::verified]
    fn get_state_mut(&mut self) -> &mut FlowState {
        &mut self.state
    }

    #[verified_engine::verified]
    fn step(&mut self, dt: f64) {
        use pure_math::pure_math::analysis::ode::RungeKutta4;
        use pure_math::pure_math::analysis::ode::SolverExt;
        let mut solver = RungeKutta4::new(self.get_state());
        let new_state = solver.solve(self, 0.0, self.get_state(), dt);
        *self.get_state_mut() = new_state;
    }
}
