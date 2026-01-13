use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StandardModelError {
    #[error("Energy scales must be positive, got mu={0}, q={1}")]
    InvalidEnergyScale(f64, f64),
    #[error("Coupling constant must be positive, got {0}")]
    InvalidCoupling(f64),
    #[error("Landau pole encountered: coupling diverges at this scale")]
    LandauPole,
}
