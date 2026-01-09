use std::ops::{Add, Mul};
use crate::pure_math::analysis::ode::VectorOperations;

/// Constants for the Hodgkin-Huxley model.
///
/// These define the conductance and reversal potentials for the ion channels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HodgkinHuxleyParameters {
    /// Sodium conductance ($g_{Na}$) in $mS/cm^2$.
    pub g_na: f64,
    /// Potassium conductance ($g_{K}$) in $mS/cm^2$.
    pub g_k: f64,
    /// Leak conductance ($g_{L}$) in $mS/cm^2$.
    pub g_l: f64,
    /// Sodium reversal potential offset ($E_{Na} - V_{rest}$) in mV.
    pub e_na_offset: f64,
    /// Potassium reversal potential offset ($E_{K} - V_{rest}$) in mV.
    pub e_k_offset: f64,
    /// Leak reversal potential offset ($E_{L} - V_{rest}$) in mV.
    pub e_l_offset: f64,
    /// Resting potential ($V_{rest}$) in mV.
    pub v_rest: f64,
}

impl Default for HodgkinHuxleyParameters {
    fn default() -> Self {
        Self {
            g_na: 120.0,
            g_k: 36.0,
            g_l: 0.3,
            e_na_offset: 115.0,
            e_k_offset: -12.0,
            e_l_offset: 10.6,
            v_rest: -65.0,
        }
    }
}

/// The state vector for the Hodgkin-Huxley model.
///
/// Contains the membrane potential $V$ and the gating variables $n, m, h$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HodgkinHuxleyState {
    pub v: f64,
    pub n: f64,
    pub m: f64,
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

// impl VectorOperations for HodgkinHuxleyState {} // Covered by blanket impl
