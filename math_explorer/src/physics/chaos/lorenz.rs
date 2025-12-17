//! Continuous Chaos (The Lorenz System)

use nalgebra::Vector3;
use crate::pure_math::analysis::ode::{OdeSystem, RungeKutta4, Solver};

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

impl LorenzSystem {
    /// Creates a new LorenzSystem with standard chaotic constants: $\sigma=10, \rho=28, \beta=8/3$.
    pub fn default_chaotic(initial_state: LorenzState) -> Self {
        LorenzSystem {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
            state: initial_state,
        }
    }

    /// Advances the system by time `dt` using the Runge-Kutta 4 (RK4) method.
    ///
    /// This now delegates to the generic `RungeKutta4` solver, ensuring DRY and type safety.
    pub fn step(&mut self, dt: f64) {
            self.state.vec = RungeKutta4::step(self, 0.0, &self.state.vec, dt);
    }

    /// Advances the system by time `dt` using a provided solver strategy.
    ///
    /// This allows the user to switch integrators (e.g., Euler, RK4) dynamically.
    pub fn step_with<S: Solver<Vector3<f64>>>(&mut self, solver: &S, dt: f64) {
        self.state.vec = solver.solve(self, 0.0, &self.state.vec, dt);
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
