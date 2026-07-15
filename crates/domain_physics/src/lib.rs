//! Legacy crate.
#[allow(missing_docs)]
pub mod error;
pub mod physics;

#[allow(missing_docs)]
pub mod prelude {
    pub use crate::error::*;
    pub use crate::physics::*;
}
