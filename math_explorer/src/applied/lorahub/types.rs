use nalgebra::DMatrix;
use std::collections::HashMap;

/// Represents a LoRA state dictionary as a map from tensor names to matrices.
///
/// We alias this to `DMatrix<f64>` for now. In a fully generic version,
/// this would be `DMatrix<T>`.
pub type LoraStateDict = HashMap<String, DMatrix<f64>>;
