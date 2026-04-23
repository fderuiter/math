//! Physical constants for High Energy Physics.

/// Speed of light in vacuum (m/s).
pub const C: f64 = 299_792_458.0;

/// Gravitational constant (m^3 kg^-1 s^-2).
pub const G: f64 = 6.674_30e-11;

/// Solar Mass (kg).
pub const SOLAR_MASS: f64 = 1.989e30;

/// Thomson Cross Section (m^2).
/// Value derived from approx 6.6524e-25 cm^2.
pub const SIGMA_T: f64 = 6.6524e-29;

/// Small epsilon for floating point comparisons.
pub const EPSILON: f64 = 1e-10;
