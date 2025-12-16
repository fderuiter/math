/// Neuroscience (Hodgkin-Huxley)
///
/// This module implements the Hodgkin-Huxley model for a neuron's action potential.
/// The model describes how action potentials in neurons are initiated and propagated.
/// It is a set of nonlinear differential equations that approximates the electrical characteristics
/// of excitable cells such as neurons and cardiac myocytes.
///
/// The current through the membrane is given by:
/// $$ I = C_m \frac{dV}{dt} + I_{ion} $$
/// where $I_{ion}$ includes Sodium ($Na^+$), Potassium ($K^+$), and Leak ($L$) currents.
use crate::pure_math::analysis::ode::{OdeSystem, Euler};
use std::ops::{Add, Mul};

/// Represents the state of a Hodgkin-Huxley neuron.
#[derive(Clone, Copy, Debug)]
pub struct HodgkinHuxleyState {
    /// Membrane potential (mV)
    pub v: f64,
    /// Gating variable for Potassium channel activation
    pub n: f64,
    /// Gating variable for Sodium channel activation
    pub m: f64,
    /// Gating variable for Sodium channel inactivation
    pub h: f64,
}

impl Add for HodgkinHuxleyState {
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

impl Mul<f64> for HodgkinHuxleyState {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            v: self.v * rhs,
            n: self.n * rhs,
            m: self.m * rhs,
            h: self.h * rhs,
        }
    }
}

// Blanket impl in ode.rs covers this
// impl VectorOperations for HodgkinHuxleyState {}

/// The Hodgkin-Huxley System definition (holding parameters).
pub struct HodgkinHuxleyNeuron {
    /// The current state of the neuron.
    state: HodgkinHuxleyState,
    /// Resting potential (mV).
    pub v_rest: f64,
    /// External current (I_ext). stored here to be used in derivative.
    /// In a more advanced design, I_ext could be a function of time.
    pub i_ext: f64,
}

impl HodgkinHuxleyNeuron {
    pub fn new(v_initial: f64) -> Self {
        let v_rest = -65.0;
        Self {
            state: HodgkinHuxleyState {
                v: v_initial,
                n: 0.32,
                m: 0.05,
                h: 0.6,
            },
            v_rest,
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

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        self.i_ext = i_ext;
        // Use the Euler solver from `ode.rs`
        self.state = Euler::step(self, 0.0, &self.state, dt);
    }

    // Accessors for backward compatibility (mostly) or clarity
    pub fn v(&self) -> f64 { self.state.v }
    pub fn n(&self) -> f64 { self.state.n }
    pub fn m(&self) -> f64 { self.state.m }
    pub fn h(&self) -> f64 { self.state.h }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neuroscience_update() {
        let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
        // Step without current, should stay roughly stable or settle
        neuron.update(0.01, 0.0);
        // Check valid values using new accessors
        assert!(neuron.v().is_finite());
        assert!(neuron.n() >= 0.0 && neuron.n() <= 1.0);
    }
}

impl OdeSystem<HodgkinHuxleyState> for HodgkinHuxleyNeuron {
    fn derivative(&self, _t: f64, state: &HodgkinHuxleyState) -> HodgkinHuxleyState {
        // Constants
        let g_na = 120.0;
        let e_na = self.v_rest + 115.0;
        let g_k = 36.0;
        let e_k = self.v_rest - 12.0;
        let g_l = 0.3;
        let e_l = self.v_rest + 10.6;

        let v = state.v;
        let n = state.n;
        let m = state.m;
        let h = state.h;

        // I_tot
        let i_na = g_na * m.powi(3) * h * (v - e_na);
        let i_k = g_k * n.powi(4) * (v - e_k);
        let i_l = g_l * (v - e_l);

        let dv_dt = self.i_ext - i_na - i_k - i_l;

        // dx/dt = alpha_x * (1 - x) - beta_x * x
        let dn_dt = Self::alpha_n(v, self.v_rest) * (1.0 - n) - Self::beta_n(v, self.v_rest) * n;
        let dm_dt = Self::alpha_m(v, self.v_rest) * (1.0 - m) - Self::beta_m(v, self.v_rest) * m;
        let dh_dt = Self::alpha_h(v, self.v_rest) * (1.0 - h) - Self::beta_h(v, self.v_rest) * h;

        HodgkinHuxleyState {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
        }
    }
}
