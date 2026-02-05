use super::error::EpidemiologyError;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

macro_rules! impl_compartmental_ops {
    ($type:ty, $($field:ident),+) => {
        impl Add for $type {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                Self {
                    $($field: self.$field + rhs.$field),+
                }
            }
        }

        impl AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$field += rhs.$field;)+
            }
        }

        impl Mul<f64> for $type {
            type Output = Self;
            fn mul(self, scalar: f64) -> Self {
                Self {
                    $($field: self.$field * scalar),+
                }
            }
        }

        impl MulAssign<f64> for $type {
            fn mul_assign(&mut self, scalar: f64) {
                $(self.$field *= scalar;)+
            }
        }

        impl VectorOperations for $type {
            fn scale_add(&mut self, other: &Self, scale: f64) {
                $(self.$field += other.$field * scale;)+
            }

            fn copy_from(&mut self, other: &Self) {
                *self = *other;
            }
        }
    };
}

/// State for the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}

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
    pub fn new(n: f64, i0: f64, beta: f64, gamma: f64) -> Result<Self, EpidemiologyError> {
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
        if gamma < 0.0 {
            return Err(EpidemiologyError::InvalidParameter {
                name: "gamma (recovery rate)".to_string(),
                value: gamma,
            });
        }

        Ok(Self {
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
    pub fn new(
        n: f64,
        i0: f64,
        beta: f64,
        sigma: f64,
        gamma: f64,
    ) -> Result<Self, EpidemiologyError> {
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

        Ok(Self {
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
    fn test_sir_step_with_euler() {
        let n = 1000.0;
        let i0 = 10.0;
        let mut model = SIRModel::new(n, i0, 0.5, 0.1).unwrap();

        // Euler is less accurate but should still run without panic
        model.step_with(&Euler, 0.1);

        assert!(model.state.s <= n);
        assert!(model.state.i >= 0.0);
    }
}
