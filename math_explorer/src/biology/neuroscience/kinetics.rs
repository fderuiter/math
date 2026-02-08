//! Ion channel gating kinetics strategies.
//!
//! Defines the `GatingKinetics` trait and standard implementations for the Hodgkin-Huxley model.

/// Defines the voltage-dependent rate constants ($\alpha$ and $\beta$) for an ion channel gating variable.
pub trait GatingKinetics: Send + Sync + std::fmt::Debug {
    /// Rate constant for channel opening (transition from closed to open).
    ///
    /// # Arguments
    /// * `v` - Current membrane potential (mV).
    /// * `v_rest` - Resting membrane potential (mV).
    fn alpha(&self, v: f64, v_rest: f64) -> f64;

    /// Rate constant for channel closing (transition from open to closed).
    ///
    /// # Arguments
    /// * `v` - Current membrane potential (mV).
    /// * `v_rest` - Resting membrane potential (mV).
    fn beta(&self, v: f64, v_rest: f64) -> f64;
}

/// Standard Potassium channel activation ($n$) kinetics.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardPotassiumKinetics;

impl GatingKinetics for StandardPotassiumKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let x = 10.0 - (v - v_rest);
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.125 * (-dv / 80.0).exp()
    }
}

/// Standard Sodium channel activation ($m$) kinetics.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardSodiumActivationKinetics;

impl GatingKinetics for StandardSodiumActivationKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        let x = 25.0 - dv;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        4.0 * (-dv / 18.0).exp()
    }
}

/// Standard Sodium channel inactivation ($h$) kinetics.
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardSodiumInactivationKinetics;

impl GatingKinetics for StandardSodiumInactivationKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        0.07 * (-dv / 20.0).exp()
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let dv = v - v_rest;
        1.0 / ((3.0 - 0.1 * dv).exp() + 1.0)
    }
}
