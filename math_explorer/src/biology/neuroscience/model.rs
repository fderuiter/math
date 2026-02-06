//! Mathematical model of the Hodgkin-Huxley equations.

use super::types::{HodgkinHuxleyParameters, HodgkinHuxleyState};
use crate::pure_math::analysis::ode::OdeSystem;

/// The Hodgkin-Huxley system of differential equations.
///
/// Defines the derivatives for the state variables $V, n, m, h$.
///
/// Note: This struct holds a reference to the parameters to avoid copying large strategy objects.
#[derive(Debug, Clone, Copy)]
pub struct HodgkinHuxleyModel<'a> {
    /// Parameters of the model (conductances, potentials, kinetics).
    pub params: &'a HodgkinHuxleyParameters,
    /// External current injection ($\mu A/cm^2$).
    pub i_ext: f64,
}

impl<'a> HodgkinHuxleyModel<'a> {
    pub fn new(params: &'a HodgkinHuxleyParameters, i_ext: f64) -> Self {
        Self { params, i_ext }
    }
}

impl<'a> OdeSystem<HodgkinHuxleyState> for HodgkinHuxleyModel<'a> {
    fn derivative(&self, _t: f64, state: &HodgkinHuxleyState) -> HodgkinHuxleyState {
        // Unpack parameters
        let p = self.params;

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

        // Use v_rest from parameters (Single Source of Truth)
        let alpha_n = p.n_gate.alpha(v, p.v_rest);
        let beta_n = p.n_gate.beta(v, p.v_rest);
        let dn_dt = alpha_n * (1.0 - n) - beta_n * n;

        let alpha_m = p.m_gate.alpha(v, p.v_rest);
        let beta_m = p.m_gate.beta(v, p.v_rest);
        let dm_dt = alpha_m * (1.0 - m) - beta_m * m;

        let alpha_h = p.h_gate.alpha(v, p.v_rest);
        let beta_h = p.h_gate.beta(v, p.v_rest);
        let dh_dt = alpha_h * (1.0 - h) - beta_h * h;

        HodgkinHuxleyState {
            v: dv_dt,
            n: dn_dt,
            m: dm_dt,
            h: dh_dt,
        }
    }
}
