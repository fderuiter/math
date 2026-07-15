#[allow(missing_docs)]
pub mod algorithm;
#[allow(missing_docs)]
pub mod kernel;

pub use algorithm::calculate_terma;
pub use kernel::{DoseKernel, ExponentialKernel};

// [cite:mmwave_radiotherapy_setup]
