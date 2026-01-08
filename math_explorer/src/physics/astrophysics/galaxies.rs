//! # Empirical Dependencies for Irregular Dwarf Galaxies
//!
//! This module provides an implementation of the empirical formulas for irregular dwarf galaxies
//! as described in the paper by Tillaboev, Tadjibaev, and Otojanova (2025).
//!
//! # Refactoring Note
//! This module uses the **Strategy Pattern** and **Strong Types** to ensure type safety and extensibility.
//! The `GalaxyModel` trait defines the interface for calculations, implemented by specific strategies
//! like `GeneralIrregular`, `TypeCode10`, and `TypeCode95To99`.
//!
//! Legacy functions are preserved for backward compatibility but are deprecated.

// --- Strong Types ---

/// Distance in Megaparsecs (Mpc).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mpc(pub f64);

impl Mpc {
    /// Returns the value as `f64`.
    pub fn as_f64(&self) -> f64 { self.0 }
}

/// Logarithm of the mass in solar masses (log(M/M_Sun)).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SolarMassLog(pub f64);

impl SolarMassLog {
    /// Returns the value as `f64`.
    pub fn as_f64(&self) -> f64 { self.0 }
}

/// Astronomical Magnitude (Apparent or Absolute).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Magnitude(pub f64);

impl Magnitude {
    /// Returns the value as `f64`.
    pub fn as_f64(&self) -> f64 { self.0 }
}

/// Redshift (z).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Redshift(pub f64);

impl Redshift {
    /// Returns the value as `f64`.
    pub fn as_f64(&self) -> f64 { self.0 }
}

// --- GalaxyModel Trait ---

/// Defines the empirical relationships for a specific galaxy morphology.
pub trait GalaxyModel {
    /// Calculates log(Mass) from Distance.
    fn log_mass_from_distance(&self, d: Mpc) -> SolarMassLog;

    /// Calculates Apparent Magnitude (B-band) from Distance.
    fn apparent_magnitude_from_distance(&self, d: Mpc) -> Option<Magnitude>;

    /// Calculates log(Mass) from Absolute Magnitude (V-band).
    fn log_mass_from_absolute_magnitude(&self, m: Magnitude) -> SolarMassLog;

    /// Calculates Redshift from log(Mass).
    fn redshift_from_log_mass(&self, m: SolarMassLog) -> Redshift;
}

// --- Strategies ---

/// Strategy for "All" irregular dwarf galaxies (General Group).
#[derive(Debug, Clone, Copy, Default)]
pub struct GeneralIrregular;

impl GalaxyModel for GeneralIrregular {
    fn log_mass_from_distance(&self, d: Mpc) -> SolarMassLog {
        SolarMassLog(0.0230 * d.0 + 0.7840)
    }

    fn apparent_magnitude_from_distance(&self, d: Mpc) -> Option<Magnitude> {
        Some(Magnitude(0.0206 * d.0 + 16.0010))
    }

    fn log_mass_from_absolute_magnitude(&self, m: Magnitude) -> SolarMassLog {
        SolarMassLog(-0.6670 * m.0 - 1.4975)
    }

    fn redshift_from_log_mass(&self, m: SolarMassLog) -> Redshift {
        Redshift(0.0094 * m.0 - 0.7270)
    }
}

/// Strategy for Type Code 10 irregular dwarf galaxies.
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeCode10;

impl GalaxyModel for TypeCode10 {
    fn log_mass_from_distance(&self, d: Mpc) -> SolarMassLog {
        SolarMassLog(0.0250 * d.0 + 7.6860)
    }

    fn apparent_magnitude_from_distance(&self, d: Mpc) -> Option<Magnitude> {
        Some(Magnitude(0.0140 * d.0 + 16.575))
    }

    fn log_mass_from_absolute_magnitude(&self, m: Magnitude) -> SolarMassLog {
        // Uses the same formula as the general case for this relationship
        SolarMassLog(-0.6670 * m.0 - 1.4975)
    }

    fn redshift_from_log_mass(&self, m: SolarMassLog) -> Redshift {
        Redshift(0.00093 * m.0 - 0.0716)
    }
}

/// Strategy for Type Code 9.5 - 9.9 irregular dwarf galaxies.
#[derive(Debug, Clone, Copy, Default)]
pub struct TypeCode95To99;

impl GalaxyModel for TypeCode95To99 {
    fn log_mass_from_distance(&self, d: Mpc) -> SolarMassLog {
        SolarMassLog(0.0504 * d.0 + 7.5715)
    }

    fn apparent_magnitude_from_distance(&self, _d: Mpc) -> Option<Magnitude> {
        None // Relationship not established in the paper
    }

    fn log_mass_from_absolute_magnitude(&self, m: Magnitude) -> SolarMassLog {
        SolarMassLog(-0.3837 * m.0 - 2.2864)
    }

    fn redshift_from_log_mass(&self, m: SolarMassLog) -> Redshift {
        Redshift(0.0031 * m.0 - 0.0223)
    }
}

// --- Legacy API ---

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

// Implement GalaxyModel for GalaxyType to delegate to the appropriate strategy.
// This allows the Enum to be used where the Trait is expected, and simplifies legacy functions.
impl GalaxyModel for GalaxyType {
    fn log_mass_from_distance(&self, d: Mpc) -> SolarMassLog {
        match self {
            GalaxyType::All => GeneralIrregular.log_mass_from_distance(d),
            GalaxyType::TypeCode10 => TypeCode10.log_mass_from_distance(d),
            GalaxyType::TypeCode9_5To9_9 => TypeCode95To99.log_mass_from_distance(d),
        }
    }

    fn apparent_magnitude_from_distance(&self, d: Mpc) -> Option<Magnitude> {
        match self {
            GalaxyType::All => GeneralIrregular.apparent_magnitude_from_distance(d),
            GalaxyType::TypeCode10 => TypeCode10.apparent_magnitude_from_distance(d),
            GalaxyType::TypeCode9_5To9_9 => TypeCode95To99.apparent_magnitude_from_distance(d),
        }
    }

    fn log_mass_from_absolute_magnitude(&self, m: Magnitude) -> SolarMassLog {
        match self {
            GalaxyType::All => GeneralIrregular.log_mass_from_absolute_magnitude(m),
            GalaxyType::TypeCode10 => TypeCode10.log_mass_from_absolute_magnitude(m),
            GalaxyType::TypeCode9_5To9_9 => TypeCode95To99.log_mass_from_absolute_magnitude(m),
        }
    }

    fn redshift_from_log_mass(&self, m: SolarMassLog) -> Redshift {
        match self {
            GalaxyType::All => GeneralIrregular.redshift_from_log_mass(m),
            GalaxyType::TypeCode10 => TypeCode10.redshift_from_log_mass(m),
            GalaxyType::TypeCode9_5To9_9 => TypeCode95To99.redshift_from_log_mass(m),
        }
    }
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
#[deprecated(note = "Use GalaxyModel::log_mass_from_distance with strong types instead")]
pub fn calculate_log_mass_from_distance(distance: f64, galaxy_type: &GalaxyType) -> f64 {
    galaxy_type.log_mass_from_distance(Mpc(distance)).0
}

/// Calculates the apparent B-band magnitude of a galaxy based on its distance.
///
/// # Arguments
/// * `distance` - The distance to the galaxy in Megaparsecs (Mpc).
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated apparent magnitude, or `None` if the formula is not applicable for the given type.
#[deprecated(note = "Use GalaxyModel::apparent_magnitude_from_distance with strong types instead")]
pub fn calculate_apparent_magnitude_from_distance(distance: f64, galaxy_type: &GalaxyType) -> Option<f64> {
    galaxy_type.apparent_magnitude_from_distance(Mpc(distance)).map(|m| m.0)
}

/// Calculates the logarithm of a galaxy's mass (in solar masses) based on its absolute V-band magnitude.
///
/// # Arguments
/// * `absolute_magnitude_v` - The absolute magnitude of the galaxy in the V-band.
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated logarithm of the mass.
#[deprecated(note = "Use GalaxyModel::log_mass_from_absolute_magnitude with strong types instead")]
pub fn calculate_log_mass_from_absolute_magnitude(absolute_magnitude_v: f64, galaxy_type: &GalaxyType) -> f64 {
    galaxy_type.log_mass_from_absolute_magnitude(Magnitude(absolute_magnitude_v)).0
}

/// Calculates the redshift (z) of a galaxy based on the logarithm of its mass.
///
/// # Arguments
/// * `log_mass_solar` - The logarithm of the galaxy's mass in solar masses.
/// * `galaxy_type` - The morphological type of the galaxy.
///
/// # Returns
/// The calculated redshift.
#[deprecated(note = "Use GalaxyModel::redshift_from_log_mass with strong types instead")]
pub fn calculate_redshift_from_log_mass(log_mass_solar: f64, galaxy_type: &GalaxyType) -> f64 {
    galaxy_type.redshift_from_log_mass(SolarMassLog(log_mass_solar)).0
}
