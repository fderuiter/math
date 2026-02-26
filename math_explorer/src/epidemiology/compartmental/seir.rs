use super::common::{validate_initial_infected, validate_population, validate_rate};
use crate::epidemiology::error::EpidemiologyError;
use crate::impl_compartmental_ops;
use crate::pure_math::analysis::ode::{OdeModel, OdeSystem, RungeKutta4};

/// State for the SEIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SEIRState {
    pub s: f64,
    pub e: f64,
    pub i: f64,
    pub r: f64,
}

impl_compartmental_ops!(SEIRState, s, e, i, r);

/// Pure dynamics of the SEIR Model.
///
/// This struct holds the parameters and defines the differential equations,
/// but does not hold the simulation state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SEIRDynamics {
    pub n: f64,
    pub beta: f64,
    pub sigma: f64,
    pub gamma: f64,
}

impl OdeSystem<SEIRState> for SEIRDynamics {
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

/// SEIR Model: Susceptible, Exposed, Infectious, Recovered.
///
/// Equations:
/// $$dE/dt = \beta S I / N - \sigma E$$
/// $$dI/dt = \sigma E - \gamma I$$
///
/// Use `SEIRModel::builder()` or `SEIRModel::new()` to construct.
pub type SEIRModel<S = RungeKutta4<SEIRState>> = OdeModel<SEIRState, SEIRDynamics, S>;

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
    pub fn build(self) -> Result<SEIRModel<RungeKutta4<SEIRState>>, EpidemiologyError> {
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

        let state = SEIRState {
            s: n - i0,
            e: 0.0,
            i: i0,
            r: 0.0,
        };

        Ok(SEIRModel::from_parts(
            state,
            SEIRDynamics {
                n,
                beta,
                sigma,
                gamma,
            },
            RungeKutta4::new(&state),
        ))
    }
}

// Removed impl SEIRModel block.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::{Euler, RungeKutta4, Solver, TimeStepper};

    #[test]
    fn test_builder() {
        let model = SEIRModelBuilder::default()
            .n(1000.0)
            .i0(10.0)
            .beta(0.5)
            .sigma(0.2)
            .gamma(0.1)
            .build();
        assert!(model.is_ok());
        let model = model.unwrap();
        assert_eq!(model.dynamics.n, 1000.0);
        assert_eq!(model.dynamics.beta, 0.5);
    }

    #[test]
    fn test_builder_missing_param() {
        let model = SEIRModelBuilder::default().n(1000.0).build();
        assert!(matches!(
            model,
            Err(EpidemiologyError::MissingParameter { .. })
        ));
    }

    #[test]
    fn test_seir_step_with_rk4() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model_std = SEIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .sigma(0.2)
            .gamma(0.1)
            .build()
            .unwrap();
        let mut model_with = SEIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .sigma(0.2)
            .gamma(0.1)
            .build()
            .unwrap();

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
        let model = SEIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .sigma(0.2)
            .gamma(0.1)
            .build()
            .unwrap();

        // Swap from RK4 (default) to Euler
        let state = model.state;
        let mut model_euler = model.with_solver(Euler::new(&state));

        model_euler.step(0.1);

        // Just check it ran
        assert!(model_euler.state.s <= n);
    }
}
