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

use crate::pure_math::analysis::ode::{OdeSystem, Solver};
use std::ops::{Add, Mul};

/// Represents the state vector of a Hodgkin-Huxley neuron.
/// This struct is used for ODE integration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HHState {
    pub v: f64,
    pub n: f64,
    pub m: f64,
    pub h: f64,
}

impl Add for HHState {
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

impl Mul<f64> for HHState {
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

/// Configuration parameters for the Hodgkin-Huxley model.
///
/// Contains the biophysical parameters defining the neuron's conductance and reversal potentials.
#[derive(Debug, Clone, Copy)]
pub struct HodgkinHuxleyParams {
    /// Sodium conductance ($mS/cm^2$)
    pub g_na: f64,
    /// Sodium reversal potential ($mV$)
    pub e_na: f64,
    /// Potassium conductance ($mS/cm^2$)
    pub g_k: f64,
    /// Potassium reversal potential ($mV$)
    pub e_k: f64,
    /// Leak conductance ($mS/cm^2$)
    pub g_l: f64,
    /// Leak reversal potential ($mV$)
    pub e_l: f64,
    /// Membrane capacitance ($uF/cm^2$)
    pub c_m: f64,
    /// Resting potential ($mV$)
    pub v_rest: f64,
}

impl HodgkinHuxleyParams {
    /// Creates a standard set of parameters based on a given resting potential.
    pub fn new(v_rest: f64) -> Self {
        Self {
            g_na: 120.0,
            e_na: v_rest + 115.0,
            g_k: 36.0,
            e_k: v_rest - 12.0,
            g_l: 0.3,
            e_l: v_rest + 10.6,
            c_m: 1.0,
            v_rest,
        }
    }
}

/// The ODE definition for the Hodgkin-Huxley model.
///
/// $$ C_m \dot{V} = I_{ext} - g_{Na} m^3 h (V - E_{Na}) - g_K n^4 (V - E_K) - g_L (V - E_L) $$
/// $$ \dot{n} = \alpha_n(V) (1-n) - \beta_n(V) n $$
/// $$ \dot{m} = \alpha_m(V) (1-m) - \beta_m(V) m $$
/// $$ \dot{h} = \alpha_h(V) (1-h) - \beta_h(V) h $$
pub struct HodgkinHuxleyModel {
    pub params: HodgkinHuxleyParams,
    pub i_ext: f64,
}

impl HodgkinHuxleyModel {
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

impl OdeSystem<HHState> for HodgkinHuxleyModel {
    fn derivative(&self, _t: f64, state: &HHState) -> HHState {
        let p = &self.params;
        let v = state.v;
        let n = state.n;
        let m = state.m;
        let h = state.h;

        // Ionic currents
        let i_na = p.g_na * m.powi(3) * h * (v - p.e_na);
        let i_k = p.g_k * n.powi(4) * (v - p.e_k);
        let i_l = p.g_l * (v - p.e_l);

        // Membrane potential derivative
        // C_m dV/dt = I_ext - I_ion
        let dv_dt = (self.i_ext - i_na - i_k - i_l) / p.c_m;

        // Gating variable derivatives
        // dx/dt = alpha * (1 - x) - beta * x
        let an = Self::alpha_n(v, p.v_rest);
        let bn = Self::beta_n(v, p.v_rest);
        let dn_dt = an * (1.0 - n) - bn * n;

        let am = Self::alpha_m(v, p.v_rest);
        let bm = Self::beta_m(v, p.v_rest);
        let dm_dt = am * (1.0 - m) - bm * m;

        let ah = Self::alpha_h(v, p.v_rest);
        let bh = Self::beta_h(v, p.v_rest);
        let dh_dt = ah * (1.0 - h) - bh * h;

        HHState {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
        }
    }
}

/// Represents the state of a Hodgkin-Huxley neuron.
///
/// # Legacy Note
/// This struct wraps `HHState` internally but exposes individual fields `v`, `n`, `m`, `h`
/// via a manual sync mechanism to preserve backward compatibility.
/// Future code should prefer using `HHState` directly or accessors.
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

    /// Configuration parameters.
    pub params: HodgkinHuxleyParams,
}

impl HodgkinHuxleyNeuron {
    pub fn new(v_initial: f64) -> Self {
        // Initialize gating variables to equilibrium at v_initial
        // Standard resting values approximated to match legacy initialization.
        // Legacy: n=0.32, m=0.05, h=0.6, v_rest=-65.0
        let v_rest = -65.0;
        let params = HodgkinHuxleyParams::new(v_rest);

        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            v_rest,
            params,
        }
    }

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    ///
    /// This method uses the explicit Euler method to match legacy behavior.
    /// For higher precision, construct a `HodgkinHuxleyModel` and use `RungeKutta4` directly.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        // 1. Pack state
        let current_state = HHState {
            v: self.v,
            n: self.n,
            m: self.m,
            h: self.h,
        };

        // 2. Create Model
        let model = HodgkinHuxleyModel {
            params: self.params,
            i_ext,
        };

        // 3. Solve using Legacy Semi-Implicit Euler Strategy
        // This reproduces the exact numerical behavior of the original implementation,
        // which updated V first, then updated gating variables using the NEW V.
        struct LegacySemiImplicitEuler;
        impl Solver<HHState> for LegacySemiImplicitEuler {
            fn solve<S>(&self, system: &S, t: f64, state: &HHState, dt: f64) -> HHState
            where
                S: OdeSystem<HHState> + ?Sized,
            {
                // Calculate dV/dt using OLD state
                let deriv_old = system.derivative(t, state);

                // Update V
                let v_new = state.v + deriv_old.v * dt;

                // Create a temporary state with NEW V but OLD gating vars to calculate gating derivatives
                // Note: The original implementation effectively did:
                // self.v += dv_dt * dt;
                // self.n = update_gate(self.n, alpha(self.v), ...) // uses NEW v

                // We can't perfectly use `system.derivative` for this mixed state easily if it's coupled.
                // However, `derivative` calculates everything.
                // Let's look at `derivative`: it uses `state.v` for alpha/beta.

                let mixed_state = HHState {
                    v: v_new,
                    n: state.n,
                    m: state.m,
                    h: state.h,
                };

                let deriv_mixed = system.derivative(t, &mixed_state);

                HHState {
                    v: v_new, // Already updated
                    n: state.n + deriv_mixed.n * dt,
                    m: state.m + deriv_mixed.m * dt,
                    h: state.h + deriv_mixed.h * dt,
                }
            }
        }

        let solver = LegacySemiImplicitEuler;
        let new_state = solver.solve(&model, 0.0, &current_state, dt);

        // 4. Unpack state
        self.v = new_state.v;
        self.n = new_state.n;
        self.m = new_state.m;
        self.h = new_state.h;
    }
}
