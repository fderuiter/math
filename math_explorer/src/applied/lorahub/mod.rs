//! # LoraHub Mathematical Core
//!
//! This module contains the structural refactoring of the LoraHub core.
//! It transforms the original functional implementation into a modular,
//! object-oriented design using the `LoraEnsemble` struct.

pub mod types;
pub mod ensemble;

pub use types::LoraStateDict;
pub use ensemble::LoraEnsemble;

// Backward compatibility or legacy exports?
// To clean up "God File" tendencies, we prefer users to use `LoraEnsemble`.
// But for now, we might not re-export the old functions, forcing a refactor in consumers.
// This is an "Architect" move: breaking API for better structure (within reason).
// However, since we are inside the same crate, we can fix the consumers (`lib.rs`).
