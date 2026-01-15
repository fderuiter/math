pub mod glm;
pub mod segmentation;
pub mod surface;
pub mod thickness;

// Re-export specific items to maintain public API compatibility
pub use glm::{estimate_beta, t_statistic};
pub use segmentation::bayesian_classification;
pub use surface::{Surface, evolve_surface, external_energy, internal_energy};
pub use thickness::cortical_thickness;
