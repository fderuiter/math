#[cfg(not(target_arch = "wasm32"))]
pub use rug::{ops, Float, Integer, Rational};

#[cfg(target_arch = "wasm32")]
pub use rug_mock::{ops, Float, Integer, Rational};
// theory_verification!
