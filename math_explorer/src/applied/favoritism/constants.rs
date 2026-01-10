/// Epsilon for numerical stability to avoid division by zero.
pub const EPSILON: f64 = 1e-9;

/// Minimum random perturbation factor (90%).
pub const RANDOM_PERTURBATION_MIN: f64 = 0.9;

/// Maximum random perturbation factor (110%).
pub const RANDOM_PERTURBATION_MAX: f64 = 1.1;

/// Multiplier for helping during a crisis (Hero Factor).
pub const CRISIS_MULTIPLIER: f64 = 1.5;

/// Multiplier for social media activity (Visibility Factor).
pub const SOCIAL_MEDIA_MULTIPLIER: f64 = 1.3;

/// Integral bound for emotional support calculation.
pub const EMOTIONAL_SUPPORT_VALUE: f64 = 8.0;

/// Default denominator if no siblings or zero integral.
pub const DEFAULT_DENOMINATOR: f64 = 1.0;
