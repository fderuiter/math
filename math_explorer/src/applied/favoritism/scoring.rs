use super::constants::*;
use super::types::FavoritismInputs;
use nalgebra::{DMatrix, DVector};
use quadrature::clenshaw_curtis;
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

/// Calculates the favoritism score using a provided RNG.
///
/// This allows for deterministic testing by injecting a seeded RNG.
pub fn calculate_favoritism_score_with_rng<R: Rng + ?Sized>(
    inputs: &FavoritismInputs,
    rng: &mut R,
) -> f64 {
    // Ensure x_0 is not zero or too close to it.
    let safe_x0 = if inputs.time.x_0.abs() < EPSILON {
        EPSILON
    } else {
        inputs.time.x_0
    };
    // Ensure t is non-negative.
    let safe_t = inputs.time.t.max(0.0);

    // r: Stochastic Perturbation (Parental Mood)
    let r = rng.gen_range(RANDOM_PERTURBATION_MIN..RANDOM_PERTURBATION_MAX);

    let proximity_score = calculate_proximity_score(safe_x0, safe_t);
    let emotional_score = calculate_emotional_support_score(safe_t);
    let gift_score = calculate_gift_score(inputs);
    let compliment_score = calculate_compliment_score(inputs);
    let frequency_term = calculate_frequency_term(inputs);
    let personality_score = calculate_personality_score(inputs);
    let social_multipliers = calculate_social_multipliers(inputs);
    let decay_factor = calculate_decay_factor(inputs);

    let numerator = proximity_score
        * emotional_score
        * gift_score
        * compliment_score
        * frequency_term
        * personality_score
        * social_multipliers
        * decay_factor
        * r;

    let denominator = calculate_sibling_penalty(inputs, safe_t);

    numerator / denominator
}

fn calculate_proximity_score(safe_x0: f64, safe_t: f64) -> f64 {
    clenshaw_curtis::integrate(|_t| 1.0 / safe_x0, 0.0, safe_t, EPSILON).integral
}

fn calculate_emotional_support_score(safe_t: f64) -> f64 {
    clenshaw_curtis::integrate(
        |_t| {
            clenshaw_curtis::integrate(|_x| EMOTIONAL_SUPPORT_VALUE, 0.0, 1.0, EPSILON).integral
        },
        0.0,
        safe_t,
        EPSILON,
    )
    .integral
}

fn calculate_gift_score(inputs: &FavoritismInputs) -> f64 {
    let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![
        inputs.gifts.g_emotional,
        inputs.gifts.g_practical,
    ]));
    gift_matrix.determinant()
}

fn calculate_compliment_score(inputs: &FavoritismInputs) -> f64 {
    inputs
        .compliments
        .compliments
        .dot(&inputs.compliments.compliment_weights)
}

fn calculate_frequency_term(inputs: &FavoritismInputs) -> f64 {
    (1.0 + inputs.contact.f_initial).max(EPSILON).ln()
}

fn calculate_personality_score(inputs: &FavoritismInputs) -> f64 {
    inputs.personality.w_i * inputs.personality.intelligence
        + inputs.personality.w_es * inputs.personality.emotional_sensitivity
        + inputs.personality.w_w * inputs.personality.wealth
        + inputs.personality.w_t * inputs.personality.talent
}

fn calculate_social_multipliers(inputs: &FavoritismInputs) -> f64 {
    let h = if inputs.social.helped_during_crisis {
        CRISIS_MULTIPLIER
    } else {
        1.0
    };

    let s = if inputs.social.active_on_social_media {
        SOCIAL_MEDIA_MULTIPLIER
    } else {
        1.0
    };

    inputs.social.birth_order_weight * inputs.social.major_life_events * h * s
}

fn calculate_decay_factor(inputs: &FavoritismInputs) -> f64 {
    (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp()
}

fn calculate_sibling_penalty(inputs: &FavoritismInputs, safe_t: f64) -> f64 {
    let sibling_proximity_integral = clenshaw_curtis::integrate(
        |_t| {
            inputs
                .family
                .sibling_distances
                .iter()
                .map(|distance| {
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
    .integral;

    if sibling_proximity_integral.abs() < EPSILON {
        DEFAULT_DENOMINATOR
    } else {
        sibling_proximity_integral
    }
}
