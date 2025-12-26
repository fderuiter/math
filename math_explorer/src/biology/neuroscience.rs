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
use nalgebra::Vector4;

/// State vector indices for clarity.
/// x: v (Voltage)
/// y: n (K+ activation)
/// z: m (Na+ activation)
/// w: h (Na+ inactivation)
type HHState = Vector4<f64>;

/// The Hodgkin-Huxley System configuration.
///
/// Contains the parameters and equations defining the ODE.
pub struct HodgkinHuxleySystem {
    /// Resting potential (mV).
    pub v_rest: f64,
    /// External current input (µA/cm²).
    pub i_ext: f64,
    // Conductances (mS/cm²)
    pub g_na: f64,
    pub g_k: f64,
    pub g_l: f64,
    // Reversal potentials relative to v_rest (mV)
    pub e_na_offset: f64,
    pub e_k_offset: f64,
    pub e_l_offset: f64,
    // Membrane capacitance (µF/cm²)
    pub c_m: f64,
}

impl Default for HodgkinHuxleySystem {
    fn default() -> Self {
        Self {
            v_rest: -65.0,
            i_ext: 0.0,
            g_na: 120.0,
            g_k: 36.0,
            g_l: 0.3,
            e_na_offset: 115.0,
            e_k_offset: -12.0,
            e_l_offset: 10.6,
            c_m: 1.0,
        }
    }
}

impl HodgkinHuxleySystem {
    pub fn new(v_rest: f64) -> Self {
        Self {
            v_rest,
            ..Default::default()
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

impl OdeSystem<HHState> for HodgkinHuxleySystem {
    fn derivative(&self, _t: f64, state: &HHState) -> HHState {
        let v = state.x;
        let n = state.y;
        let m = state.z;
        let h = state.w;

        let e_na = self.v_rest + self.e_na_offset;
        let e_k = self.v_rest + self.e_k_offset;
        let e_l = self.v_rest + self.e_l_offset;

        // I_ion
        let i_na = self.g_na * m.powi(3) * h * (v - e_na);
        let i_k = self.g_k * n.powi(4) * (v - e_k);
        let i_l = self.g_l * (v - e_l);

        // dV/dt = (I_ext - I_ion) / C_m
        let dv_dt = (self.i_ext - i_na - i_k - i_l) / self.c_m;

        // Gating variable derivatives: dx/dt = alpha(1-x) - beta*x
        let dn_dt = Self::alpha_n(v, self.v_rest) * (1.0 - n) - Self::beta_n(v, self.v_rest) * n;
        let dm_dt = Self::alpha_m(v, self.v_rest) * (1.0 - m) - Self::beta_m(v, self.v_rest) * m;
        let dh_dt = Self::alpha_h(v, self.v_rest) * (1.0 - h) - Self::beta_h(v, self.v_rest) * h;

        Vector4::new(dv_dt, dn_dt, dm_dt, dh_dt)
    }
}

/// Represents the state of a Hodgkin-Huxley neuron.
///
/// Acts as a high-level wrapper around the `HodgkinHuxleySystem` and state vector.
pub struct HodgkinHuxleyNeuron {
    /// The state vector (v, n, m, h).
    state: HHState,
    /// The physical system parameters.
    system: HodgkinHuxleySystem,
}

impl HodgkinHuxleyNeuron {
    /// Creates a new neuron with the given initial membrane potential.
    pub fn new(v_initial: f64) -> Self {
        let v_rest = -65.0; // Default v_rest
        // Initialize gating variables to equilibrium at v_initial
        // We use the static functions from the system logic
        // But for backward compatibility with the previous implementation, we hardcode approximations or calc exact.
        // Previous impl: n: 0.32, m: 0.05, h: 0.6

        let state = Vector4::new(v_initial, 0.32, 0.05, 0.6);
        let system = HodgkinHuxleySystem::new(v_rest);

        Self {
            state,
            system,
        }
    }

    /// Accessor for membrane potential (v).
    pub fn v(&self) -> f64 {
        self.state.x
    }

    /// Accessor for Potassium channel activation (n).
    pub fn n(&self) -> f64 {
        self.state.y
    }

    /// Accessor for Sodium channel activation (m).
    pub fn m(&self) -> f64 {
        self.state.z
    }

    /// Accessor for Sodium channel inactivation (h).
    pub fn h(&self) -> f64 {
        self.state.w
    }

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    ///
    /// Uses Euler integration by default to match legacy behavior.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        self.update_with(dt, i_ext, &Euler)
    }

    /// Updates using a specific solver strategy.
    pub fn update_with<S>(&mut self, dt: f64, i_ext: f64, solver: &S)
    where
        S: Solver<HHState>,
    {
        self.system.i_ext = i_ext;
        // Time is not explicitly tracked in the system equations (autonomous), pass 0.0
        self.state = solver.solve(&self.system, 0.0, &self.state, dt);
    }
}
