//! Gating variable kinetics for Hodgkin-Huxley models.
//!
//! Defines the `GatingKinetics` trait (Strategy Pattern) and standard implementations
//! for Potassium and Sodium channels.

use std::fmt::Debug;

/// Trait defining the kinetics of a voltage-gated ion channel.
///
/// Implements the Strategy Pattern to allow swapping different channel models.
/// The methods take `v_rest` as context to ensure the channel responds to the
/// cell's current resting potential (Single Source of Truth).
pub trait GatingKinetics: Debug + Send + Sync {
    /// Forward rate constant $\alpha(V)$ ($ms^{-1}$).
    ///
    /// # Arguments
    /// * `v` - Membrane potential (mV).
    /// * `v_rest` - Resting potential (mV).
    fn alpha(&self, v: f64, v_rest: f64) -> f64;

    /// Backward rate constant $\beta(V)$ ($ms^{-1}$).
    fn beta(&self, v: f64, v_rest: f64) -> f64;

    /// Helper for cloning trait objects.
    fn box_clone(&self) -> Box<dyn GatingKinetics>;
}

impl Clone for Box<dyn GatingKinetics> {
    fn clone(&self) -> Box<dyn GatingKinetics> {
        self.box_clone()
    }
}

/// Standard Hodgkin-Huxley Potassium (K+) activation kinetics (n-gate).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardPotassiumKinetics;

impl GatingKinetics for StandardPotassiumKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let u = v - v_rest;
        let x = 10.0 - u;
        if x.abs() < 1e-9 {
            0.1 // Limit as x -> 0
        } else {
            0.01 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let u = v - v_rest;
        0.125 * (-u / 80.0).exp()
    }

    fn box_clone(&self) -> Box<dyn GatingKinetics> {
        Box::new(*self)
    }
}

/// Standard Hodgkin-Huxley Sodium (Na+) activation kinetics (m-gate).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardSodiumActivationKinetics;

impl GatingKinetics for StandardSodiumActivationKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let u = v - v_rest;
        let x = 25.0 - u;
        if x.abs() < 1e-9 {
            1.0
        } else {
            0.1 * x / ((0.1 * x).exp() - 1.0)
        }
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let u = v - v_rest;
        4.0 * (-u / 18.0).exp()
    }

    fn box_clone(&self) -> Box<dyn GatingKinetics> {
        Box::new(*self)
    }
}

/// Standard Hodgkin-Huxley Sodium (Na+) inactivation kinetics (h-gate).
#[derive(Debug, Clone, Copy, Default)]
pub struct StandardSodiumInactivationKinetics;

impl GatingKinetics for StandardSodiumInactivationKinetics {
    fn alpha(&self, v: f64, v_rest: f64) -> f64 {
        let u = v - v_rest;
        0.07 * (-u / 20.0).exp()
    }

    fn beta(&self, v: f64, v_rest: f64) -> f64 {
        let u = v - v_rest;
        1.0 / ((3.0 - 0.1 * u).exp() + 1.0)
    }

    fn box_clone(&self) -> Box<dyn GatingKinetics> {
        Box::new(*self)
    }
}
