use super::types::FavoritismInputs;
use crate::pure_math::analysis::integration::{ClenshawCurtis, Integrator};
use nalgebra::{DMatrix, DVector};
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
    // SECURITY: Input validation to prevent Division by Zero and Infinity propagation.
    const EPSILON: f64 = 1e-9;

    // Ensure x_0 is not zero or too close to it.
    let safe_x0 = if inputs.time.x_0.abs() < EPSILON {
        EPSILON
    } else {
        inputs.time.x_0
    };
    // Ensure t is non-negative.
    let safe_t = inputs.time.t.max(0.0);

    // r: Stochastic Perturbation (Parental Mood)
    // Adds a ±10% random variation to the final score to simulate human unpredictability.
    let r = rng.gen_range(0.9..1.1);

    let proximity_integral = integrator
        .integrate(|_t| 1.0 / safe_x0, 0.0, safe_t, EPSILON)
        .value;

    let emotional_support_integral = integrator
        .integrate(
            |_t| {
                integrator
                    .integrate(|_x| 8.0, 0.0, 1.0, EPSILON)
                    .value
            },
            0.0,
            safe_t,
            EPSILON,
        )
        .value;

    let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![
        inputs.gifts.g_emotional,
        inputs.gifts.g_practical,
    ]));
    let gift_matrix_determinant = gift_matrix.determinant();

    let compliment_score = inputs
        .compliments
        .compliments
        .dot(&inputs.compliments.compliment_weights);

    // Ensure log input is positive
    let frequency_term = (1.0 + inputs.contact.f_initial).max(EPSILON).ln();

    let personality_score = inputs.personality.w_i * inputs.personality.intelligence
        + inputs.personality.w_es * inputs.personality.emotional_sensitivity
        + inputs.personality.w_w * inputs.personality.wealth
        + inputs.personality.w_t * inputs.personality.talent;

    // h: Crisis Multiplier (Hero Factor)
    // Helping during a crisis boosts the score by 50%.
    let h = if inputs.social.helped_during_crisis {
        1.5
    } else {
        1.0
    };

    // s: Visibility Multiplier (Social Media)
    // Publicly praising parents boosts the score by 30%.
    let s = if inputs.social.active_on_social_media {
        1.3
    } else {
        1.0
    };

    // d: Decay Factor (Memory Loss)
    // The memory of good deeds decays exponentially over time without contact.
    let d = (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp();

    let sibling_proximity_integral = integrator
        .integrate(
            |_t| {
                inputs
                    .family
                    .sibling_distances
                    .iter()
                    .map(|distance| {
                        // Prevent division by zero for individual sibling distances
                        let safe_distance = if distance.abs() < EPSILON {
                            EPSILON
                        } else {
                            *distance
                        };
                        1.0 / safe_distance
                    })
                    .sum()
            },
            0.0,
            safe_t,
            EPSILON,
        )
        .value;

    let numerator = proximity_integral
        * emotional_support_integral
        * gift_matrix_determinant
        * compliment_score
        * frequency_term
        * personality_score
        * inputs.social.birth_order_weight
        * inputs.social.major_life_events
        * h
        * s
        * d
        * r;

    // Prevent division by zero if there are no siblings or integral is zero.
    // If no siblings, assume minimal competition (denominator = 1.0 equivalent).
    let denominator = if sibling_proximity_integral.abs() < EPSILON {
        1.0
    } else {
        sibling_proximity_integral
    };

    numerator / denominator
}
