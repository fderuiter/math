pub mod surface;
pub mod segmentation;
pub mod thickness;
pub mod glm;

// Re-export specific items to maintain public API compatibility
pub use surface::{Surface, internal_energy, external_energy, evolve_surface};
pub use segmentation::bayesian_classification;
pub use thickness::cortical_thickness;
pub use glm::{estimate_beta, t_statistic};
