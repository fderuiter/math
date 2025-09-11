//! # Empirical Dependencies for Irregular Dwarf Galaxies
//!
//! This module provides an implementation of the empirical formulas for irregular dwarf galaxies
//! as described in the paper by Tillaboev, Tadjibaev, and Otojanova (2025).

/// Represents the morphological type of an irregular dwarf galaxy.
/// The formulas for calculating physical properties can vary based on the type.
#[derive(Debug, PartialEq)]
pub enum GalaxyType {
    /// Represents all irregular dwarf galaxies as a single group.
    All,
    /// Represents irregular dwarf galaxies with a type code of 10.
    TypeCode10,
    /// Represents irregular dwarf galaxies with a type code between 9.5 and 9.9.
    TypeCode9_5To9_9,
}

/// Represents the physical properties of a dwarf galaxy.
/// Not all properties may be known for a given galaxy.
#[derive(Debug, Default)]
pub struct Galaxy {
    /// Distance to the galaxy in Megaparsecs (Mpc).
    pub distance: Option<f64>,
    /// Apparent magnitude in the B-band.
    pub apparent_magnitude_b: Option<f64>,
    /// Absolute magnitude in the V-band.
    pub absolute_magnitude_v: Option<f64>,
    /// Logarithm of the mass, in units of solar masses (log(M/M_Sun)).
    pub log_mass_solar: Option<f64>,
    /// Redshift (z).
    pub redshift: Option<f64>,
}

/// Calculates the logarithm of a galaxy's mass (in solar masses) based on its distance.
///
/// # Arguments
/// * `distance` - The distance to the galaxy in Megaparsecs (Mpc).
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated logarithm of the mass.
pub fn calculate_log_mass_from_distance(distance: f64, galaxy_type: &GalaxyType) -> f64 {
    match galaxy_type {
        GalaxyType::All => 0.0230 * distance + 0.7840,
        GalaxyType::TypeCode10 => 0.0250 * distance + 7.6860,
        GalaxyType::TypeCode9_5To9_9 => 0.0504 * distance + 7.5715,
    }
}

/// Calculates the apparent B-band magnitude of a galaxy based on its distance.
///
/// # Arguments
/// * `distance` - The distance to the galaxy in Megaparsecs (Mpc).
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated apparent magnitude, or `None` if the formula is not applicable for the given type.
pub fn calculate_apparent_magnitude_from_distance(distance: f64, galaxy_type: &GalaxyType) -> Option<f64> {
    match galaxy_type {
        GalaxyType::All => Some(0.0206 * distance + 16.0010),
        GalaxyType::TypeCode10 => Some(0.0140 * distance + 16.575),
        GalaxyType::TypeCode9_5To9_9 => None, // Not provided in the paper
    }
}

/// Calculates the logarithm of a galaxy's mass (in solar masses) based on its absolute V-band magnitude.
///
/// # Arguments
/// * `absolute_magnitude_v` - The absolute magnitude of the galaxy in the V-band.
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated logarithm of the mass.
pub fn calculate_log_mass_from_absolute_magnitude(absolute_magnitude_v: f64, galaxy_type: &GalaxyType) -> f64 {
    match galaxy_type {
        GalaxyType::All | GalaxyType::TypeCode10 => -0.6670 * absolute_magnitude_v - 1.4975,
        GalaxyType::TypeCode9_5To9_9 => -0.3837 * absolute_magnitude_v - 2.2864,
    }
}

/// Calculates the redshift (z) of a galaxy based on the logarithm of its mass.
///
/// # Arguments
/// * `log_mass_solar` - The logarithm of the galaxy's mass in solar masses.
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated redshift.
pub fn calculate_redshift_from_log_mass(log_mass_solar: f64, galaxy_type: &GalaxyType) -> f64 {
    match galaxy_type {
        GalaxyType::All => 0.0094 * log_mass_solar - 0.7270,
        GalaxyType::TypeCode10 => 0.00093 * log_mass_solar - 0.0716,
        GalaxyType::TypeCode9_5To9_9 => 0.0031 * log_mass_solar - 0.0223,
    }
}
