//! # LoraHub: Automated Model Merging
//!
//! LoraHub provides a flexible framework for merging **Low-Rank Adaptation (LoRA)** modules
//! without the need for expensive retraining. It treats LoRA weights as modular components
//! that can be composed to create new model behaviors.
//!
//! ## The Problem
//! Fine-tuning Large Language Models (LLMs) is computationally expensive. While LoRA reduces
//! this cost, users often end up with multiple adapters for different tasks. Manually
//! selecting or switching between them is inefficient.
//!
//! ## The Solution
//! LoraHub allows you to **linearly combine** multiple LoRA adapters into a single one.
//! It uses a **Strategy Pattern** to decouple the combination logic (how to merge)
//! from the objective evaluation (how to score the result).
//!
//! ## Architecture
//!
//! ```mermaid
//! classDiagram
//!     class LoraEnsemble {
//!         +Vec~LoraStateDict~ modules
//!         +combine(weights)
//!         +evaluate(weights, loss)
//!     }
//!
//!     class CombinationStrategy {
//!         <<interface>>
//!         +combine(modules, weights)
//!     }
//!
//!     class ObjectiveStrategy {
//!         <<interface>>
//!         +evaluate(weights, loss)
//!     }
//!
//!     class LinearCombinationStrategy {
//!         +combine()
//!     }
//!
//!     class L1RegularizationStrategy {
//!         +alpha: f64
//!         +evaluate()
//!     }
//!
//!     LoraEnsemble --> CombinationStrategy
//!     LoraEnsemble --> ObjectiveStrategy
//!     LinearCombinationStrategy ..|> CombinationStrategy
//!     L1RegularizationStrategy ..|> ObjectiveStrategy
//! ```
//!
//! ## 🚀 Quick Start
//!
//! Create an ensemble of LoRA modules and merge them using weighted averaging.
//!
//! ```rust
//! use math_explorer::applied::lorahub::{LoraEnsemble, LoraStateDict};
//! use nalgebra::DMatrix;
//! use std::collections::HashMap;
//!
//! // 1. Create dummy LoRA modules (simulating weights for a layer)
//! let mut lora1 = HashMap::new();
//! lora1.insert("layer1".to_string(), DMatrix::from_element(2, 2, 1.0));
//!
//! let mut lora2 = HashMap::new();
//! lora2.insert("layer1".to_string(), DMatrix::from_element(2, 2, 2.0));
//!
//! let modules = vec![lora1, lora2];
//!
//! // 2. Initialize the Ensemble (defaults to Linear Combination)
//! let ensemble = LoraEnsemble::new(modules);
//!
//! // 3. Define weights (e.g., 50% contribution from each)
//! let weights = vec![0.5, 0.5];
//!
//! // 4. Combine
//! let result = ensemble.combine(&weights).expect("Combination failed");
//! let merged_layer = &result["layer1"];
//!
//! // Check: 0.5 * 1.0 + 0.5 * 2.0 = 1.5
//! assert!((merged_layer[(0, 0)] - 1.5).abs() < 1e-6);
//! println!("Merged Weight: {:.2}", merged_layer[(0, 0)]);
//! ```

pub mod ensemble;
pub mod strategies;
pub mod types;

pub use ensemble::LoraEnsemble;
pub use types::LoraStateDict;
