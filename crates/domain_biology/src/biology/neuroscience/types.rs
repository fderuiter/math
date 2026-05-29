//! Type definitions for the Hodgkin-Huxley model.

use super::kinetics::{GatingKinetics, StandardKinetics};
use pure_math::pure_math::analysis::ode::VectorOperations;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::sync::Arc;

/// Parameters for the Hodgkin-Huxley model.
///
/// Defines the conductances and equilibrium potentials for the ion channels.
/// Default values correspond to the Squid Giant Axon (Hodgkin & Huxley, 1952).
#[derive(Debug, Clone)]
pub struct HodgkinHuxleyParameters {
    /// Maximum Sodium conductance ($mS/cm^2$).
    pub g_na: f64,
    /// Sodium equilibrium potential (mV).
    pub e_na: f64,
    /// Maximum Potassium conductance ($mS/cm^2$).
    pub g_k: f64,
    /// Potassium equilibrium potential (mV).
    pub e_k: f64,
    /// Leak conductance ($mS/cm^2$).
    pub g_l: f64,
    /// Leak equilibrium potential (mV).
    pub e_l: f64,
    /// Resting potential (mV). Used for gating variable rate calculations.
    pub v_rest: f64,
    /// Strategy for calculating gating variable rates.
    pub kinetics: Arc<dyn GatingKinetics>,
}

impl Default for HodgkinHuxleyParameters {
    fn default() -> Self {
        let v_rest = -65.0;
        Self {
            g_na: 120.0,
            e_na: v_rest + 115.0,
            g_k: 36.0,
            e_k: v_rest - 12.0,
            g_l: 0.3,
            e_l: v_rest + 10.6,
            v_rest,
            kinetics: Arc::new(StandardKinetics),
        }
    }
}

/// Represents the state vector of a Hodgkin-Huxley neuron.
///
/// Contains the membrane potential $V$ and the dimensionless gating variables $n, m, h$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HodgkinHuxleyState {
    /// Membrane potential (mV).
    pub v: f64,
    /// Potassium activation gating variable ($0 \le n \le 1$).
    pub n: f64,
    /// Sodium activation gating variable ($0 \le m \le 1$).
    pub m: f64,
    /// Sodium inactivation gating variable ($0 \le h \le 1$).
    pub h: f64,
}

impl Add for HodgkinHuxleyState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            v: self.v + rhs.v,
            n: self.n + rhs.n,
            m: self.m + rhs.m,
            h: self.h + rhs.h,
        }
    }
}

impl AddAssign for HodgkinHuxleyState {
    fn add_assign(&mut self, rhs: Self) {
        self.v += rhs.v;
        self.n += rhs.n;
        self.m += rhs.m;
        self.h += rhs.h;
    }
}

impl Mul<f64> for HodgkinHuxleyState {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            v: self.v * scalar,
            n: self.n * scalar,
            m: self.m * scalar,
            h: self.h * scalar,
        }
    }
}

impl MulAssign<f64> for HodgkinHuxleyState {
    fn mul_assign(&mut self, scalar: f64) {
        self.v *= scalar;
        self.n *= scalar;
        self.m *= scalar;
        self.h *= scalar;
    }
}

impl VectorOperations for HodgkinHuxleyState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        self.v += other.v * scale;
        self.n += other.n * scale;
        self.m += other.m * scale;
        self.h += other.h * scale;
    }

    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}
