use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// Energy in Electron Volts (eV).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct ElectronVolts(pub f64);

impl ElectronVolts {
    /// Creates a new `ElectronVolts` instance.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the absolute value.
    pub fn abs(self) -> Self {
        Self(self.0.abs())
    }

    /// Returns the value as `f64`.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for ElectronVolts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4} eV", self.0)
    }
}

impl Add for ElectronVolts {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for ElectronVolts {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul<f64> for ElectronVolts {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self(self.0 * scalar)
    }
}

impl Mul<ElectronVolts> for f64 {
    type Output = ElectronVolts;
    fn mul(self, ev: ElectronVolts) -> ElectronVolts {
        ElectronVolts(self * ev.0)
    }
}

impl Div<f64> for ElectronVolts {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self(self.0 / scalar)
    }
}

impl Div for ElectronVolts {
    type Output = f64;
    fn div(self, other: Self) -> f64 {
        self.0 / other.0
    }
}

impl Neg for ElectronVolts {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

/// Temperature in Kelvin (K).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Kelvin(pub f64);

impl Kelvin {
    /// Creates a new `Kelvin` instance.
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the value as `f64`.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} K", self.0)
    }
}
