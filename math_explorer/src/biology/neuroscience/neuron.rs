use super::types::HodgkinHuxleyParameters;
use super::model::HodgkinHuxleyModel;

/// Represents the state of a Hodgkin-Huxley neuron.
///
/// Stores the membrane potential and the state of the three gating variables ($n, m, h$).
///
/// This struct acts as a high-level controller for the underlying ODE model.
pub struct HodgkinHuxleyNeuron {
    /// Membrane potential (mV).
    pub v: f64,
    /// Gating variable for Potassium channel activation ($0 \le n \le 1$).
    pub n: f64,
    /// Gating variable for Sodium channel activation ($0 \le m \le 1$).
    pub m: f64,
    /// Gating variable for Sodium channel inactivation ($0 \le h \le 1$).
    pub h: f64,

    /// Resting potential used for relative calculations (mV).
    pub v_rest: f64,

    /// Configuration parameters for the model.
    pub params: HodgkinHuxleyParameters,
}

impl HodgkinHuxleyNeuron {
    /// Creates a new neuron state with the given initial membrane potential.
    /// Gating variables are initialized to their steady-state values at rest.
    ///
    /// # Arguments
    /// * `v_initial` - Initial membrane potential (typically -65.0 mV).
    pub fn new(v_initial: f64) -> Self {
        let params = HodgkinHuxleyParameters::default();
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            v_rest: params.v_rest,
            params,
        }
    }

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    ///
    /// # Legacy Behavior
    /// This method preserves the exact staggered integration order of the original implementation:
    /// 1. Update $V$ using current $n, m, h$.
    /// 2. Update $n, m, h$ using the **new** $V$.
    ///
    /// For standard ODE solver behavior (simultaneous update), construct a `HodgkinHuxleyModel`
    /// and use `Solver::solve`.
    ///
    /// # Arguments
    /// * `dt` - Time step in milliseconds (e.g., 0.01).
    /// * `i_ext` - External injected current ($\mu A/cm^2$).
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        // Sync v_rest if it was changed externally
        if (self.params.v_rest - self.v_rest).abs() > 1e-9 {
             self.params.v_rest = self.v_rest;
        }

        let p = &self.params;

        // 1. Calculate currents based on current state
        let e_na = p.v_rest + p.e_na_offset;
        let e_k = p.v_rest + p.e_k_offset;
        let e_l = p.v_rest + p.e_l_offset;

        let i_na = p.g_na * self.m.powi(3) * self.h * (self.v - e_na);
        let i_k = p.g_k * self.n.powi(4) * (self.v - e_k);
        let i_l = p.g_l * (self.v - e_l);

        // 2. Update Membrane Potential V
        let dv_dt = i_ext - i_na - i_k - i_l;
        self.v += dv_dt * dt;

        // 3. Update Gating Variables using the NEW V (Staggered/Semi-Implicit)
        // We reuse the alpha/beta logic from the Model to avoid duplication,
        // but we apply it in the staggered order.

        let update_gate = |x: f64, alpha: f64, beta: f64| -> f64 {
            let dx_dt = alpha * (1.0 - x) - beta * x;
            x + dx_dt * dt
        };

        // Note: We need to access alpha/beta functions.
        // Since they are private helper functions in Model, we should expose them
        // or just implement them on Parameters?
        // They were implemented as private associated functions on HodgkinHuxleyModel.
        // I'll make them public in Model or duplicate the logic if I can't access them.
        // Better: Make them public associated functions on HodgkinHuxleyModel.

        self.n = update_gate(self.n, HodgkinHuxleyModel::alpha_n(self.v, p.v_rest), HodgkinHuxleyModel::beta_n(self.v, p.v_rest));
        self.m = update_gate(self.m, HodgkinHuxleyModel::alpha_m(self.v, p.v_rest), HodgkinHuxleyModel::beta_m(self.v, p.v_rest));
        self.h = update_gate(self.h, HodgkinHuxleyModel::alpha_h(self.v, p.v_rest), HodgkinHuxleyModel::beta_h(self.v, p.v_rest));
    }
}
