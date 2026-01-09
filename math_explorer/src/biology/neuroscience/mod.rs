//! Neuroscience (Hodgkin-Huxley)
//!
//! This module implements the Hodgkin-Huxley model for a neuron's action potential.
//! The model describes how action potentials in neurons are initiated and propagated.
//! It is a set of nonlinear differential equations that approximates the electrical characteristics
//! of excitable cells such as neurons and cardiac myocytes.
//!
//! The current through the membrane is given by:
//! $$ I = C_m \frac{dV}{dt} + I_{ion} $$
//! where $I_{ion}$ includes Sodium ($Na^+$), Potassium ($K^+$), and Leak ($L$) currents.
//!
//! # Ion Channel Gating
//!
//! The model uses three gating variables to control the flow of ions:
//! - **$m$**: Sodium channel activation (opens quickly when depolarized).
//! - **$h$**: Sodium channel inactivation (closes slowly when depolarized).
//! - **$n$**: Potassium channel activation (opens slowly when depolarized).
//!
//! ```mermaid
//! graph TD
//!     V[Membrane Potential V]
//!     V -->|Controls| AlphaBeta[Rate Constants alpha/beta]
//!     AlphaBeta -->|Update| Gates[Gating Variables m, h, n]
//!     Gates -->|Conductance| Channels[Ion Channels Na, K]
//!     Channels -->|Ionic Currents| Current[I_Na, I_K]
//!     Current -->|Feedback| V
//!     Ext[External Current] --> V
//! ```
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
//!     if neuron.v > 0.0 {
//!         spiked = true;
//!     }
//! }
//! assert!(spiked, "Neuron should have generated an action potential");
//! ```

pub mod types;
pub mod model;
pub mod neuron;

pub use neuron::HodgkinHuxleyNeuron;
pub use types::{HodgkinHuxleyState, HodgkinHuxleyParameters};
pub use model::HodgkinHuxleyModel;
