//! # LoraHub: Modular Weight Merging
//!
//! LoraHub is a framework for composable Large Language Model (LLM) adaptation.
//! It allows users to merge multiple Low-Rank Adaptation (LoRA) modules into a single
//! unified model using various combination strategies.
//!
//! Unlike simple averaging, LoraHub employs a **Strategy Pattern** to decouple
//! the *data* (the weights) from the *logic* (how they are combined and evaluated).
//!
//! ## Architecture
//!
//! ```mermaid
//! classDiagram
//!     class LoraEnsemble {
//!         -modules: Vec~LoraStateDict~
//!         +combine(weights: &[f64])
//!         +evaluate(weights: &[f64])
//!     }
//!     class CombinationStrategy {
//!         <<interface>>
//!         +combine(modules, weights)
//!     }
//!     class ObjectiveStrategy {
//!         <<interface>>
//!         +evaluate(weights, mock_loss)
//!     }
//!     class LinearCombinationStrategy
//!     class L1RegularizationStrategy
//!
//!     LoraEnsemble o-- CombinationStrategy
//!     LoraEnsemble o-- ObjectiveStrategy
//!     CombinationStrategy <|.. LinearCombinationStrategy
//!     ObjectiveStrategy <|.. L1RegularizationStrategy
//! ```
//!
//! ## 🚀 Quick Start
//!
//! Create an ensemble of dummy LoRA modules and combine them.
//!
//! ```rust
//! use math_explorer::applied::lorahub::{LoraEnsemble, LoraStateDict};
//! use nalgebra::DMatrix;
//! use std::collections::HashMap;
//!
//! fn main() {
//!     // 1. Create dummy LoRA modules (Maps of Tensor Name -> Matrix)
//!     let mut module_a = HashMap::new();
//!     module_a.insert("layer1.weight".to_string(), DMatrix::from_element(2, 2, 1.0));
//!
//!     let mut module_b = HashMap::new();
//!     module_b.insert("layer1.weight".to_string(), DMatrix::from_element(2, 2, 2.0));
//!
//!     let modules = vec![module_a, module_b];
//!
//!     // 2. Initialize the Ensemble
//!     // Default strategies: Linear Combination, L1 Regularization
//!     let ensemble = LoraEnsemble::new(modules);
//!
//!     // 3. Combine with weights (e.g., 0.5 from A, 0.5 from B)
//!     let weights = vec![0.5, 0.5];
//!     let combined = ensemble.combine(&weights).expect("Combination failed");
//!
//!     // Result should be 1.0*0.5 + 2.0*0.5 = 1.5
//!     let result_matrix = &combined["layer1.weight"];
//!     assert_eq!(result_matrix[(0, 0)], 1.5);
//!     println!("Combined Matrix:\n{}", result_matrix);
//! }
//! ```

pub mod ensemble;
pub mod strategies;
pub mod types;

pub use ensemble::LoraEnsemble;
pub use types::LoraStateDict;
