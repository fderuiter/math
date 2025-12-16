use crate::pure_math::analysis::ode::{OdeSystem, RungeKutta4};
use std::ops::{Add, Mul};

/// State for the SIR Model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SIRState {
    pub s: f64,
    pub i: f64,
    pub r: f64,
}

impl Add for SIRState {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            s: self.s + rhs.s,
            i: self.i + rhs.i,
            r: self.r + rhs.r,
        }
    }
}

impl Mul<f64> for SIRState {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            s: self.s * scalar,
            i: self.i * scalar,
            r: self.r * scalar,
        }
    }
}

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
        self.state = RungeKutta4::step(self, 0.0, &self.state, dt);
    }
}

impl OdeSystem<SIRState> for SIRModel {
    fn derivative(&self, _t: f64, state: &SIRState) -> SIRState {
        let s = state.s;
        let i = state.i;

        let ds = -self.beta * s * i / self.n;
        let di = self.beta * s * i / self.n - self.gamma * i;
        let dr = self.gamma * i;

        SIRState { s: ds, i: di, r: dr }
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

impl Add for SEIRState {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            s: self.s + rhs.s,
            e: self.e + rhs.e,
            i: self.i + rhs.i,
            r: self.r + rhs.r,
        }
    }
}

impl Mul<f64> for SEIRState {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            s: self.s * scalar,
            e: self.e * scalar,
            i: self.i * scalar,
            r: self.r * scalar,
        }
    }
}

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
        self.state = RungeKutta4::step(self, 0.0, &self.state, dt);
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

        SEIRState { s: ds, e: de, i: di, r: dr }
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

    #[test]
    fn test_threshold_theorem() {
        let n = 1000.0;
        let i0 = 10.0;
        // R0 = beta / gamma = 0.5 / 1.0 = 0.5 < 1
        let mut model = SIRModel::new(n, i0, 0.5, 1.0);

        let initial_i = model.state.i;
        model.step(0.1);

        assert!(model.state.i < initial_i, "Infected should decrease when R0 < 1");
    }
}
