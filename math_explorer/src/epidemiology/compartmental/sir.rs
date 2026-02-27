use super::common::{validate_initial_infected, validate_population, validate_rate};
use crate::epidemiology::error::EpidemiologyError;
use crate::impl_compartmental_ops;
use crate::pure_math::analysis::ode::{OdeModel, OdeSystem, RungeKutta4, Solver, TimeStepper};
use std::ops::{Deref, DerefMut};

/// State for the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}

impl_compartmental_ops!(SIRState, s, i, r);

/// Pure dynamics of the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRDynamics {
    pub n: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl OdeSystem<SIRState> for SIRDynamics {
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

/// SIR Model: Susceptible, Infectious, Recovered.
///
/// Equations:
/// $$dS/dt = -\beta S I / N$$
/// $$dI/dt = \beta S I / N - \gamma I$$
/// $$dR/dt = \gamma I$$
///
/// Use `SIRModel::builder()` or `SIRModel::new()` to construct.
#[derive(Debug, Clone)]
pub struct SIRModel<S = RungeKutta4<SIRState>>(pub OdeModel<SIRState, SIRDynamics, S>);

// Deref implementation allows `model.state` access directly
impl<S> Deref for SIRModel<S> {
    type Target = OdeModel<SIRState, SIRDynamics, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> DerefMut for SIRModel<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Delegate OdeSystem
impl<S: Solver<SIRState>> OdeSystem<SIRState> for SIRModel<S> {
    fn derivative(&self, t: f64, state: &SIRState) -> SIRState {
        self.0.derivative(t, state)
    }

    fn derivative_in_place(&self, t: f64, state: &SIRState, out: &mut SIRState) {
        self.0.derivative_in_place(t, state, out);
    }
}

// Delegate TimeStepper
impl<S: Solver<SIRState>> TimeStepper<SIRState> for SIRModel<S> {
    fn get_state(&self) -> &SIRState {
        self.0.get_state()
    }

    fn get_state_mut(&mut self) -> &mut SIRState {
        self.0.get_state_mut()
    }

    fn step(&mut self, dt: f64) {
        self.0.step(dt);
    }

    fn step_with<OtherS: Solver<SIRState>>(&mut self, solver: &mut OtherS, dt: f64) {
        self.0.step_with(solver, dt);
    }
}

/// Builder for SIRModel to ensure valid parameter configuration.
#[derive(Debug, Default, Clone)]
pub struct SIRModelBuilder {
    n: Option<f64>,
    i0: Option<f64>,
    beta: Option<f64>,
    gamma: Option<f64>,
}

impl SIRModelBuilder {
    pub fn n(mut self, n: f64) -> Self {
        self.n = Some(n);
        self
    }
    pub fn i0(mut self, i0: f64) -> Self {
        self.i0 = Some(i0);
        self
    }
    pub fn beta(mut self, beta: f64) -> Self {
        self.beta = Some(beta);
        self
    }
    pub fn gamma(mut self, gamma: f64) -> Self {
        self.gamma = Some(gamma);
        self
    }

    pub fn build(self) -> Result<SIRModel<RungeKutta4<SIRState>>, EpidemiologyError> {
        let n = self.n.ok_or(EpidemiologyError::MissingParameter {
            name: "n (population)".to_string(),
        })?;
        let i0 = self.i0.ok_or(EpidemiologyError::MissingParameter {
            name: "i0 (initial infected)".to_string(),
        })?;
        let beta = self.beta.ok_or(EpidemiologyError::MissingParameter {
            name: "beta (transmission rate)".to_string(),
        })?;
        let gamma = self.gamma.ok_or(EpidemiologyError::MissingParameter {
            name: "gamma (recovery rate)".to_string(),
        })?;

        validate_population(n)?;
        validate_initial_infected(i0, n)?;
        validate_rate("beta (transmission rate)", beta)?;
        validate_rate("gamma (recovery rate)", gamma)?;

        let state = SIRState {
            s: n - i0,
            i: i0,
            r: 0.0,
        };

        Ok(SIRModel(OdeModel::new(
            state,
            SIRDynamics { n, beta, gamma },
            RungeKutta4::new(&state),
        )))
    }
}

// Inherent methods for SIRModel
impl SIRModel<RungeKutta4<SIRState>> {
    pub fn builder() -> SIRModelBuilder {
        SIRModelBuilder::default()
    }

    pub fn new(
        n: f64,
        i0: f64,
        beta: f64,
        gamma: f64,
    ) -> Result<SIRModel<RungeKutta4<SIRState>>, EpidemiologyError> {
        Self::builder().n(n).i0(i0).beta(beta).gamma(gamma).build()
    }
}

impl<S: Solver<SIRState>> SIRModel<S> {
    pub fn with_solver<NewS: Solver<SIRState>>(self, new_solver: NewS) -> SIRModel<NewS> {
        SIRModel(OdeModel {
            state: self.0.state,
            dynamics: self.0.dynamics,
            solver: new_solver,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::{Euler, RungeKutta4};

    #[test]
    fn test_builder() {
        let model = SIRModel::builder()
            .n(1000.0)
            .i0(10.0)
            .beta(0.5)
            .gamma(0.1)
            .build();
        assert!(model.is_ok());
        let model = model.unwrap();
        // Access fields directly via Deref
        assert_eq!(model.dynamics.n, 1000.0);
        assert_eq!(model.dynamics.beta, 0.5);
    }

    #[test]
    fn test_threshold_theorem() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model = SIRModel::new(n, i0, 0.5, 1.0).unwrap();

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
        let mut model_std = SIRModel::new(n, i0, 0.5, 0.1).unwrap();
        let mut model_with = SIRModel::new(n, i0, 0.5, 0.1).unwrap();

        let dt = 0.1;
        model_std.step(dt);
        let state = model_with.state;

        model_with.step_with(&mut RungeKutta4::new(&state), dt);

        assert_eq!(
            model_std.state, model_with.state,
            "step and step_with(RK4) should yield identical results"
        );
    }

    #[test]
    fn test_swapping_solver() {
        let n = 1000.0;
        let i0 = 10.0;
        let model = SIRModel::new(n, i0, 0.5, 0.1).unwrap();

        let state = model.state;
        let mut model_euler = model.with_solver(Euler::new(&state));

        model_euler.step(0.1);

        assert!(model_euler.state.s <= n);
    }
}
