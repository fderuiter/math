//! Kinetics strategy for ion channel gating variables.

/// Defines the voltage-dependent rate constants for gating variables.
///
/// Implement this trait to define custom ion channel dynamics (e.g., for different neuron types).
pub trait GatingKinetics: Send + Sync + std::fmt::Debug {
    /// Rate constant $\alpha_n$ for Potassium activation.
    fn alpha_n(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\beta_n$ for Potassium activation.
    fn beta_n(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\alpha_m$ for Sodium activation.
    fn alpha_m(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\beta_m$ for Sodium activation.
    fn beta_m(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\alpha_h$ for Sodium inactivation.
    fn alpha_h(&self, v: f64, v_rest: f64) -> f64;
    /// Rate constant $\beta_h$ for Sodium inactivation.
    fn beta_h(&self, v: f64, v_rest: f64) -> f64;
}

/// Standard Hodgkin-Huxley kinetics (Squid Giant Axon).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardKinetics;

impl GatingKinetics for StandardKinetics {
    fn alpha_n(&self, v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta_n(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }

    fn alpha_m(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta_m(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }

    fn alpha_h(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    fn beta_h(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}
