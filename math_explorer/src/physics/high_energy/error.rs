use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum HighEnergyError {
    #[error("Mass must be positive, got {0}")]
    InvalidMass(f64),
    #[error("Radius must be greater than Schwarzschild radius (r > rs), got r={0}, rs={1}")]
    InsideEventHorizon(f64, f64),
    #[error("Adiabatic index must be > 1, got {0}")]
    InvalidAdiabaticIndex(f64),
    #[error("Density must be positive, got {0}")]
    InvalidDensity(f64),
    #[error("Pressure cannot be negative, got {0}")]
    InvalidPressure(f64),
}
