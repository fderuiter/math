//! Ordinary Differential Equation (ODE) solvers.
//!
//! This module provides a trait `OdeSystem` for defining ODEs and solvers
//! like `RungeKutta4` to integrate them numerically.

/// A trait defining a system of Ordinary Differential Equations.
///
/// $$ \frac{d\vec{y}}{dt} = f(t, \vec{y}) $$
pub trait OdeSystem {
    /// Computes the time derivative of the system state.
    ///
    /// # Arguments
    /// * `t` - The current time.
    /// * `state` - The current state vector $\vec{y}$.
    ///
    /// # Returns
    /// The derivative vector $\frac{d\vec{y}}{dt}$.
    fn derivative(&self, t: f64, state: &[f64]) -> Vec<f64>;
}

/// Runge-Kutta 4th Order Solver.
///
/// A classic fixed-step integrator for ODEs.
pub struct RungeKutta4;

impl RungeKutta4 {
    /// Performs a single integration step.
    ///
    /// # Arguments
    /// * `system` - The ODE system to solve.
    /// * `t` - The current time.
    /// * `state` - The current state vector.
    /// * `dt` - The time step size.
    ///
    /// # Returns
    /// The new state vector after time `dt`.
    pub fn step<S: OdeSystem + ?Sized>(system: &S, t: f64, state: &[f64], dt: f64) -> Vec<f64> {
        let k1 = system.derivative(t, state);
        let k2 = system.derivative(t + dt / 2.0, &vec_add(state, &vec_scale(&k1, dt / 2.0)));
        let k3 = system.derivative(t + dt / 2.0, &vec_add(state, &vec_scale(&k2, dt / 2.0)));
        let k4 = system.derivative(t + dt, &vec_add(state, &vec_scale(&k3, dt)));

        // delta = (k1 + 2k2 + 2k3 + k4) * dt / 6
        let k2_2 = vec_scale(&k2, 2.0);
        let k3_2 = vec_scale(&k3, 2.0);
        let sum_k = vec_add(&vec_add(&k1, &k2_2), &vec_add(&k3_2, &k4));
        let delta = vec_scale(&sum_k, dt / 6.0);

        vec_add(state, &delta)
    }
}

// Helper functions for vector arithmetic.
// In a future refactor, these could be replaced by a generic Vector trait.

fn vec_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

fn vec_scale(a: &[f64], s: f64) -> Vec<f64> {
    a.iter().map(|x| x * s).collect()
}
