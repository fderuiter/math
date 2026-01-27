use crate::impl_vector_ops;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper};

/// State for the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}

impl_vector_ops!(SIRState, s, i, r);

/// SIR Model: Susceptible, Infectious, Recovered.
///
/// Equations:
/// $$dS/dt = -\beta S I / N$$
/// $$dI/dt = \beta S I / N - \gamma I$$
/// $$dR/dt = \gamma I$$
#[derive(Debug, Clone)]
pub struct SIRModel {
    pub state: SIRState,
    pub n: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl TimeStepper<SIRState> for SIRModel {
    fn get_state(&self) -> &SIRState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut SIRState {
        &mut self.state
    }
}

impl SIRModel {
    pub fn new(n: f64, i0: f64, beta: f64, gamma: f64) -> Self {
        Self {
            state: SIRState {
                s: n - i0,
                i: i0,
                r: 0.0,
            },
            n,
            beta,
            gamma,
        }
    }

    /// Advances the state by dt using Runge-Kutta 4.
    pub fn step(&mut self, dt: f64) {
        <Self as TimeStepper<SIRState>>::step(self, dt);
    }

    /// Advances the state by dt using a provided solver strategy.
    pub fn step_with<S: Solver<SIRState>>(&mut self, solver: &S, dt: f64) {
        <Self as TimeStepper<SIRState>>::step_with(self, solver, dt);
    }
}

impl OdeSystem<SIRState> for SIRModel {
    fn derivative(&self, _t: f64, state: &SIRState) -> SIRState {
        let s = state.s;
        let i = state.i;

        let ds = -self.beta * s * i / self.n;
        let di = self.beta * s * i / self.n - self.gamma * i;
        let dr = self.gamma * i;

        SIRState {
            s: ds,
            i: di,
            r: dr,
        }
    }
}

/// State for the SEIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SEIRState {
    pub s: f64,
    pub e: f64,
    pub i: f64,
    pub r: f64,
}

impl_vector_ops!(SEIRState, s, e, i, r);

/// SEIR Model: Susceptible, Exposed, Infectious, Recovered.
///
/// Equations:
/// $$dE/dt = \beta S I / N - \sigma E$$
/// $$dI/dt = \sigma E - \gamma I$$
#[derive(Debug, Clone)]
pub struct SEIRModel {
    pub state: SEIRState,
    pub n: f64,
    pub beta: f64,
    pub sigma: f64,
    pub gamma: f64,
}

impl TimeStepper<SEIRState> for SEIRModel {
    fn get_state(&self) -> &SEIRState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut SEIRState {
        &mut self.state
    }
}

impl SEIRModel {
    pub fn new(n: f64, i0: f64, beta: f64, sigma: f64, gamma: f64) -> Self {
        Self {
            state: SEIRState {
                s: n - i0,
                e: 0.0,
                i: i0,
                r: 0.0,
            },
            n,
            beta,
            sigma,
            gamma,
        }
    }

    pub fn step(&mut self, dt: f64) {
        <Self as TimeStepper<SEIRState>>::step(self, dt);
    }

    /// Advances the state by dt using a provided solver strategy.
    pub fn step_with<S: Solver<SEIRState>>(&mut self, solver: &S, dt: f64) {
        <Self as TimeStepper<SEIRState>>::step_with(self, solver, dt);
    }
}

impl OdeSystem<SEIRState> for SEIRModel {
    fn derivative(&self, _t: f64, state: &SEIRState) -> SEIRState {
        let s = state.s;
        let e = state.e;
        let i = state.i;

        let new_exposed = self.beta * s * i / self.n;
        let ds = -new_exposed;
        let de = new_exposed - self.sigma * e;
        let di = self.sigma * e - self.gamma * i;
        let dr = self.gamma * i;

        SEIRState {
            s: ds,
            e: de,
            i: di,
            r: dr,
        }
    }
}

pub fn basic_reproduction_number(beta: f64, gamma: f64) -> f64 {
    if gamma == 0.0 {
        f64::INFINITY
    } else {
        beta / gamma
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::{Euler, RungeKutta4};

    #[test]
    fn test_threshold_theorem() {
        let n = 1000.0;
        let i0 = 10.0;
        // R0 = beta / gamma = 0.5 / 1.0 = 0.5 < 1
        let mut model = SIRModel::new(n, i0, 0.5, 1.0);

        let initial_i = model.state.i;
        model.step(0.1);

        assert!(
            model.state.i < initial_i,
            "Infected should decrease when R0 < 1"
        );
    }

    #[test]
    fn test_sir_step_with_rk4() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model_std = SIRModel::new(n, i0, 0.5, 0.1);
        let mut model_with = SIRModel::new(n, i0, 0.5, 0.1);

        let dt = 0.1;
        model_std.step(dt);
        model_with.step_with(&RungeKutta4, dt);

        assert_eq!(
            model_std.state, model_with.state,
            "step and step_with(RK4) should yield identical results"
        );
    }

    #[test]
    fn test_seir_step_with_rk4() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model_std = SEIRModel::new(n, i0, 0.5, 0.2, 0.1);
        let mut model_with = SEIRModel::new(n, i0, 0.5, 0.2, 0.1);

        let dt = 0.1;
        model_std.step(dt);
        model_with.step_with(&RungeKutta4, dt);

        assert_eq!(
            model_std.state, model_with.state,
            "step and step_with(RK4) should yield identical results"
        );
    }

    #[test]
    fn test_sir_step_with_euler() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model = SIRModel::new(n, i0, 0.5, 0.1);

        // Euler is less accurate but should still run without panic
        model.step_with(&Euler, 0.1);

        assert!(model.state.s <= n);
        assert!(model.state.i >= 0.0);
    }
}
