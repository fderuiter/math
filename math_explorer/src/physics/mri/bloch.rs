//! Classical Dynamics Simulator using Bloch Equations.

use super::proton;
use crate::pure_math::analysis::ode::{Euler, OdeSystem, Solver};
use nalgebra::Vector3;

/// Internal helper struct to define the Bloch equations as an OdeSystem.
struct BlochSystem {
    m0: f64,
    t1: f64,
    t2: f64,
    b_field: Vector3<f64>,
}

impl OdeSystem<Vector3<f64>> for BlochSystem {
    fn derivative(&self, _t: f64, state: &Vector3<f64>) -> Vector3<f64> {
        let gamma = proton::GYROMAGNETIC_RATIO;

        // Precession term: M x (gamma B)
        let precession = state.cross(&(self.b_field * gamma));

        // Relaxation terms
        // Transverse relaxation (x and y components decay with T2)
        let transverse_decay = Vector3::new(state.x / self.t2, state.y / self.t2, 0.0);

        // Longitudinal relaxation (z component recovers to M0 with T1)
        let longitudinal_recovery = Vector3::new(0.0, 0.0, (state.z - self.m0) / self.t1);

        // Total derivative dM/dt
        precession - transverse_decay - longitudinal_recovery
    }
}

/// Classical Dynamics Simulator using Bloch Equations.
pub struct BlochSimulator {
    /// Current magnetization vector $\vec{M} = (M_x, M_y, M_z)$.
    pub magnetization: Vector3<f64>,
    /// Equilibrium magnetization $M_0$ (aligned with z-axis).
    pub m0: f64,
}

impl BlochSimulator {
    /// Creates a new BlochSimulator.
    ///
    /// # Arguments
    /// * `initial_magnetization` - Initial state of $\vec{M}$.
    /// * `m0` - Equilibrium magnetization.
    pub fn new(initial_magnetization: Vector3<f64>, m0: f64) -> Self {
        Self {
            magnetization: initial_magnetization,
            m0,
        }
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
    pub fn step_with<S>(&mut self, dt: f64, b_field: Vector3<f64>, t1: f64, t2: f64, solver: &mut S)
    where
        S: Solver<Vector3<f64>>,
    {
        let system = BlochSystem {
            m0: self.m0,
            t1,
            t2,
            b_field,
        };
        // Time is treated as 0.0 for the step since B is constant over the interval
        self.magnetization = solver.solve(&system, 0.0, &self.magnetization, dt);
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
    pub fn step(&mut self, dt: f64, b_field: Vector3<f64>, t1: f64, t2: f64) {
        // Create a solver with the current magnetization structure
        let mut solver = Euler::new(&self.magnetization);
        self.step_with(dt, b_field, t1, t2, &mut solver)
    }
}
