use super::strategies::UnifiedFavoritismModel;
use super::types::FavoritismInputs;
use super::FavoritismError;
use crate::pure_math::analysis::integration::{ClenshawCurtis, Integrator};
use rand::Rng;

/// Calculates the favoritism score safely, validating inputs first.
///
/// # Returns
/// * `Ok(f64)` - The calculated score.
/// * `Err(FavoritismError)` - If inputs are invalid (NaN, Inf, negative time, etc.).
pub fn try_calculate_favoritism_score(inputs: &FavoritismInputs) -> Result<f64, FavoritismError> {
    inputs.validate()?;
    let mut rng = rand::thread_rng();
    Ok(calculate_favoritism_score_with_rng(inputs, &mut rng))
}

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
#[deprecated(since = "0.2.0", note = "Use try_calculate_favoritism_score instead")]
pub fn calculate_favoritism_score(inputs: &FavoritismInputs) -> f64 {
    try_calculate_favoritism_score(inputs).unwrap_or(f64::NAN)
}

/// Calculates the favoritism score using an injected RNG.
///
/// See `calculate_favoritism_score` for details.
pub fn calculate_favoritism_score_with_rng<R: Rng>(inputs: &FavoritismInputs, rng: &mut R) -> f64 {
    calculate_favoritism_score_full(inputs, rng, &ClenshawCurtis)
}

/// Calculates the favoritism score using an injected RNG and Integrator.
///
/// This allows for deterministic testing (via RNG) and flexible integration strategies (via Integrator).
pub fn calculate_favoritism_score_full<R: Rng, I: Integrator + ?Sized>(
    inputs: &FavoritismInputs,
    rng: &mut R,
    integrator: &I,
) -> f64 {
    let model = UnifiedFavoritismModel::<I>::default();
    model.calculate(inputs, rng, integrator)
}
