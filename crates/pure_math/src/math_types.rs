#[cfg(not(target_arch = "wasm32"))]
pub use rug::{Float, Integer, Rational, ops};

#[cfg(target_arch = "wasm32")]
// theory_verification!
pub use rug_mock::{Float, Integer, Rational, ops};
// theory_verification!
