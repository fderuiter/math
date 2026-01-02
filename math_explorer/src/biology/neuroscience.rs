//! Neuroscience (Hodgkin-Huxley)
//!
//! This module implements the Hodgkin-Huxley model for a neuron's action potential.
//! The model describes how action potentials in neurons are initiated and propagated.
//! It is a set of nonlinear differential equations that approximates the electrical characteristics
//! of excitable cells such as neurons and cardiac myocytes.
//!
//! # 🧠 Model Overview
//!
//! The membrane potential $V$ is controlled by the flow of ions across the membrane.
//! The total current $I$ is given by:
//!
//! $$ I = C_m \frac{dV}{dt} + I_{ion} $$
//!
//! where $I_{ion}$ includes Sodium ($Na^+$), Potassium ($K^+$), and Leak ($L$) currents.
//!
//! # 🔄 Gating Dynamics
//!
//! The probability of ion channels being open is controlled by gating variables $n, m, h$.
//!
//! ```mermaid
//! stateDiagram-v2
//!     [*] --> Resting
//!
//!     state "Sodium Activation (m)" as Na_Act {
//!         Closed_m --> Open_m : Depolarization
//!         Open_m --> Closed_m : Repolarization
//!     }
//!
//!     state "Sodium Inactivation (h)" as Na_Inact {
//!         Open_h --> Closed_h : Depolarization
//!         Closed_h --> Open_h : Repolarization
//!     }
//!
//!     state "Potassium Activation (n)" as K_Act {
//!         Closed_n --> Open_n : Depolarization
//!         Open_n --> Closed_n : Repolarization
//!     }
//!
//!     note right of Na_Act
//!         Fast activation leads to
//!         rapid rising phase (Spike).
//!     end note
//!
//!     note right of K_Act
//!         Slower activation leads to
//!         repolarization (Falling phase).
//!     end note
//! ```
//!
//! # 🚀 Usage
//!
//! ```rust
//! use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;
//!
//! // Initialize a neuron at resting potential (-65 mV)
//! let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
//!
//! // Simulate for 10ms with an external current injection of 10.0 nA
//! let dt = 0.01; // 0.01 ms time step
//! for _ in 0..1000 {
//!     neuron.update(dt, 10.0);
//! }
//!
//! println!("Final Membrane Potential: {:.2} mV", neuron.v);
//! ```

/// Represents the state of a Hodgkin-Huxley neuron.
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

impl HodgkinHuxleyNeuron {
    pub fn new(v_initial: f64) -> Self {
        // Initialize gating variables to equilibrium at v_initial
        // For simplicity, we can start them at some standard values or calculate steady state.
        // Let's start with standard resting values approx.
        let v_rest = -65.0;
        Self {
            v: v_initial,
            n: 0.32,
            m: 0.05,
            h: 0.6,
            v_rest,

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
    /// Uses Euler integration for simplicity as requested/implied.
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        // Constants
        let g_na = 120.0;
        let e_na = self.v_rest + 115.0; // Standard offset from rest
        let g_k = 36.0;
        let e_k = self.v_rest - 12.0;
        let g_l = 0.3;
        let e_l = self.v_rest + 10.6;

        // Calculate currents
        // I_tot equation from prompt: I_ext - g_Na m^3 h (V - E_Na) - g_K n^4 (V - E_K) - g_L (V - E_L)
        // Note: Standard HH usually has C_m * dV/dt = I_ext - I_ionic
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

        self.n = update_gate(self.n, Self::alpha_n(self.v, self.v_rest), Self::beta_n(self.v, self.v_rest));
        self.m = update_gate(self.m, Self::alpha_m(self.v, self.v_rest), Self::beta_m(self.v, self.v_rest));
        self.h = update_gate(self.h, Self::alpha_h(self.v, self.v_rest), Self::beta_h(self.v, self.v_rest));
    }
}
