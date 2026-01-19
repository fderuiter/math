use super::model::UnifiedFavoritismModel;
use super::types::FavoritismInputs;
use crate::pure_math::analysis::integration::{ClenshawCurtis, Integrator};
use rand::Rng;

/// Calculates the favoritism score based on the provided inputs.
///
/// This function implements the core logic of the Unified Favoritism Theory (UFT).
/// It aggregates various metrics (financial, social, emotional) into a single scalar value.
///
/// **Why this matters:**
/// It provides an objective metric for what is traditionally a subjective and emotionally charged subject.
/// By reducing affection to a floating-point number, we can optimize strategies for inheritance maximization.
///
/// # Arguments
///
/// * `inputs` - A reference to a `FavoritismInputs` struct containing all necessary parameters,
///   including time horizons, gift values, and sibling competition metrics.
///
/// # Returns
///
/// A `f64` representing the calculated favoritism score.
/// * **Higher is better.**
/// * **Range:** Technically $[0, \infty)$, but usually within $[10^6, 10^9]$.
///
/// # Example
///
/// ```
/// use math_explorer::applied::favoritism::{FavoritismInputs, calculate_favoritism_score};
///
/// let mut inputs = FavoritismInputs::default();
/// inputs.social.helped_during_crisis = true; // Crucial factor
///
/// let score = calculate_favoritism_score(&inputs);
/// assert!(score > 0.0);
/// ```
pub fn calculate_favoritism_score(inputs: &FavoritismInputs) -> f64 {
    let mut rng = rand::thread_rng();
    calculate_favoritism_score_with_rng(inputs, &mut rng)
}

/// Calculates the favoritism score using an injected RNG.
///
/// See `calculate_favoritism_score` for details.
pub fn calculate_favoritism_score_with_rng<R: Rng + ?Sized>(
    inputs: &FavoritismInputs,
    rng: &mut R,
) -> f64 {
    calculate_favoritism_score_full(inputs, rng, &ClenshawCurtis)
}

/// Calculates the favoritism score using an injected RNG and Integrator.
///
/// This allows for deterministic testing (via RNG) and flexible integration strategies (via Integrator).
pub fn calculate_favoritism_score_full<R: Rng + ?Sized, I: Integrator + ?Sized>(
    inputs: &FavoritismInputs,
    rng: &mut R,
    integrator: &I,
) -> f64 {
    // We now delegate the calculation to the UnifiedFavoritismModel, which uses
    // the Strategy Pattern to compose the score from individual factors.
    let model = UnifiedFavoritismModel::new();
    model.calculate(inputs, rng, integrator)
}
