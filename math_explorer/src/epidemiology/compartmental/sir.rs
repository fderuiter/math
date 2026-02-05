use super::validation::{validate_initial_infected, validate_population, validate_rate};
use crate::epidemiology::error::EpidemiologyError;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// State for the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}

// We assume this macro is available from the parent module.
impl_compartmental_ops!(SIRState, s, i, r);

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
    /// Creates a new SIR Model.
    ///
    /// # Parameters
    /// * `n` - Total population size.
    /// * `i0` - Initial infected population.
    /// * `beta` - Transmission rate.
    /// * `gamma` - Recovery rate.
    pub fn new(n: f64, i0: f64, beta: f64, gamma: f64) -> Result<Self, EpidemiologyError> {
        Self::builder()
            .population(n)
            .initial_infected(i0)
            .transmission_rate(beta)
            .recovery_rate(gamma)
            .build()
    }

    /// Returns a builder for constructing the model.
    pub fn builder() -> SIRModelBuilder {
        SIRModelBuilder::default()
    }

    /// Advances the state by dt using the default solver (Runge-Kutta 4).
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

/// Builder for SIRModel.
#[derive(Default)]
pub struct SIRModelBuilder {
    n: Option<f64>,
    i0: Option<f64>,
    beta: Option<f64>,
    gamma: Option<f64>,
}

impl SIRModelBuilder {
    /// Sets the total population size ($N$).
    pub fn population(mut self, n: f64) -> Self {
        self.n = Some(n);
        self
    }

    /// Sets the initial number of infected individuals ($I_0$).
    pub fn initial_infected(mut self, i0: f64) -> Self {
        self.i0 = Some(i0);
        self
    }

    /// Sets the transmission rate ($\beta$).
    pub fn transmission_rate(mut self, beta: f64) -> Self {
        self.beta = Some(beta);
        self
    }

    /// Sets the recovery rate ($\gamma$).
    pub fn recovery_rate(mut self, gamma: f64) -> Self {
        self.gamma = Some(gamma);
        self
    }

    /// Builds the SIRModel, validating all parameters.
    pub fn build(self) -> Result<SIRModel, EpidemiologyError> {
        let n = self.n.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "n (population)".to_string(),
            value: 0.0, // Placeholder
        })?;
        validate_population(n)?;

        let i0 = self.i0.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "i0 (initial infected)".to_string(),
            value: 0.0,
        })?;
        validate_initial_infected(i0, n)?;

        let beta = self
            .beta
            .ok_or_else(|| EpidemiologyError::InvalidParameter {
                name: "beta (transmission rate)".to_string(),
                value: 0.0,
            })?;
        validate_rate(beta, "beta (transmission rate)")?;

        let gamma = self
            .gamma
            .ok_or_else(|| EpidemiologyError::InvalidParameter {
                name: "gamma (recovery rate)".to_string(),
                value: 0.0,
            })?;
        validate_rate(gamma, "gamma (recovery rate)")?;

        Ok(SIRModel {
            state: SIRState {
                s: n - i0,
                i: i0,
                r: 0.0,
            },
            n,
            beta,
            gamma,
        })
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
        let mut model = SIRModel::new(n, i0, 0.5, 0.1).unwrap();

        // Euler is less accurate but should still run without panic
        model.step_with(&Euler, 0.1);

        assert!(model.state.s <= n);
        assert!(model.state.i >= 0.0);
    }

    #[test]
    fn test_builder() {
        let model = SIRModel::builder()
            .population(1000.0)
            .initial_infected(10.0)
            .transmission_rate(0.5)
            .recovery_rate(0.1)
            .build()
            .unwrap();

        assert_eq!(model.n, 1000.0);
        assert_eq!(model.state.i, 10.0);
    }
}
