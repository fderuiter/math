//! Strong types for battery degradation modeling.
//!
//! These types ensure physical validity of parameters (e.g., depth of discharge cannot be negative).

use std::fmt;

/// Depth of Discharge (DoD) as a percentage (0.0 to 100.0).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DepthOfDischarge(f64);

impl DepthOfDischarge {
    /// Creates a new `DepthOfDischarge`.
    ///
    /// # Panics
    ///
    /// Panics if `value` is not between 0.0 and 100.0 inclusive.
    pub fn new(value: f64) -> Self {
        if !(0.0..=100.0).contains(&value) {
            panic!(
                "DepthOfDischarge must be between 0.0 and 100.0, got {}",
                value
            );
        }
        Self(value)
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
    /// # Panics
    ///
    /// Panics if `value` is not between 0.0 and 1.0 inclusive.
    pub fn new(value: f64) -> Self {
        if !(0.0..=1.0).contains(&value) {
            panic!("Capacity must be between 0.0 and 1.0, got {}", value);
        }
        Self(value)
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
    /// # Panics
    ///
    /// Panics if `value` is negative.
    pub fn new(value: f64) -> Self {
        if value < 0.0 {
            panic!("Cycles cannot be negative, got {}", value);
        }
        Self(value)
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
