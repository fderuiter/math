//! Classical Dynamics Simulator using Bloch Equations.

use super::proton;
use nalgebra::Vector3;
use pure_math::pure_math::analysis::ode::{Euler, OdeSystem, Solver, SolverExt, TimeStepper};

/// Classical Dynamics Simulator using Bloch Equations.
pub struct BlochSimulator {
    /// Current magnetization vector $\vec{M} = (M_x, M_y, M_z)$.
    pub magnetization: Vector3<f64>,
    /// Equilibrium magnetization $M_0$ (aligned with z-axis).
    pub m0: f64,
    /// Longitudinal relaxation time ($T_1$) in seconds.
    pub t1: f64,
    /// Transverse relaxation time ($T_2$) in seconds.
    pub t2: f64,
    /// External magnetic field vector $\vec{B}$ in Tesla.
    pub b_field: Vector3<f64>,
    /// Time step `dt`.
    pub dt: f64,
    /// Numerical integration method.
    pub integration_method: pure_math::pure_math::analysis::ode::IntegrationMethod,
}

impl OdeSystem<Vector3<f64>> for BlochSimulator {
    #[verified_engine::verified]
    fn derivative(&self, _t: f64, state: &Vector3<f64>) -> Vector3<f64> {
        let gamma = proton::GYROMAGNETIC_RATIO;

        // Precession term: M x (gamma B)
        let precession = state.cross(&(self.b_field * gamma));

        // Relaxation terms
        // Transverse relaxation (x and y components decay with T2)
        let transverse_decay = if self.t2.is_infinite() {
            Vector3::zeros()
        } else {
            Vector3::new(state.x / self.t2, state.y / self.t2, 0.0)
        };

        // Longitudinal relaxation (z component recovers to M0 with T1)
        let longitudinal_recovery = if self.t1.is_infinite() {
            Vector3::zeros()
        } else {
            Vector3::new(0.0, 0.0, (state.z - self.m0) / self.t1)
        };

        // Total derivative dM/dt
        precession - transverse_decay - longitudinal_recovery
    }
}

impl TimeStepper<Vector3<f64>> for BlochSimulator {
    #[verified_engine::verified]
    fn get_state(&self) -> &Vector3<f64> {
        &self.magnetization
    }

    #[verified_engine::verified]
    fn get_state_mut(&mut self) -> &mut Vector3<f64> {
        &mut self.magnetization
    }
}

impl BlochSimulator {
    /// Creates a new BlochSimulator.
    ///
    /// # Arguments
    /// * `initial_magnetization` - Initial state of $\vec{M}$.
    /// * `m0` - Equilibrium magnetization.
    #[verified_engine::verified]
    pub fn new(initial_magnetization: Vector3<f64>, m0: f64) -> Self {
        Self {
            magnetization: initial_magnetization,
            m0,
            t1: f64::INFINITY,
            t2: f64::INFINITY,
            b_field: Vector3::zeros(),
            dt: 0.01,
            integration_method: pure_math::pure_math::analysis::ode::IntegrationMethod::RungeKutta4,
        }
    }

    /// Sets the magnetic field vector $\vec{B}$.
    #[verified_engine::verified]
    pub fn set_b_field(&mut self, b_field: Vector3<f64>) {
        self.b_field = b_field;
    }

    /// Sets the relaxation times $T_1$ and $T_2$.
    #[verified_engine::verified]
    pub fn set_relaxation(&mut self, t1: f64, t2: f64) {
        self.t1 = t1;
        self.t2 = t2;
    }

    /// Performs a time-step update of the magnetization vector using a provided solver.
    ///
    /// This method allows dependency injection of the numerical integration strategy (e.g., Euler, RK4).
    ///
    /// # Arguments
    /// * `dt` - Time step in seconds.
    /// * `b_field` - Magnetic field vector $\vec{B}$ in Tesla.
    /// * `t1` - Longitudinal relaxation time in seconds.
    /// * `t2` - Transverse relaxation time in seconds.
    /// * `solver` - The numerical solver strategy to use.
    #[deprecated(
        since = "0.2.0",
        note = "Use set_parameters() then solver.step() or TimeStepper::step()"
    )]
    #[verified_engine::verified]
    pub fn step_with<S>(&mut self, dt: f64, b_field: Vector3<f64>, t1: f64, t2: f64, solver: &mut S)
    where
        S: Solver<Vector3<f64>>,
    {
        self.set_b_field(b_field);
        self.set_relaxation(t1, t2);

        let new_state = solver.solve(self, 0.0, &self.magnetization, dt);
        self.magnetization = new_state;
    }

    /// Performs a time-step update of the magnetization vector using the Bloch equations.
    ///
    /// The coupled differential equations are:
    /// $\frac{d\vec{M}}{dt} = \vec{M} \times (\gamma \vec{B}) - \frac{M_x \hat{i} + M_y \hat{j}}{T_2} - \frac{(M_z - M_0)\hat{k}}{T_1}$
    ///
    /// Uses Euler integration for backward compatibility.
    ///
    /// # Arguments
    /// * `dt` - Time step in seconds.
    /// * `b_field` - Magnetic field vector $\vec{B}$ in Tesla.
    /// * `t1` - Longitudinal relaxation time in seconds.
    /// * `t2` - Transverse relaxation time in seconds.
    #[deprecated(
        since = "0.2.0",
        note = "Use set_parameters() then TimeStepper::step()"
    )]
    #[verified_engine::verified]
    pub fn step(&mut self, dt: f64, b_field: Vector3<f64>, t1: f64, t2: f64) {
        // Create a solver with the current magnetization structure
        let mut solver = Euler::new(&self.magnetization);
        #[allow(deprecated)]
        self.step_with(dt, b_field, t1, t2, &mut solver)
    }
}

use oxidize_core::{ModelConfig, ModelState, SimulationModel};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BlochConfig {
    pub t1: f64,
    pub t2: f64,
    pub b0: f64, // Not directly used in simple relaxation, but typical
    pub dt: f64,
    pub m0: f64,
    #[serde(default)]
    pub integration_method: pure_math::pure_math::analysis::ode::IntegrationMethod,
}

impl ModelConfig for BlochConfig {}

#[derive(Clone)]
pub struct BlochState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ModelState for BlochState {}

impl SimulationModel for BlochSimulator {
    type Config = BlochConfig;
    type State = BlochState;
    type Error = std::io::Error;

    #[verified_engine::verified]
    fn initialize(
        config: Self::Config,
        _provider: oxidize_core::rng::OxidizeRng,
    ) -> Result<Self, Self::Error> {
        let initial_m = Vector3::new(0.0, 1.0, 0.0); // Default 90 deg flipped
        let mut sim = BlochSimulator::new(initial_m, config.m0);
        sim.set_relaxation(config.t1, config.t2);
        sim.dt = config.dt;
        sim.integration_method = config.integration_method;
        Ok(sim)
    }

    #[verified_engine::verified]
    fn step(&mut self) -> Result<(), Self::Error> {
        let dt = self.dt;
        match self.integration_method {
            pure_math::pure_math::analysis::ode::IntegrationMethod::Euler => {
                let mut solver = pure_math::pure_math::analysis::ode::Euler::new(&self.magnetization);
                <Self as pure_math::pure_math::analysis::ode::TimeStepper<Vector3<f64>>>::step_with(self, &mut solver, dt);
            }
            pure_math::pure_math::analysis::ode::IntegrationMethod::RungeKutta4 => {
                <Self as pure_math::pure_math::analysis::ode::TimeStepper<Vector3<f64>>>::step(self, dt);
            }
        }
        Ok(())
    }

    #[verified_engine::verified]
    fn get_state(&self) -> Self::State {
        BlochState {
            x: self.magnetization.x,
            y: self.magnetization.y,
            z: self.magnetization.z,
        }
    }
}
