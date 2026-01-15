//! Type definitions for the Hodgkin-Huxley model.

use std::ops::{Add, AddAssign, Mul, MulAssign};
use crate::pure_math::analysis::ode::VectorOperations;

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
