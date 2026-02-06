//! Neuroscience (Hodgkin-Huxley)
//!
//! This module implements the Hodgkin-Huxley model for a neuron's action potential.
//! The model describes how action potentials in neurons are initiated and propagated.
//!
//! The module is decomposed into:
//! - `types`: Defines the state vector `HodgkinHuxleyState` and parameters `HodgkinHuxleyParameters`.
//! - `model`: Defines the differential equations via `HodgkinHuxleyModel`.
//! - `neuron`: Provides the public `HodgkinHuxleyNeuron` facade.
//!
//! # Example
//!
//! ```rust
//! use math_explorer::biology::neuroscience::HodgkinHuxleyNeuron;
//!
//! let mut neuron = HodgkinHuxleyNeuron::new(-65.0);
//! // Simulate for 10ms with 10 uA/cm^2 current injection
//! let dt = 0.01;
//! let mut spiked = false;
//!
//! for _ in 0..1000 {
//!     neuron.update(dt, 10.0);
//!     if neuron.v() > 0.0 {
//!         spiked = true;
//!     }
//! }
//! assert!(spiked, "Neuron should have generated an action potential");
//! ```

pub mod error;
pub mod model;
pub mod neuron;
pub mod types;

// Re-export the public facade to maintain backward compatibility
pub use neuron::HodgkinHuxleyNeuron;
pub use error::HodgkinHuxleyError;

// Optionally re-export types if we want users to use the advanced API
pub use model::HodgkinHuxleyModel;
pub use types::{HodgkinHuxleyParameters, HodgkinHuxleyState};
