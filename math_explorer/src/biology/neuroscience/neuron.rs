//! Public facade for the Hodgkin-Huxley neuron.

use super::model::HodgkinHuxleyModel;
use super::types::{HodgkinHuxleyParameters, HodgkinHuxleyState};
use crate::pure_math::analysis::ode::{Euler, Solver};

/// Represents the state of a Hodgkin-Huxley neuron.
///
/// Stores the membrane potential and the state of the three gating variables ($n, m, h$).
pub struct HodgkinHuxleyNeuron {
    /// Membrane potential (mV).
    pub v: f64,
    /// Gating variable for Potassium channel activation ($0 \le n \le 1$).
    pub n: f64,
    /// Gating variable for Sodium channel activation ($0 \le m \le 1$).
    pub m: f64,
    /// Gating variable for Sodium channel inactivation ($0 \le h \le 1$).
    pub h: f64,

    /// Parameters for the neuron model.
    pub params: HodgkinHuxleyParameters,
}

impl HodgkinHuxleyNeuron {
    /// Creates a new neuron state with the given initial membrane potential.
    /// Gating variables are initialized to their steady-state values at rest.
    /// Uses default Hodgkin-Huxley parameters.
    ///
    /// # Arguments
    /// * `v_initial` - Initial membrane potential (typically -65.0 mV).
    pub fn new(v_initial: f64) -> Self {
        // Initialize gating variables to standard resting values approx.
        let params = HodgkinHuxleyParameters::default();
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            params,
        }
    }

    /// Creates a new neuron with custom parameters.
    pub fn new_with_params(v_initial: f64, params: HodgkinHuxleyParameters) -> Self {
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            params,
        }
    }

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    ///
    /// Uses the Euler method by default to maintain backward compatibility.
    ///
    /// # Arguments
    /// * `dt` - Time step in milliseconds (e.g., 0.01).
    /// * `i_ext` - External injected current ($\mu A/cm^2$).
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        self.update_with(dt, i_ext, &Euler);
    }

    /// Updates the neuron state using a provided solver strategy.
    ///
    /// Allows switching between Euler, Runge-Kutta, etc.
    pub fn update_with<S: Solver<HodgkinHuxleyState>>(&mut self, dt: f64, i_ext: f64, solver: &S) {
        // Convert to strongly typed state
        let state = HodgkinHuxleyState {
            v: self.v,
            n: self.n,
            m: self.m,
            h: self.h,
        };

        // Create the model with current parameters
        let model = HodgkinHuxleyModel::new(&self.params, i_ext);

        // Solve using the provided solver
        let new_state = solver.solve(&model, 0.0, &state, dt);

        // Update fields
        self.v = new_state.v;
        self.n = new_state.n;
        self.m = new_state.m;
        self.h = new_state.h;
    }
}
