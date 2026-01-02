//! Biology: Modeling Life at Every Scale
//!
//! This module provides mathematical models for biological systems, ranging from
//! single-molecule enzyme kinetics to population-level evolutionary dynamics.
//!
//! # 🌟 Overview
//!
//! We categorize biological modeling into three distinct scales:
//!
//! 1.  **Molecular Scale**: Enzyme kinetics and biochemical reactions.
//! 2.  **Tissue/Cellular Scale**: Neural dynamics and pattern formation (morphogenesis).
//! 3.  **Population Scale**: Evolutionary game theory and species interactions.
//!
//! # 🗺️ Biological Scales
//!
//! ```mermaid
//! graph TD
//!     subgraph Population [Population Scale]
//!         E[Evolutionary Dynamics] -->|Game Theory| HD[Hawk-Dove Game]
//!     end
//!
//!     subgraph Tissue [Tissue & Cellular Scale]
//!         N[Neuroscience] -->|Action Potentials| HH[Hodgkin-Huxley]
//!         M[Morphogenesis] -->|Turing Patterns| RD[Reaction-Diffusion]
//!     end
//!
//!     subgraph Molecular [Molecular Scale]
//!         K[Kinetics] -->|Enzymes| MM[Michaelis-Menten]
//!     end
//!
//!     Molecular -->|Aggregates into| Tissue
//!     Tissue -->|Emerges into| Population
//! ```
//!
//! # 🚀 Quick Start
//!
//! Simulate a simple enzymatic reaction using Michaelis-Menten kinetics:
//!
//! ```rust
//! use math_explorer::biology::kinetics::EnzymeReaction;
//!
//! fn main() {
//!     // Create an enzyme with Vmax = 100.0 and Km = 50.0
//!     let enzyme = EnzymeReaction::new(100.0, 50.0).unwrap();
//!
//!     // Calculate reaction rate at substrate concentration [S] = 10.0
//!     let rate = enzyme.reaction_velocity(10.0).unwrap();
//!
//!     println!("Reaction Rate: {:.2}", rate);
//! }
//! ```

pub mod kinetics;
pub mod neuroscience;
pub mod morphogenesis;
pub mod evolution;
