//! Legacy crate.
#[allow(missing_docs)]
pub mod allocator;
#[allow(missing_docs)]
pub mod engine;
#[allow(missing_docs)]
pub mod metrics;

// Re-export the macro
pub use verified_engine_macros::{Theory, embed_theory, verified};
