/// Proton mass in MeV/c^2.
pub const PROTON_MASS: f64 = 938.272;
/// Neutron mass in MeV/c^2.
pub const NEUTRON_MASS: f64 = 939.565;
/// Reduced Planck constant times speed of light (hbar * c) in MeV fm.
pub const HBAR_C: f64 = 197.3;
/// Speed of light in fm/s (approx 3.0e23).
pub const LIGHT_SPEED: f64 = 2.99792458e23;
/// Squared elementary charge (e^2) in MeV fm.
/// Derived from fine-structure constant alpha = e^2 / (hbar c) ~ 1/137.036.
pub const E_SQUARED: f64 = 1.439976;

/// Liquid Drop Model Constants
pub mod liquid_drop_constants {
    pub const A_V: f64 = 15.75;
    pub const A_S: f64 = 17.8;
    pub const A_C: f64 = 0.711;
    pub const A_SYM: f64 = 23.7;
    pub const DELTA_COEFF: f64 = 11.18;
}

/// Nuclear Properties Constants
pub mod property_constants {
    pub const R0: f64 = 1.2; // fm
}
