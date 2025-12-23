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

use crate::pure_math::analysis::ode::{OdeSystem, Solver, Euler};
use std::ops::{Add, Mul};

/// Represents the state of a Hodgkin-Huxley neuron.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct NeuronState {
    /// Membrane potential (mV)
    pub v: f64,
    /// Gating variable for Potassium channel activation
    pub n: f64,
    /// Gating variable for Sodium channel activation
    pub m: f64,
    /// Gating variable for Sodium channel inactivation
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

// impl VectorOperations for NeuronState {}
// This is automatically implemented by the blanket impl in pure_math/analysis/ode.rs
// because NeuronState implements Clone + Add + Mul<f64>.

/// Configuration parameters for the Hodgkin-Huxley model.
#[derive(Debug, Clone, Copy)]
pub struct HodgkinHuxleyConfig {
    /// Resting potential (mV)
    pub v_rest: f64,
    /// Membrane capacitance (uF/cm^2)
    pub c_m: f64,
    /// Sodium conductance (mS/cm^2)
    pub g_na: f64,
    /// Potassium conductance (mS/cm^2)
    pub g_k: f64,
    /// Leak conductance (mS/cm^2)
    pub g_l: f64,
    /// Sodium reversal potential offset (mV)
    pub e_na_offset: f64,
    /// Potassium reversal potential offset (mV)
    pub e_k_offset: f64,
    /// Leak reversal potential offset (mV)
    pub e_l_offset: f64,
}

impl Default for HodgkinHuxleyConfig {
    fn default() -> Self {
        Self {
            v_rest: -65.0,
            c_m: 1.0,
            g_na: 120.0,
            g_k: 36.0,
            g_l: 0.3,
            e_na_offset: 115.0,
            e_k_offset: -12.0,
            e_l_offset: 10.6,
        }
    }
}

/// The System definition for Hodgkin-Huxley equations.
/// This struct is stateless regarding the simulation; it only defines the differential equations.
pub struct HodgkinHuxleySystem {
    config: HodgkinHuxleyConfig,
    /// External current applied to the neuron (uA/cm^2)
    /// This is stored here to be accessible during derivative calculation.
    /// In a more complex setup, this could be a closure or a trait object.
    pub i_ext: f64,
}

impl HodgkinHuxleySystem {
    pub fn new(config: HodgkinHuxleyConfig) -> Self {
        Self {
            config,
            i_ext: 0.0,
        }
    }

    fn alpha_n(v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
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
        let c = &self.config;
        let e_na = c.v_rest + c.e_na_offset;
        let e_k = c.v_rest + c.e_k_offset;
        let e_l = c.v_rest + c.e_l_offset;

        // Currents
        let i_na = c.g_na * state.m.powi(3) * state.h * (state.v - e_na);
        let i_k = c.g_k * state.n.powi(4) * (state.v - e_k);
        let i_l = c.g_l * (state.v - e_l);

        let dv_dt = (self.i_ext - i_na - i_k - i_l) / c.c_m;

        // Gating variable derivatives
        // dx/dt = alpha * (1 - x) - beta * x
        let dn_dt = Self::alpha_n(state.v, c.v_rest) * (1.0 - state.n)
            - Self::beta_n(state.v, c.v_rest) * state.n;
        let dm_dt = Self::alpha_m(state.v, c.v_rest) * (1.0 - state.m)
            - Self::beta_m(state.v, c.v_rest) * state.m;
        let dh_dt = Self::alpha_h(state.v, c.v_rest) * (1.0 - state.h)
            - Self::beta_h(state.v, c.v_rest) * state.h;

        NeuronState {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
        }
    }
}

/// A wrapper for the Hodgkin-Huxley neuron simulation.
///
/// This struct holds the current state and the system definition.
/// It delegates the integration step to a `Solver`.
pub struct HodgkinHuxleyNeuron {
    pub system: HodgkinHuxleySystem,
    pub state: NeuronState,
}

impl HodgkinHuxleyNeuron {
    /// Creates a new neuron with default configuration.
    pub fn new(v_initial: f64) -> Self {
        let config = HodgkinHuxleyConfig::default();
        Self {
            system: HodgkinHuxleySystem::new(config),
            state: NeuronState {
                v: v_initial,
                n: 0.32,
                m: 0.05,
                h: 0.6,
            },
        }
    }

    /// Accessor for legacy compatibility.
    pub fn v(&self) -> f64 {
        self.state.v
    }

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    ///
    /// This method uses the `Euler` solver by default to maintain backward compatibility
    /// with the previous behavior (mostly), but refactored to use the generic system.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        self.system.i_ext = i_ext;
        let solver = Euler;
        // Time `t` is not explicitly tracked in this simple update loop, passing 0.0
        self.state = solver.solve(&self.system, 0.0, &self.state, dt);
    }

    /// Updates the neuron state using a specific solver strategy.
    ///
    /// This allows for higher-order integration (e.g., Runge-Kutta 4).
    pub fn update_with<S: Solver<NeuronState>>(&mut self, solver: &S, dt: f64, i_ext: f64) {
        self.system.i_ext = i_ext;
        self.state = solver.solve(&self.system, 0.0, &self.state, dt);
    }
}
