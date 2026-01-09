use crate::pure_math::analysis::ode::OdeSystem;
use super::types::{HodgkinHuxleyParameters, HodgkinHuxleyState};

/// The Hodgkin-Huxley system of differential equations.
///
/// Wraps the parameters and the current external input current.
pub struct HodgkinHuxleyModel<'a> {
    pub params: &'a HodgkinHuxleyParameters,
    pub i_ext: f64,
}

impl<'a> HodgkinHuxleyModel<'a> {
    /// Rate constant $\alpha_n$ for Potassium activation.
    pub fn alpha_n(v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    /// Rate constant $\beta_n$ for Potassium activation.
    pub fn beta_n(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    /// Rate constant $\alpha_m$ for Sodium activation.
    pub fn alpha_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    /// Rate constant $\beta_m$ for Sodium activation.
    pub fn beta_m(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }

    /// Rate constant $\alpha_h$ for Sodium inactivation.
    pub fn alpha_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    /// Rate constant $\beta_h$ for Sodium inactivation.
    pub fn beta_h(v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}

impl<'a> OdeSystem<HodgkinHuxleyState> for HodgkinHuxleyModel<'a> {
    fn derivative(&self, _t: f64, state: &HodgkinHuxleyState) -> HodgkinHuxleyState {
        let p = self.params;
        let v = state.v;
        let n = state.n;
        let m = state.m;
        let h = state.h;

        let e_na = p.v_rest + p.e_na_offset;
        let e_k = p.v_rest + p.e_k_offset;
        let e_l = p.v_rest + p.e_l_offset;

        let i_na = p.g_na * m.powi(3) * h * (v - e_na);
        let i_k = p.g_k * n.powi(4) * (v - e_k);
        let i_l = p.g_l * (v - e_l);

        // Assuming C_m = 1.0 uF/cm^2
        let dv_dt = self.i_ext - i_na - i_k - i_l;

        // dx/dt = alpha * (1 - x) - beta * x
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
