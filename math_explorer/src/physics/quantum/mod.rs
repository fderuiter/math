pub mod coupling;
pub mod fourier;
pub mod schrodinger;
pub mod spin;
pub mod types;

// Re-export key types for convenience
pub use fourier::{dft_operator, idft_operator};
pub use schrodinger::{evolve_state, time_evolution_operator};
pub use spin::{sigma_x, sigma_y, sigma_z};
pub use types::{QuantumOperator, QuantumState};
pub use coupling::clebsch_gordan;
