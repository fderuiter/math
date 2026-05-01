#![doc = include_str!("README.md")]

pub mod error;
pub mod kinetics;
pub mod model;
pub mod neuron;
pub mod types;

pub use error::HodgkinHuxleyError;
// Re-export the public facade to maintain backward compatibility
pub use neuron::HodgkinHuxleyNeuron;

// Optionally re-export types if we want users to use the advanced API
pub use kinetics::{GatingKinetics, StandardKinetics};
pub use model::HodgkinHuxleyModel;
pub use types::{HodgkinHuxleyParameters, HodgkinHuxleyState};
