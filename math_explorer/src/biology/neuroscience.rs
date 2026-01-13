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
//!
//! # Ion Channel Gating
//!
//! The model uses three gating variables to control the flow of ions:
//! - **$m$**: Sodium channel activation (opens quickly when depolarized).
//! - **$h$**: Sodium channel inactivation (closes slowly when depolarized).
//! - **$n$**: Potassium channel activation (opens slowly when depolarized).
//!
//! ```mermaid
//! graph TD
//!     V[Membrane Potential V]
//!     V -->|Controls| AlphaBeta[Rate Constants alpha/beta]
//!     AlphaBeta -->|Update| Gates[Gating Variables m, h, n]
//!     Gates -->|Conductance| Channels[Ion Channels Na, K]
//!     Channels -->|Ionic Currents| Current[I_Na, I_K]
//!     Current -->|Feedback| V
//!     Ext[External Current] --> V
//! ```
//!
//! # Example
//!
//! ```rust
//! use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;
//!
//! let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
//! // Simulate for 10ms with 10 uA/cm^2 current injection
//! let dt = 0.01;
//! let mut spiked = false;
//!
//! for _ in 0..1000 {
//!     neuron.update(dt, 10.0);
//!     if neuron.v > 0.0 {
//!         spiked = true;
//!     }
//! }
//! assert!(spiked, "Neuron should have generated an action potential");
//! ```

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

    /// Resting potential used for relative calculations (mV).
    pub v_rest: f64,
}

impl HodgkinHuxleyNeuron {
    /// Creates a new neuron state with the given initial membrane potential.
    /// Gating variables are initialized to their steady-state values at rest.
    ///
    /// # Arguments
    /// * `v_initial` - Initial membrane potential (typically -65.0 mV).
    pub fn new(v_initial: f64) -> Self {
        // Initialize gating variables to standard resting values approx.
        let v_rest = -65.0;
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            v_rest,
        }
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

    /// Updates the neuron state by a time step `dt` with external current `i_ext`.
    ///
    /// Uses the Euler method to integrate the differential equations.
    ///
    /// # Arguments
    /// * `dt` - Time step in milliseconds (e.g., 0.01).
    /// * `i_ext` - External injected current ($\mu A/cm^2$).
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        // Constants
        let g_na = 120.0;
        let e_na = self.v_rest + 115.0; // Standard offset from rest
        let g_k = 36.0;
        let e_k = self.v_rest - 12.0;
        let g_l = 0.3;
        let e_l = self.v_rest + 10.6;

        // Calculate currents
        // I_tot equation: I_ext - g_Na m^3 h (V - E_Na) - g_K n^4 (V - E_K) - g_L (V - E_L)
        // Assuming C_m = 1.0 uF/cm^2

        let i_na = g_na * self.m.powi(3) * self.h * (self.v - e_na);
        let i_k = g_k * self.n.powi(4) * (self.v - e_k);
        let i_l = g_l * (self.v - e_l);

        let dv_dt = i_ext - i_na - i_k - i_l; // Assuming C_m = 1

        self.v += dv_dt * dt;

        // Update gating variables
        // dx/dt = alpha_x * (1 - x) - beta_x * x
        let update_gate = |x: f64, alpha: f64, beta: f64| -> f64 {
            let dx_dt = alpha * (1.0 - x) - beta * x;
            x + dx_dt * dt
        };

        self.n = update_gate(
            self.n,
            Self::alpha_n(self.v, self.v_rest),
            Self::beta_n(self.v, self.v_rest),
        );
        self.m = update_gate(
            self.m,
            Self::alpha_m(self.v, self.v_rest),
            Self::beta_m(self.v, self.v_rest),
        );
        self.h = update_gate(
            self.h,
            Self::alpha_h(self.v, self.v_rest),
            Self::beta_h(self.v, self.v_rest),
        );
    }
}
