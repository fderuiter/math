use super::common::{validate_initial_infected, validate_population, validate_rate};
use crate::epidemiology::error::EpidemiologyError;
use crate::impl_compartmental_ops;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper};

/// State for the SEIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SEIRState {
    pub s: f64,
    pub e: f64,
    pub i: f64,
    pub r: f64,
}

impl_compartmental_ops!(SEIRState, s, e, i, r);

/// SEIR Model: Susceptible, Exposed, Infectious, Recovered.
///
/// Equations:
/// $$dE/dt = \beta S I / N - \sigma E$$
/// $$dI/dt = \sigma E - \gamma I$$
///
/// Use `SEIRModel::builder()` or `SEIRModel::new()` to construct.
#[derive(Debug, Clone)]
pub struct SEIRModel {
    state: SEIRState,
    n: f64,
    beta: f64,
    sigma: f64,
    gamma: f64,
}

impl TimeStepper<SEIRState> for SEIRModel {
    fn get_state(&self) -> &SEIRState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut SEIRState {
        &mut self.state
    }

    fn step(&mut self, dt: f64) {
        use crate::pure_math::analysis::ode::RungeKutta4;
        let new_state = RungeKutta4::step(self, 0.0, self.get_state(), dt);
        *self.get_state_mut() = new_state;
    }
}

/// Builder for SEIRModel to ensure valid parameter configuration.
#[derive(Debug, Default, Clone)]
pub struct SEIRModelBuilder {
    n: Option<f64>,
    i0: Option<f64>,
    beta: Option<f64>,
    sigma: Option<f64>,
    gamma: Option<f64>,
}

impl SEIRModelBuilder {
    /// Sets the total population size N.
    pub fn n(mut self, n: f64) -> Self {
        self.n = Some(n);
        self
    }

    /// Sets the initial infected count I0.
    pub fn i0(mut self, i0: f64) -> Self {
        self.i0 = Some(i0);
        self
    }

    /// Sets the transmission rate beta.
    pub fn beta(mut self, beta: f64) -> Self {
        self.beta = Some(beta);
        self
    }

    /// Sets the incubation rate sigma.
    pub fn sigma(mut self, sigma: f64) -> Self {
        self.sigma = Some(sigma);
        self
    }

    /// Sets the recovery rate gamma.
    pub fn gamma(mut self, gamma: f64) -> Self {
        self.gamma = Some(gamma);
        self
    }

    /// Builds the SEIRModel, validating all parameters.
    pub fn build(self) -> Result<SEIRModel, EpidemiologyError> {
        let n = self.n.ok_or(EpidemiologyError::MissingParameter {
            name: "n (population)".to_string(),
        })?;
        let i0 = self.i0.ok_or(EpidemiologyError::MissingParameter {
            name: "i0 (initial infected)".to_string(),
        })?;
        let beta = self.beta.ok_or(EpidemiologyError::MissingParameter {
            name: "beta (transmission rate)".to_string(),
        })?;
        let sigma = self.sigma.ok_or(EpidemiologyError::MissingParameter {
            name: "sigma (incubation rate)".to_string(),
        })?;
        let gamma = self.gamma.ok_or(EpidemiologyError::MissingParameter {
            name: "gamma (recovery rate)".to_string(),
        })?;

        validate_population(n)?;
        validate_initial_infected(i0, n)?;
        validate_rate("beta (transmission rate)", beta)?;
        validate_rate("sigma (incubation rate)", sigma)?;
        validate_rate("gamma (recovery rate)", gamma)?;

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

impl SEIRModel {
    /// Returns a new builder for the SEIRModel.
    pub fn builder() -> SEIRModelBuilder {
        SEIRModelBuilder::default()
    }

    /// Constructs a new SEIRModel with the given parameters.
    pub fn new(
        n: f64,
        i0: f64,
        beta: f64,
        sigma: f64,
        gamma: f64,
    ) -> Result<Self, EpidemiologyError> {
        Self::builder()
            .n(n)
            .i0(i0)
            .beta(beta)
            .sigma(sigma)
            .gamma(gamma)
            .build()
    }

    /// Advances the state by dt using Runge-Kutta 4.
    pub fn step(&mut self, dt: f64) {
        <Self as TimeStepper<SEIRState>>::step(self, dt);
    }

    /// Advances the state by dt using a provided solver strategy.
    pub fn step_with<S: Solver<SEIRState>>(&mut self, solver: &mut S, dt: f64) {
        <Self as TimeStepper<SEIRState>>::step_with(self, solver, dt);
    }

    /// Returns the transmission rate beta.
    pub fn beta(&self) -> f64 {
        self.beta
    }

    /// Returns the incubation rate sigma.
    pub fn sigma(&self) -> f64 {
        self.sigma
    }

    /// Returns the recovery rate gamma.
    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    /// Returns the total population size N.
    pub fn n(&self) -> f64 {
        self.n
    }

    /// Returns the current state.
    pub fn state(&self) -> &SEIRState {
        &self.state
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::RungeKutta4;

    #[test]
    fn test_builder() {
        let model = SEIRModel::builder()
            .n(1000.0)
            .i0(10.0)
            .beta(0.5)
            .sigma(0.2)
            .gamma(0.1)
            .build();
        assert!(model.is_ok());
        let model = model.unwrap();
        assert_eq!(model.n(), 1000.0);
        assert_eq!(model.beta(), 0.5);
    }

    #[test]
    fn test_builder_missing_param() {
        let model = SEIRModel::builder().n(1000.0).build();
        assert!(matches!(
            model,
            Err(EpidemiologyError::MissingParameter { .. })
        ));
    }

    #[test]
    fn test_seir_step_with_rk4() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model_std = SEIRModel::new(n, i0, 0.5, 0.2, 0.1).unwrap();
        let mut model_with = SEIRModel::new(n, i0, 0.5, 0.2, 0.1).unwrap();

        let dt = 0.1;
        model_std.step(dt);
        let state = *model_with.state();
        model_with.step_with(&mut RungeKutta4::new(&state), dt);

        assert_eq!(
            model_std.state(),
            model_with.state(),
            "step and step_with(RK4) should yield identical results"
        );
    }
}
