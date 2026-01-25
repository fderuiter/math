//! Strong types for battery degradation modeling.
//!
//! These types ensure physical validity of parameters (e.g., depth of discharge cannot be negative).

use super::BatteryError;
use std::fmt;

/// Depth of Discharge (DoD) as a percentage (0.0 to 100.0).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DepthOfDischarge(f64);

impl DepthOfDischarge {
    /// Creates a new `DepthOfDischarge`.
    ///
    /// # Errors
    ///
    /// Returns `BatteryError::InvalidDepthOfDischarge` if `value` is not between 0.0 and 100.0 inclusive.
    pub fn new(value: f64) -> Result<Self, BatteryError> {
        if !(0.0..=100.0).contains(&value) {
            return Err(BatteryError::InvalidDepthOfDischarge(value));
        }
        Ok(Self(value))
    }

    /// Returns the value as a f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for DepthOfDischarge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}%", self.0)
    }
}

/// Battery Capacity as a fraction of initial capacity (0.0 to 1.0).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Capacity(f64);

impl Capacity {
    /// Creates a new `Capacity`.
    ///
    /// # Errors
    ///
    /// Returns `BatteryError::InvalidCapacity` if `value` is not between 0.0 and 1.0 inclusive.
    pub fn new(value: f64) -> Result<Self, BatteryError> {
        if !(0.0..=1.0).contains(&value) {
            return Err(BatteryError::InvalidCapacity(value));
        }
        Ok(Self(value))
    }

    /// Returns the value as a f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Capacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// Number of Equivalent Full Cycles (EFC).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cycles(f64);

impl Cycles {
    /// Creates a new `Cycles`.
    ///
    /// # Errors
    ///
    /// Returns `BatteryError::NegativeCycles` if `value` is negative.
    pub fn new(value: f64) -> Result<Self, BatteryError> {
        if value < 0.0 {
            return Err(BatteryError::NegativeCycles(value));
        }
        Ok(Self(value))
    }

    /// Returns the value as a f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Cycles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} cycles", self.0)
    }
}
