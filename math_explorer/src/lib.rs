//! # Math Explorer

pub use oxidize_core::*;

#[cfg(feature = "ai")]
pub use oxidize_ai as ai;

#[cfg(feature = "applied")]
pub use oxidize_applied as applied;

#[cfg(feature = "biology")]
pub use oxidize_biology as biology;

#[cfg(feature = "climate")]
pub use oxidize_climate as climate;

#[cfg(feature = "epidemiology")]
pub use oxidize_epidemiology as epidemiology;

#[cfg(feature = "physics")]
pub use oxidize_physics as physics;

#[cfg(feature = "pure_math")]
pub use oxidize_pure_math as pure_math;

#[cfg(feature = "ai")]
pub use ai::self_calibration;

