pub mod allocator;
pub mod engine;
pub mod metrics;

// Re-export the macro
pub use verified_engine_macros::{Theory, verified, embed_theory};
