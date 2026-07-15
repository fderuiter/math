//! Legacy crate.
#[allow(missing_docs)]
pub mod error;
#[allow(missing_docs)]
pub mod math_types;
pub mod pure_math;
#[allow(missing_docs)]
pub mod theory_macro;

#[doc(hidden)]
pub mod __macro_deps {
    pub use oxidize_core;
    pub use verified_engine;
}
