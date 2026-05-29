#[cfg(not(target_arch = "wasm32"))]
pub use rug::{Float, Integer, Rational, ops};

#[cfg(target_arch = "wasm32")]
pub use rug_mock::{Float, Integer, Rational, ops};
