//! # Biology
//!
//! Computational biology models spanning from molecular kinetics to evolutionary game theory.
//!
//! This module is organized by biological scale:
//!
//! ## 🔬 Molecular & Cellular
//! * **[Kinetics](kinetics)**: Michaelis-Menten enzyme kinetics.
//! * **[Neuroscience](neuroscience)**: Hodgkin-Huxley model for neuron action potentials.
//!
//! ## 🧬 Tissue & Organ
//! * **[Morphogenesis](morphogenesis)**: Turing Patterns (Reaction-Diffusion systems) for pattern formation.
//!
//! ## 🌍 Population
//! * **[Evolution](evolution)**: Evolutionary Game Theory (Hawk-Dove) and Replicator Dynamics.
//!
//! ## 🗺️ Overview
//!
//! ```mermaid
//! graph TD
//!     subgraph Molecular["🔬 Molecular"]
//!         K[Enzyme Kinetics]
//!     end
//!     subgraph Cellular["⚡ Cellular"]
//!         N[Hodgkin-Huxley Neuron]
//!     end
//!     subgraph Tissue["🧬 Tissue"]
//!         T[Turing Patterns]
//!     end
//!     subgraph Population["🌍 Population"]
//!         E[Evolutionary Dynamics]
//!     end
//!
//!     K -->|Metabolism| N
//!     N -->|Networking| T
//!     T -->|Phenotypes| E
//! ```

pub mod kinetics;
pub mod neuroscience;
pub mod morphogenesis;
pub mod evolution;
