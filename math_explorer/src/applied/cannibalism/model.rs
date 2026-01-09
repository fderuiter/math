//! ODE System implementation for the Cannibalism Model.

use super::types::CannibalismParams;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, RungeKutta4};
use nalgebra::Vector2;

/// A system modeling the dynamics between Normal ($N$) and Cannibalistic ($C$) populations.
///
/// It implements `OdeSystem` for `Vector2<f64>`, where:
/// - `state.x` corresponds to $N$ (Normal population)
/// - `state.y` corresponds to $C$ (Cannibal population)
#[derive(Debug, Clone, Copy)]
pub struct CannibalismModel {
    pub params: CannibalismParams,
}

impl CannibalismModel {
    /// Creates a new model with the given parameters.
    pub fn new(params: CannibalismParams) -> Self {
        Self { params }
    }

    /// Advances the system state by `dt` using the default Runge-Kutta 4 solver.
    pub fn step(&self, state: &Vector2<f64>, dt: f64) -> Vector2<f64> {
        self.step_with(&RungeKutta4, state, dt)
    }

    /// Advances the system state by `dt` using a provided solver strategy.
    pub fn step_with<S: Solver<Vector2<f64>>>(&self, solver: &S, state: &Vector2<f64>, dt: f64) -> Vector2<f64> {
        solver.solve(self, 0.0, state, dt)
    }
}

impl OdeSystem<Vector2<f64>> for CannibalismModel {
    fn derivative(&self, _t: f64, state: &Vector2<f64>) -> Vector2<f64> {
        let n = state.x;
        let c = state.y;
        let p = &self.params;

        // Loss of Normals due to cannibalism: phi(N, C) = alpha * N * C
        let phi_n_c = p.alpha * n * c;

        // dN/dt = beta_n * N + beta_c * C - k_n * N - phi(N, C) - mu_n * N
        // Grouping terms: (beta_n - k_n - mu_n) * N + beta_c * C - phi_n_c
        let dndt = p.beta_n * n + p.beta_c * c - p.k_n * n - phi_n_c - p.mu_n * n;

        // dC/dt = k_n * N - mu_c * C
        let dcdt = p.k_n * n - p.mu_c * c;

        Vector2::new(dndt, dcdt)
    }
}
