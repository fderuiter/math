use super::validation::{validate_initial_infected, validate_population, validate_rate};
use crate::epidemiology::error::EpidemiologyError;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// State for the SEIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SEIRState {
    pub s: f64,
    pub e: f64,
    pub i: f64,
    pub r: f64,
}

// We assume this macro is available from the parent module.
impl_compartmental_ops!(SEIRState, s, e, i, r);

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
    /// Creates a new SEIR Model.
    ///
    /// # Parameters
    /// * `n` - Total population size.
    /// * `i0` - Initial infected population.
    /// * `beta` - Transmission rate.
    /// * `sigma` - Incubation rate (1/incubation period).
    /// * `gamma` - Recovery rate.
    pub fn new(
        n: f64,
        i0: f64,
        beta: f64,
        sigma: f64,
        gamma: f64,
    ) -> Result<Self, EpidemiologyError> {
        Self::builder()
            .population(n)
            .initial_infected(i0)
            .transmission_rate(beta)
            .incubation_rate(sigma)
            .recovery_rate(gamma)
            .build()
    }

    /// Returns a builder for constructing the model.
    pub fn builder() -> SEIRModelBuilder {
        SEIRModelBuilder::default()
    }

    /// Advances the state by dt using the default solver (Runge-Kutta 4).
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

/// Builder for SEIRModel.
#[derive(Default)]
pub struct SEIRModelBuilder {
    n: Option<f64>,
    i0: Option<f64>,
    beta: Option<f64>,
    sigma: Option<f64>,
    gamma: Option<f64>,
}

impl SEIRModelBuilder {
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

    /// Sets the incubation rate ($\sigma$).
    pub fn incubation_rate(mut self, sigma: f64) -> Self {
        self.sigma = Some(sigma);
        self
    }

    /// Sets the recovery rate ($\gamma$).
    pub fn recovery_rate(mut self, gamma: f64) -> Self {
        self.gamma = Some(gamma);
        self
    }

    /// Builds the SEIRModel, validating all parameters.
    pub fn build(self) -> Result<SEIRModel, EpidemiologyError> {
        let n = self.n.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "n (population)".to_string(),
            value: 0.0,
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

        let sigma = self
            .sigma
            .ok_or_else(|| EpidemiologyError::InvalidParameter {
                name: "sigma (incubation rate)".to_string(),
                value: 0.0,
            })?;
        validate_rate(sigma, "sigma (incubation rate)")?;

        let gamma = self
            .gamma
            .ok_or_else(|| EpidemiologyError::InvalidParameter {
                name: "gamma (recovery rate)".to_string(),
                value: 0.0,
            })?;
        validate_rate(gamma, "gamma (recovery rate)")?;

        Ok(SEIRModel {
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
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::RungeKutta4;

    #[test]
    fn test_seir_step_with_rk4() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model_std = SEIRModel::new(n, i0, 0.5, 0.2, 0.1).unwrap();
        let mut model_with = SEIRModel::new(n, i0, 0.5, 0.2, 0.1).unwrap();

        let dt = 0.1;
        model_std.step(dt);
        model_with.step_with(&RungeKutta4, dt);

        assert_eq!(
            model_std.state, model_with.state,
            "step and step_with(RK4) should yield identical results"
        );
    }

    #[test]
    fn test_builder() {
        let model = SEIRModel::builder()
            .population(1000.0)
            .initial_infected(10.0)
            .transmission_rate(0.5)
            .incubation_rate(0.2)
            .recovery_rate(0.1)
            .build()
            .unwrap();

        assert_eq!(model.n, 1000.0);
        assert_eq!(model.sigma, 0.2);
    }
}
