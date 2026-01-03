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

use crate::pure_math::analysis::ode::{OdeSystem, Solver, VectorOperations, Euler};
use std::ops::{Add, Mul};

/// Parameters for the Hodgkin-Huxley neuron model.
#[derive(Debug, Clone, Copy)]
pub struct HodgkinHuxleyParameters {
    pub g_na: f64,
    pub e_na_offset: f64, // Standard offset from rest (115.0)
    pub g_k: f64,
    pub e_k_offset: f64, // Standard offset from rest (-12.0)
    pub g_l: f64,
    pub e_l_offset: f64, // Standard offset from rest (10.6)
    pub c_m: f64,        // Membrane capacitance (usually 1.0 uF/cm^2)
}

impl Default for HodgkinHuxleyParameters {
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

/// Represents the state of a Hodgkin-Huxley neuron.
#[derive(Debug, Clone, Copy, PartialEq)]
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
}

impl Add for HodgkinHuxleyNeuron {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            v: self.v + rhs.v,
            n: self.n + rhs.n,
            m: self.m + rhs.m,
            h: self.h + rhs.h,
            // When adding a derivative (rhs) to a state (self), we preserve the original v_rest.
            // If both are states, this operation is ambiguous in physical meaning, but for
            // integration (State + dState), we want to keep the base parameters.
            // The derivative should effectively have v_rest = 0.
            v_rest: self.v_rest,
        }
    }
}

impl Mul<f64> for HodgkinHuxleyNeuron {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            v: self.v * scalar,
            n: self.n * scalar,
            m: self.m * scalar,
            h: self.h * scalar,
            // When scaling a derivative, v_rest (if 0 in derivative) stays 0.
            // When scaling a state, v_rest scales too? No, parameters shouldn't scale.
            // But VectorOperations implies mathematical vector space.
            // We'll treat v_rest as metadata that propagates from the LHS of an addition.
            // For multiplication, if this is a derivative, v_rest is likely dummy.
            v_rest: self.v_rest,
        }
    }
}

// Satisfy the VectorOperations trait requirements.
// The conflicting implementation error happened because there is a blanket impl for T
// where T: Sized + Clone + Add + Mul.
// Since we implemented Add and Mul for HodgkinHuxleyNeuron, it automatically gets VectorOperations.
// We should NOT implement it manually.
// impl VectorOperations for HodgkinHuxleyNeuron {}

impl HodgkinHuxleyNeuron {
    pub fn new(v_initial: f64) -> Self {
        // Initialize gating variables to equilibrium at v_initial
        let v_rest = -65.0;
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            v_rest,
        }
    }

    /// Helper functions for gating variables
    fn alpha_n(v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 { 0.1 } else { 0.01 * x / ((0.1 * x).exp() - 1.0) }
    }

    fn beta_n(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    fn alpha_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 { 1.0 } else { 0.1 * x / ((0.1 * x).exp() - 1.0) }
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

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    /// Uses Euler integration by default to maintain backward compatibility.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        let params = HodgkinHuxleyParameters::default();
        let system = HodgkinHuxleySystem {
            params,
            i_ext,
        };

        let solver = Euler;
        *self = solver.solve(&system, 0.0, self, dt);
    }

    /// Updates the neuron state using a specific solver strategy.
    /// This allows for higher-order integration methods (like RK4).
    pub fn step_with<S: Solver<HodgkinHuxleyNeuron>>(&mut self, solver: &S, dt: f64, i_ext: f64) {
        let params = HodgkinHuxleyParameters::default();
        let system = HodgkinHuxleySystem {
            params,
            i_ext,
        };
        *self = solver.solve(&system, 0.0, self, dt);
    }
}

/// The System wrapper that implements OdeSystem logic
pub struct HodgkinHuxleySystem {
    pub params: HodgkinHuxleyParameters,
    pub i_ext: f64,
}

impl OdeSystem<HodgkinHuxleyNeuron> for HodgkinHuxleySystem {
    fn derivative(&self, _t: f64, state: &HodgkinHuxleyNeuron) -> HodgkinHuxleyNeuron {
        let p = &self.params;
        let v_rest = state.v_rest;

        let e_na = v_rest + p.e_na_offset;
        let e_k = v_rest + p.e_k_offset;
        let e_l = v_rest + p.e_l_offset;

        let i_na = p.g_na * state.m.powi(3) * state.h * (state.v - e_na);
        let i_k = p.g_k * state.n.powi(4) * (state.v - e_k);
        let i_l = p.g_l * (state.v - e_l);

        let dv_dt = (self.i_ext - i_na - i_k - i_l) / p.c_m;

        // dx/dt = alpha_x * (1 - x) - beta_x * x
        let dn_dt = HodgkinHuxleyNeuron::alpha_n(state.v, v_rest) * (1.0 - state.n)
                  - HodgkinHuxleyNeuron::beta_n(state.v, v_rest) * state.n;

        let dm_dt = HodgkinHuxleyNeuron::alpha_m(state.v, v_rest) * (1.0 - state.m)
                  - HodgkinHuxleyNeuron::beta_m(state.v, v_rest) * state.m;

        let dh_dt = HodgkinHuxleyNeuron::alpha_h(state.v, v_rest) * (1.0 - state.h)
                  - HodgkinHuxleyNeuron::beta_h(state.v, v_rest) * state.h;

        HodgkinHuxleyNeuron {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
            v_rest, // Use v_rest from state for consistency, though irrelevant for derivative
        }
    }
}
