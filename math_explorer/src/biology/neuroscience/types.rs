//! Type definitions for the Hodgkin-Huxley model.

use std::ops::{Add, Mul};

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

// The blanket implementation in `ode.rs` covers this type since it implements Add and Mul.
// impl VectorOperations for HodgkinHuxleyState {}
