use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NuclearError {
    InvalidMassNumber(String),
    InvalidAtomicNumber(String),
    InvalidHalfLife,
    InvalidVelocity,
    InvalidGammaWidth,
    VolumeZero,
}

impl fmt::Display for NuclearError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NuclearError::InvalidMassNumber(msg) => write!(f, "Invalid mass number A: {}", msg),
            NuclearError::InvalidAtomicNumber(msg) => write!(f, "Invalid atomic number Z: {}", msg),
            NuclearError::InvalidHalfLife => write!(f, "Half-life must be positive"),
            NuclearError::InvalidVelocity => write!(f, "Velocity must be positive"),
            NuclearError::InvalidGammaWidth => write!(f, "Gamma width must be positive"),
            NuclearError::VolumeZero => write!(f, "Calculated volume is zero"),
        }
    }
}

impl std::error::Error for NuclearError {}

/// Represents the Atomic Number Z (number of protons).
/// Must be non-negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AtomicNumber(u32);

impl AtomicNumber {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

impl From<u32> for AtomicNumber {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Represents the Mass Number A (total nucleons).
/// Must be positive (>= 1 for any physical nucleus of interest, though 0 is technically handled as error in logic usually).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MassNumber(u32);

impl MassNumber {
    pub fn new(value: u32) -> Result<Self, NuclearError> {
        if value == 0 {
            return Err(NuclearError::InvalidMassNumber(
                "Must be positive".to_string(),
            ));
        }
        Ok(Self(value))
    }

    /// Creates a MassNumber without validation (use with caution).
    pub fn new_unchecked(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

impl TryFrom<u32> for MassNumber {
    type Error = NuclearError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}
