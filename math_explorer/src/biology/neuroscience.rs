//! # Neuroscience: Hodgkin-Huxley Model
//!
//! A mathematical model that describes how action potentials in neurons are initiated and propagated.
//! It is a set of nonlinear differential equations that approximates the electrical characteristics
//! of excitable cells.
//!
//! ## 🧠 The Model
//!
//! The membrane potential $V$ is controlled by the flow of ions across the membrane.
//! The total current $I$ is given by:
//!
//! $$ C_m \frac{dV}{dt} = I_{ext} - I_{Na} - I_K - I_L $$
//!
//! Where the ionic currents are defined by voltage-dependent gating variables:
//!
//! * **Sodium ($Na^+$)**: Controlled by activation ($m$) and inactivation ($h$).
//! * **Potassium ($K^+$)**: Controlled by activation ($n$).
//! * **Leak ($L$)**: Passive flow.
//!
//! ## 📊 Gating Dynamics
//!
//! ```mermaid
//! graph LR
//!     subgraph Sodium["Sodium (Na+) Channel"]
//!         M[m: Activation] -- Fast --> O_Na[Open Probability: m³h]
//!         H[h: Inactivation] -- Slow --> O_Na
//!     end
//!     subgraph Potassium["Potassium (K+) Channel"]
//!         N[n: Activation] -- Slow --> O_K[Open Probability: n⁴]
//!     end
//!
//!     O_Na --> |Depolarization| V[Membrane Potential]
//!     O_K --> |Repolarization| V
//! ```
//!
//! ## 🚀 Usage
//!
//! ```rust
//! use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;
//!
//! // 1. Initialize a neuron at resting potential (-65 mV)
//! let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
//!
//! // 2. Simulate 10ms with a current injection
//! let dt = 0.01; // 0.01 ms time step
//! let steps = 1000;
//! let current_injection = 20.0; // µA/cm²
//!
//! for i in 0..steps {
//!     neuron.update(dt, current_injection);
//!
//!     if i % 100 == 0 {
//!         println!("Time: {:.2} ms, Voltage: {:.2} mV", i as f64 * dt, neuron.v);
//!     }
//! }
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
    /// Creates a new neuron state.
    ///
    /// # Arguments
    /// * `v_initial` - Initial membrane potential (e.g., -65.0 mV).
    pub fn new(v_initial: f64) -> Self {
        // Initialize gating variables to equilibrium at v_initial
        let v_rest = -65.0;
        Self {
            v: v_initial,
            n: 0.32, // Approximate steady state at rest
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
    ///
    /// Uses Euler integration to solve the differential equations.
    ///
    /// # Arguments
    /// * `dt` - Time step in milliseconds (ms).
    /// * `i_ext` - External current injection ($\mu A / cm^2$).
    pub fn update(&mut self, dt: f64, i_ext: f64) {
        // Constants
        let g_na = 120.0;
        let e_na = self.v_rest + 115.0; // Standard offset from rest
        let g_k = 36.0;
        let e_k = self.v_rest - 12.0;
        let g_l = 0.3;
        let e_l = self.v_rest + 10.6;

        // Calculate currents
        // I_ion = g_Na m^3 h (V - E_Na) + g_K n^4 (V - E_K) + g_L (V - E_L)
        let i_na = g_na * self.m.powi(3) * self.h * (self.v - e_na);
        let i_k = g_k * self.n.powi(4) * (self.v - e_k);
        let i_l = g_l * (self.v - e_l);

        // dV/dt = (I_ext - I_ion) / C_m
        // Assuming C_m = 1.0 uF/cm^2
        let dv_dt = i_ext - i_na - i_k - i_l;

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
