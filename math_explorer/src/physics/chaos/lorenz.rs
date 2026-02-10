//! Continuous Chaos (The Lorenz System)

use crate::pure_math::analysis::ode::{OdeError, OdeSystem, Solver, TimeStepper};
use nalgebra::Vector3;

/// Represents the state of the Lorenz system $(x, y, z)$.
#[derive(Debug, Clone, Copy)]
pub struct LorenzState {
    /// The 3D state vector.
    pub vec: Vector3<f64>,
}

impl LorenzState {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        LorenzState {
            vec: Vector3::new(x, y, z),
        }
    }
}

/// A builder for the `LorenzSystem`.
#[derive(Debug, Clone, Copy)]
pub struct LorenzBuilder {
    sigma: f64,
    rho: f64,
    beta: f64,
}

impl Default for LorenzBuilder {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

impl LorenzBuilder {
    /// Creates a new builder with standard chaotic constants.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the Prandtl number $\sigma$.
    pub fn sigma(mut self, val: f64) -> Self {
        self.sigma = val;
        self
    }

    /// Sets the Rayleigh number $\rho$.
    pub fn rho(mut self, val: f64) -> Self {
        self.rho = val;
        self
    }

    /// Sets the geometric factor $\beta$.
    pub fn beta(mut self, val: f64) -> Self {
        self.beta = val;
        self
    }

    /// Builds the `LorenzSystem` with the configured parameters and initial state.
    pub fn build(self, state: LorenzState) -> LorenzSystem {
        LorenzSystem {
            sigma: self.sigma,
            rho: self.rho,
            beta: self.beta,
            state,
        }
    }
}

/// The Lorenz System simulator.
///
/// The Lorenz equations are:
/// $\dot{x} = \sigma(y - x)$
/// $\dot{y} = x(\rho - z) - y$
/// $\dot{z} = xy - \beta z$
///
/// These equations originally modeled atmospheric convection but became the seminal example of deterministic chaos.
#[derive(Debug, Clone, Copy)]
pub struct LorenzSystem {
    /// The Prandtl number $\sigma$, representing the ratio of momentum diffusivity to thermal diffusivity.
    pub sigma: f64,
    /// The Rayleigh number $\rho$, representing the temperature difference driving the convection.
    pub rho: f64,
    /// The geometric factor $\beta$, related to the aspect ratio of the convection rolls.
    pub beta: f64,
    /// The current state of the system.
    pub state: LorenzState,
}

impl TimeStepper<Vector3<f64>> for LorenzSystem {
    fn get_state(&self) -> &Vector3<f64> {
        &self.state.vec
    }

    fn get_state_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.state.vec
    }
}

impl LorenzSystem {
    /// Creates a new LorenzSystem with standard chaotic constants: $\sigma=10, \rho=28, \beta=8/3$.
    ///
    /// # Deprecation Notice
    /// Prefer using `LorenzBuilder::new().build(initial_state)` for better composability.
    pub fn default_chaotic(initial_state: LorenzState) -> Self {
        LorenzBuilder::new().build(initial_state)
    }

    /// Advances the system by time `dt` using the Runge-Kutta 4 (RK4) method.
    ///
    /// This now delegates to the generic `RungeKutta4` solver via `TimeStepper`.
    pub fn step(&mut self, dt: f64) -> Result<(), OdeError> {
        <Self as TimeStepper<Vector3<f64>>>::step(self, dt)
    }

    /// Advances the system by time `dt` using a provided solver strategy.
    ///
    /// This allows the user to switch integrators (e.g., Euler, RK4) dynamically.
    pub fn step_with<S: Solver<Vector3<f64>>>(&mut self, solver: &mut S, dt: f64) -> Result<(), OdeError> {
        <Self as TimeStepper<Vector3<f64>>>::step_with(self, solver, dt)
    }
}

impl OdeSystem<Vector3<f64>> for LorenzSystem {
    /// Calculates the derivative at a given state.
    fn derivative(&self, _t: f64, state: &Vector3<f64>) -> Vector3<f64> {
        let x = state.x;
        let y = state.y;
        let z = state.z;

        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;

        Vector3::new(dx, dy, dz)
    }
}
