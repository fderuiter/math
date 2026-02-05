use crate::epidemiology::error::EpidemiologyError;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper, VectorOperations};
use crate::impl_compartmental_ops;

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
/// $$dS/dt = -\beta S I / N$$
/// $$dE/dt = \beta S I / N - \sigma E$$
/// $$dI/dt = \sigma E - \gamma I$$
/// $$dR/dt = \gamma I$$
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
    /// Creates a new Builder for constructing an SEIRModel.
    pub fn builder() -> SEIRBuilder {
        SEIRBuilder::default()
    }

    /// Creates a new SEIR Model.
    ///
    /// # Arguments
    /// * `n` - Total population.
    /// * `i0` - Initial number of infected individuals.
    /// * `beta` - Transmission rate.
    /// * `sigma` - Incubation rate.
    /// * `gamma` - Recovery rate.
    pub fn new(
        n: f64,
        i0: f64,
        beta: f64,
        sigma: f64,
        gamma: f64,
    ) -> Result<Self, EpidemiologyError> {
        SEIRBuilder::default()
            .population(n)
            .initial_infected(i0)
            .transmission_rate(beta)
            .incubation_rate(sigma)
            .recovery_rate(gamma)
            .build()
    }

    /// Advances the state by dt using Runge-Kutta 4.
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

/// Builder for constructing an SEIRModel.
#[derive(Debug, Clone, Default)]
pub struct SEIRBuilder {
    n: Option<f64>,
    i0: Option<f64>,
    beta: Option<f64>,
    sigma: Option<f64>,
    gamma: Option<f64>,
}

impl SEIRBuilder {
    /// Sets the total population size (N).
    pub fn population(mut self, n: f64) -> Self {
        self.n = Some(n);
        self
    }

    /// Sets the initial number of infected individuals (I0).
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

    /// Builds the SEIRModel, verifying all parameters are valid.
    pub fn build(self) -> Result<SEIRModel, EpidemiologyError> {
        let n = self.n.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "n (population)".to_string(),
            value: 0.0,
        })?;
        let i0 = self.i0.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "i0 (initial infected)".to_string(),
            value: 0.0,
        })?;
        let beta = self.beta.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "beta (transmission rate)".to_string(),
            value: 0.0,
        })?;
        let sigma = self.sigma.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "sigma (incubation rate)".to_string(),
            value: 0.0,
        })?;
        let gamma = self.gamma.ok_or_else(|| EpidemiologyError::InvalidParameter {
            name: "gamma (recovery rate)".to_string(),
            value: 0.0,
        })?;

        if n <= 0.0 {
            return Err(EpidemiologyError::InvalidParameter {
                name: "n (population)".to_string(),
                value: n,
            });
        }
        if i0 < 0.0 || i0 > n {
            return Err(EpidemiologyError::InvalidParameter {
                name: "i0 (initial infected)".to_string(),
                value: i0,
            });
        }
        if beta < 0.0 {
            return Err(EpidemiologyError::InvalidParameter {
                name: "beta (transmission rate)".to_string(),
                value: beta,
            });
        }
        if sigma < 0.0 {
            return Err(EpidemiologyError::InvalidParameter {
                name: "sigma (incubation rate)".to_string(),
                value: sigma,
            });
        }
        if gamma < 0.0 {
            return Err(EpidemiologyError::InvalidParameter {
                name: "gamma (recovery rate)".to_string(),
                value: gamma,
            });
        }

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
}
