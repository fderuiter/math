#[cfg(feature = "ai")]
pub use domain_ai::ai;

#[cfg(feature = "applied")]
pub use domain_applied::applied;

#[cfg(feature = "biology")]
pub use domain_biology::biology;

#[cfg(feature = "climate")]
pub use domain_climate::climate;

#[cfg(feature = "epidemiology")]
pub use domain_epidemiology::epidemiology;

#[cfg(feature = "physics")]
pub use domain_physics::physics;

#[cfg(feature = "pure_math")]
pub use pure_math::pure_math;

pub use math_commons::diagnostics;
pub use math_commons::math_kernel;

// math_explorer/src/error.rs could just export the individual errors, but it's simpler to keep it as it was if possible, or just let users import from specific crates.
