//! Mathematical model of the Hodgkin-Huxley equations.

use super::types::{HodgkinHuxleyParameters, HodgkinHuxleyState};
use crate::pure_math::analysis::ode::OdeSystem;

/// The Hodgkin-Huxley system of differential equations.
///
/// Defines the derivatives for the state variables $V, n, m, h$.
#[derive(Debug, Clone, Copy)]
pub struct HodgkinHuxleyModel {
    /// Parameters of the model (conductances, potentials).
    pub params: HodgkinHuxleyParameters,
    /// External current injection ($\mu A/cm^2$).
    pub i_ext: f64,
}

impl HodgkinHuxleyModel {
    pub fn new(params: HodgkinHuxleyParameters, i_ext: f64) -> Self {
        Self { params, i_ext }
    }

    /// Rate constant $\alpha_n$ for Potassium activation.
    fn alpha_n(v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    /// Rate constant $\beta_n$ for Potassium activation.
    fn beta_n(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    /// Rate constant $\alpha_m$ for Sodium activation.
    fn alpha_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    /// Rate constant $\beta_m$ for Sodium activation.
    fn beta_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }

    /// Rate constant $\alpha_h$ for Sodium inactivation.
    fn alpha_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    /// Rate constant $\beta_h$ for Sodium inactivation.
    fn beta_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}

impl OdeSystem<HodgkinHuxleyState> for HodgkinHuxleyModel {
    fn derivative(&self, _t: f64, state: &HodgkinHuxleyState) -> HodgkinHuxleyState {
        // Unpack parameters
        let p = &self.params;

        let v = state.v;
        let n = state.n;
        let m = state.m;
        let h = state.h;

        // I_tot equation: I_ext - g_Na m^3 h (V - E_Na) - g_K n^4 (V - E_K) - g_L (V - E_L)
        let i_na = p.g_na * m.powi(3) * h * (v - p.e_na);
        let i_k = p.g_k * n.powi(4) * (v - p.e_k);
        let i_l = p.g_l * (v - p.e_l);

        let dv_dt = self.i_ext - i_na - i_k - i_l; // Assuming C_m = 1.0

        // Gating variable derivatives
        // dx/dt = alpha_x * (1 - x) - beta_x * x
        let dn_dt = Self::alpha_n(v, p.v_rest) * (1.0 - n) - Self::beta_n(v, p.v_rest) * n;
        let dm_dt = Self::alpha_m(v, p.v_rest) * (1.0 - m) - Self::beta_m(v, p.v_rest) * m;
        let dh_dt = Self::alpha_h(v, p.v_rest) * (1.0 - h) - Self::beta_h(v, p.v_rest) * h;

        HodgkinHuxleyState {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
        }
    }
}
