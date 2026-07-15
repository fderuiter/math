//! Legacy crate.
pub mod ai;
#[allow(missing_docs)]
pub mod error;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::ai::*;
    pub use crate::error::*;
}
