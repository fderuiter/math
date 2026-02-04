//! # Computational Biology
//!
//! This module applies mathematical modeling to biological systems, spanning from
//! molecular interactions to population dynamics.
//!
//! ## Hierarchy of Scales
//!
//! The module is organized by biological scale, implementing classic models at each level.
//!
//! ```mermaid
//! graph TD
//!     Pop[Population Scale] -->|Evolutionary Dynamics| Evolution[evolution]
//!     Tissue[Tissue Scale] -->|Reaction-Diffusion| Morph[morphogenesis]
//!     Cell[Cellular Scale] -->|Action Potentials| Neuro[neuroscience]
//!     Mol[Molecular Scale] -->|Enzyme Kinetics| Kinetics[kinetics]
//!
//!     style Pop fill:#ff9,stroke:#333,stroke-width:2px
//!     style Tissue fill:#bbf,stroke:#333,stroke-width:2px
//!     style Cell fill:#dfd,stroke:#333,stroke-width:2px
//!     style Mol fill:#fdd,stroke:#333,stroke-width:2px
//! ```
//!
//! ## Submodules
//!
//! - **Molecular**: [`kinetics`] - Michaelis-Menten enzyme kinetics.
//! - **Cellular**: [`neuroscience`] - Hodgkin-Huxley model for action potentials.
//! - **Tissue**: [`morphogenesis`] - Turing reaction-diffusion systems for pattern formation.
//! - **Population**: [`evolution`] - Evolutionary game theory (Hawk-Dove).

/// Michaelis-Menten enzyme kinetics.
pub mod kinetics;

/// Hodgkin-Huxley neuron model.
pub mod neuroscience;

/// Turing reaction-diffusion systems.
pub mod morphogenesis;

/// Spatial diffusion strategies.
pub mod diffusion;

/// Evolutionary game theory models.
pub mod evolution;
