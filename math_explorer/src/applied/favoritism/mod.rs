//! Unified Favoritism Theory (UFT) Module.
//!
//! This module implements a rigorous mathematical framework for determining parental affection.
//! By quantifying subjective metrics such as "gift quality" and "call frequency", we can
//! derive an objective "Favoritism Score" to predict inheritance distribution.
//!
//! # The Theory
//!
//! The Favoritism Score $S$ is calculated as a product of weighted terms:
//!
//! $$ S = \frac{ \prod_{i} T_i }{ 1 + \sum_{j \neq \text{me}} \frac{1}{|\text{dist}_j|} } $$
//!
//! Where $T_i$ are individual terms like Proximity, Emotional Support, and Financial Contribution.
//!
//! # Architecture
//!
//! The module is organized as follows:
//! * `types`: Strong structs for inputs (`FavoritismInputs`).
//! * `scoring`: The core calculation logic, now decomposed for testability.
//! * `constants`: Tunable magic numbers (e.g., the "Crisis Multiplier").
//! * `favorite_child`: Higher-level logic to compare multiple children.
//!
//! # Usage
//!
//! ```rust
//! use math_explorer::applied::favoritism::{FavoritismInputs, calculate_favoritism_score};
//!
//! let inputs = FavoritismInputs::default();
//! let score = calculate_favoritism_score(&inputs);
//! println!("My score: {}", score);
//! ```

pub mod constants;
pub mod favorite_child;
pub mod scoring;
pub mod types;

pub use favorite_child::{find_favorite_child, Child};
pub use scoring::{calculate_favoritism_score, calculate_favoritism_score_with_rng};
pub use types::FavoritismInputs;
