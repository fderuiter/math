use super::common::{validate_initial_infected, validate_population, validate_rate};
use crate::epidemiology::error::EpidemiologyError;
use crate::impl_compartmental_ops;
use crate::pure_math::analysis::ode::{OdeModel, OdeSystem, RungeKutta4};

/// State for the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}

impl_compartmental_ops!(SIRState, s, i, r);

/// Pure dynamics of the SIR Model.
///
/// This struct holds the parameters and defines the differential equations,
/// but does not hold the simulation state. This allows it to be used
/// as a stateless Strategy or Flyweight.
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
pub type SIRModel<S = RungeKutta4<SIRState>> = OdeModel<SIRState, SIRDynamics, S>;

/// Builder for SIRModel to ensure valid parameter configuration.
#[derive(Debug, Default, Clone)]
pub struct SIRModelBuilder {
    n: Option<f64>,
    i0: Option<f64>,
    beta: Option<f64>,
    gamma: Option<f64>,
}

impl SIRModelBuilder {
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

    /// Sets the recovery rate gamma.
    pub fn gamma(mut self, gamma: f64) -> Self {
        self.gamma = Some(gamma);
        self
    }

    /// Builds the SIRModel, validating all parameters.
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

        Ok(SIRModel::from_parts(
            state,
            SIRDynamics { n, beta, gamma },
            RungeKutta4::new(&state),
        ))
    }
}

impl SIRModelBuilder {
    /// Returns a new builder for the SIRModel.
    /// (This is redundant but kept for discoverability if needed, or simply rely on default)
    pub fn new() -> Self {
        Self::default()
    }
}

// Removed impl SIRModel block as inherent impls on type aliases are not allowed.
// Users should use SIRModelBuilder and access dynamics directly.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::{Euler, RungeKutta4, Solver, TimeStepper};

    #[test]
    fn test_builder() {
        let model = SIRModelBuilder::default()
            .n(1000.0)
            .i0(10.0)
            .beta(0.5)
            .gamma(0.1)
            .build();
        assert!(model.is_ok());
        let model = model.unwrap();
        assert_eq!(model.dynamics.n, 1000.0); // Use dynamics.n
        assert_eq!(model.dynamics.beta, 0.5); // Use dynamics.beta
    }

    #[test]
    fn test_builder_missing_param() {
        let model = SIRModelBuilder::default().n(1000.0).build();
        assert!(matches!(
            model,
            Err(EpidemiologyError::MissingParameter { .. })
        ));
    }

    #[test]
    fn test_threshold_theorem() {
        let n = 1000.0;
        let i0 = 10.0;
        // R0 = beta / gamma = 0.5 / 1.0 = 0.5 < 1
        let mut model = SIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .gamma(1.0)
            .build()
            .unwrap();

        let initial_i = model.state.i; // Use direct field access
        model.step(0.1); // Uses TimeStepper trait

        assert!(
            model.state.i < initial_i,
            "Infected should decrease when R0 < 1"
        );
    }

    #[test]
    fn test_sir_step_with_rk4() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model_std = SIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .gamma(0.1)
            .build()
            .unwrap();
        let mut model_with = SIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .gamma(0.1)
            .build()
            .unwrap();

        let dt = 0.1;
        model_std.step(dt);
        let state = model_with.state; // Copy state

        // Use external solver
        model_with.step_with(&mut RungeKutta4::new(&state), dt);

        assert_eq!(
            model_std.state, model_with.state,
            "step and step_with(RK4) should yield identical results"
        );
    }

    #[test]
    fn test_sir_step_with_euler() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model = SIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
            .gamma(0.1)
            .build()
            .unwrap();

        // Euler is less accurate but should still run without panic
        let state = model.state;
        model.step_with(&mut Euler::new(&state), 0.1);

        assert!(model.state.s <= n);
        assert!(model.state.i >= 0.0);
    }

    #[test]
    fn test_independent_dynamics() {
        // Demonstrate usage of SIRDynamics without SIRModel (pure strategy pattern)
        let dynamics = SIRDynamics {
            n: 1000.0,
            beta: 0.5,
            gamma: 0.1,
        };

        // Initial State
        let mut state = SIRState {
            s: 990.0,
            i: 10.0,
            r: 0.0,
        };

        // Use a generic solver directly with the dynamics and the state
        let mut solver = RungeKutta4::new(&state);
        let dt = 0.1;

        // Step forward
        solver.step(&dynamics, 0.0, &mut state, dt);

        // Verify state changed appropriately
        // S should decrease (infection spreads)
        assert!(state.s < 990.0);
        // I should increase (R0 = 5 > 1)
        assert!(state.i > 10.0);
        // R should increase (recovery)
        assert!(state.r > 0.0);

        // Ensure conservation of mass (population size constant)
        let total = state.s + state.i + state.r;
        assert!((total - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn test_swapping_solver() {
        let n = 1000.0;
        let i0 = 10.0;
        let model = SIRModelBuilder::default()
            .n(n)
            .i0(i0)
            .beta(0.5)
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
