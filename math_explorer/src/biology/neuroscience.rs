//! Neuroscience (Hodgkin-Huxley)
//!
//! This module implements the Hodgkin-Huxley model for a neuron's action potential.
//! The model describes how action potentials in neurons are initiated and propagated.
//! It is a set of nonlinear differential equations that approximates the electrical characteristics
//! of excitable cells such as neurons and cardiac myocytes.
//!
//! The current through the membrane is given by:
//! $$ I = C_m \frac{dV}{dt} + I_{ion} $$
//! where $I_{ion}$ includes Sodium ($Na^+$), Potassium ($K^+$), and Leak ($L$) currents.

use crate::pure_math::analysis::ode::{VectorOperations, OdeSystem, Solver, Euler};
use std::ops::{Add, Mul};

/// Configuration parameters for the Hodgkin-Huxley model.
/// Allows for customization of channel conductances and potentials.
#[derive(Debug, Clone, Copy)]
pub struct HodgkinHuxleyConfig {
    pub g_na: f64,
    pub e_na_offset: f64,
    pub g_k: f64,
    pub e_k_offset: f64,
    pub g_l: f64,
    pub e_l_offset: f64,
    pub c_m: f64,
    // Note: v_rest is stored in the neuron instance in the legacy code,
    // but semantically it's a parameter. We'll handle it carefully.
}

impl Default for HodgkinHuxleyConfig {
    fn default() -> Self {
        Self {
            g_na: 120.0,
            e_na_offset: 115.0,
            g_k: 36.0,
            e_k_offset: -12.0,
            g_l: 0.3,
            e_l_offset: 10.6,
            c_m: 1.0,
        }
    }
}

/// The state of the neuron: Membrane potential (V) and gating variables (n, m, h).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NeuronState {
    pub v: f64,
    pub n: f64,
    pub m: f64,
    pub h: f64,
}

impl Add for NeuronState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            v: self.v + rhs.v,
            n: self.n + rhs.n,
            m: self.m + rhs.m,
            h: self.h + rhs.h,
        }
    }
}

impl Mul<f64> for NeuronState {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            v: self.v * scalar,
            n: self.n * scalar,
            m: self.m * scalar,
            h: self.h * scalar,
        }
    }
}

/// The Hodgkin-Huxley ODE System definition.
pub struct HodgkinHuxleySystem {
    pub config: HodgkinHuxleyConfig,
    pub v_rest: f64,
    pub i_ext: f64,
}

impl HodgkinHuxleySystem {
    fn alpha_n(v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta_n(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    fn alpha_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }

    fn alpha_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    fn beta_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}

impl OdeSystem<NeuronState> for HodgkinHuxleySystem {
    fn derivative(&self, _t: f64, state: &NeuronState) -> NeuronState {
        let v = state.v;
        let n = state.n;
        let m = state.m;
        let h = state.h;
        let v_rest = self.v_rest;

        let e_na = v_rest + self.config.e_na_offset;
        let e_k = v_rest + self.config.e_k_offset;
        let e_l = v_rest + self.config.e_l_offset;

        let i_na = self.config.g_na * m.powi(3) * h * (v - e_na);
        let i_k = self.config.g_k * n.powi(4) * (v - e_k);
        let i_l = self.config.g_l * (v - e_l);

        let dv_dt = (self.i_ext - i_na - i_k - i_l) / self.config.c_m;

        let dn_dt = Self::alpha_n(v, v_rest) * (1.0 - n) - Self::beta_n(v, v_rest) * n;
        let dm_dt = Self::alpha_m(v, v_rest) * (1.0 - m) - Self::beta_m(v, v_rest) * m;
        let dh_dt = Self::alpha_h(v, v_rest) * (1.0 - h) - Self::beta_h(v, v_rest) * h;

        NeuronState {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
        }
    }
}

/// Represents the state of a Hodgkin-Huxley neuron.
///
/// Refactored to use the Strategy Pattern via `OdeSystem` and `Solver`,
/// but maintains the legacy field structure for backward compatibility.
pub struct HodgkinHuxleyNeuron {
    /// Membrane potential (mV)
    pub v: f64,
    /// Gating variable for Potassium channel activation
    pub n: f64,
    /// Gating variable for Sodium channel activation
    pub m: f64,
    /// Gating variable for Sodium channel inactivation
    pub h: f64,
    /// Resting potential used for relative calculations (mV).
    pub v_rest: f64,

    // Internal configuration
    config: HodgkinHuxleyConfig,
}

impl HodgkinHuxleyNeuron {
    pub fn new(v_initial: f64) -> Self {
        let config = HodgkinHuxleyConfig::default();
        let v_rest = -65.0;
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            v_rest,
            config,
        }
    }

    /// Sets the physical configuration for the neuron.
    pub fn set_config(&mut self, config: HodgkinHuxleyConfig) {
        self.config = config;
    }

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    /// Uses Euler integration for simplicity as requested/implied.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        let solver = Euler;
        self.update_with_solver(&solver, dt, i_ext);
    }

    /// Updates the neuron state using a provided solver strategy.
    pub fn update_with_solver<S: Solver<NeuronState>>(&mut self, solver: &S, dt: f64, i_ext: f64) {
        // Copy-In
        let state = NeuronState {
            v: self.v,
            n: self.n,
            m: self.m,
            h: self.h,
        };

        let system = HodgkinHuxleySystem {
            config: self.config,
            v_rest: self.v_rest,
            i_ext,
        };

        // Solve
        let new_state = solver.solve(&system, 0.0, &state, dt);

        // Copy-Out
        self.v = new_state.v;
        self.n = new_state.n;
        self.m = new_state.m;
        self.h = new_state.h;
    }
}
