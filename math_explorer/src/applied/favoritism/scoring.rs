use super::types::FavoritismInputs;
use nalgebra::{DMatrix, DVector};
use quadrature::clenshaw_curtis;
use rand::Rng;

// Security / Stability Constants
const EPSILON: f64 = 1e-9;

// Domain Constants
const EMOTIONAL_SUPPORT_BASE_VALUE: f64 = 8.0;
const CRISIS_MULTIPLIER: f64 = 1.5;
const VISIBILITY_MULTIPLIER: f64 = 1.3;
const PERTURBATION_MIN: f64 = 0.9;
const PERTURBATION_MAX: f64 = 1.1;

/// A rigorous calculator for the Satirical Favoritism Model.
///
/// Implements the "Unified Favoritism Theory" by decomposing the scoring logic
/// into discrete, testable components (Strategy / Builder pattern spirit).
pub struct FavoritismCalculator;

impl FavoritismCalculator {
    /// Calculates the total favoritism score with stochastic perturbation.
    ///
    /// # Arguments
    /// * `inputs` - The configuration parameters.
    /// * `rng` - A mutable reference to a Random Number Generator.
    pub fn calculate<R: Rng + ?Sized>(inputs: &FavoritismInputs, rng: &mut R) -> f64 {
        // r: Stochastic Perturbation (Parental Mood)
        // Adds a ±10% random variation to the final score to simulate human unpredictability.
        let r = rng.gen_range(PERTURBATION_MIN..PERTURBATION_MAX);
        Self::calculate_with_perturbation(inputs, r)
    }

    /// Calculates the score with a deterministic perturbation factor.
    ///
    /// Useful for regression testing and theoretical analysis.
    pub fn calculate_with_perturbation(inputs: &FavoritismInputs, perturbation: f64) -> f64 {
        let numerator = Self::calculate_numerator(inputs, perturbation);
        let denominator = Self::calculate_competition_factor(inputs);

        // Prevent division by zero if there are no siblings or integral is zero.
        // If no siblings, assume minimal competition (denominator = 1.0 equivalent).
        if denominator.abs() < EPSILON {
            numerator // Implicitly / 1.0
        } else {
            numerator / denominator
        }
    }

    fn calculate_numerator(inputs: &FavoritismInputs, r: f64) -> f64 {
        let proximity = Self::proximity_score(inputs);
        let emotional = Self::emotional_support_score(inputs);
        let gifts = Self::gift_score(inputs);
        let compliments = Self::compliment_score(inputs);
        let frequency = Self::frequency_score(inputs);
        let personality = Self::personality_score(inputs);
        let social = Self::social_multipliers(inputs);
        let decay = Self::decay_factor(inputs);

        // Order of multiplication preserved from original script to ensure bit-identical results
        proximity
            * emotional
            * gifts
            * compliments
            * frequency
            * personality
            * social // Note: Original mixed social params, we grouped them. Multiplication is associative-ish.
                     // Original: ... * inputs.social.birth_order_weight * inputs.social.major_life_events * h * s * d * r;
                     // Our `social` combines birth_order, life_events, h, s.
                     // Our `decay` is d.
            * decay
            * r
    }

    fn safe_time_params(inputs: &FavoritismInputs) -> (f64, f64) {
        let safe_x0 = if inputs.time.x_0.abs() < EPSILON { EPSILON } else { inputs.time.x_0 };
        let safe_t = inputs.time.t.max(0.0);
        (safe_x0, safe_t)
    }

    fn proximity_score(inputs: &FavoritismInputs) -> f64 {
        let (safe_x0, safe_t) = Self::safe_time_params(inputs);
        clenshaw_curtis::integrate(|_t| 1.0 / safe_x0, 0.0, safe_t, EPSILON).integral
    }

    fn emotional_support_score(inputs: &FavoritismInputs) -> f64 {
        let (_, safe_t) = Self::safe_time_params(inputs);
        // Nested integration preserved for "Theory" compliance
        clenshaw_curtis::integrate(
            |_t| {
                clenshaw_curtis::integrate(|_x| EMOTIONAL_SUPPORT_BASE_VALUE, 0.0, 1.0, EPSILON).integral
            },
            0.0,
            safe_t,
            EPSILON,
        )
        .integral
    }

    fn gift_score(inputs: &FavoritismInputs) -> f64 {
        let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![inputs.gifts.g_emotional, inputs.gifts.g_practical]));
        gift_matrix.determinant()
    }

    fn compliment_score(inputs: &FavoritismInputs) -> f64 {
        inputs.compliments.compliments.dot(&inputs.compliments.compliment_weights)
    }

    fn frequency_score(inputs: &FavoritismInputs) -> f64 {
        // Ensure log input is positive
        (1.0 + inputs.contact.f_initial).max(EPSILON).ln()
    }

    fn personality_score(inputs: &FavoritismInputs) -> f64 {
        inputs.personality.w_i * inputs.personality.intelligence
            + inputs.personality.w_es * inputs.personality.emotional_sensitivity
            + inputs.personality.w_w * inputs.personality.wealth
            + inputs.personality.w_t * inputs.personality.talent
    }

    fn social_multipliers(inputs: &FavoritismInputs) -> f64 {
        // h: Crisis Multiplier (Hero Factor)
        let h = if inputs.social.helped_during_crisis { CRISIS_MULTIPLIER } else { 1.0 };

        // s: Visibility Multiplier (Social Media)
        let s = if inputs.social.active_on_social_media { VISIBILITY_MULTIPLIER } else { 1.0 };

        inputs.social.birth_order_weight * inputs.social.major_life_events * h * s
    }

    fn decay_factor(inputs: &FavoritismInputs) -> f64 {
        // d: Decay Factor (Memory Loss)
        (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp()
    }

    fn calculate_competition_factor(inputs: &FavoritismInputs) -> f64 {
        let (_, safe_t) = Self::safe_time_params(inputs);

        clenshaw_curtis::integrate(
            |_t| {
                inputs
                    .family
                    .sibling_distances
                    .iter()
                    .map(|distance| {
                        // Prevent division by zero for individual sibling distances
                        let safe_distance = if distance.abs() < EPSILON { EPSILON } else { *distance };
                        1.0 / safe_distance
                    })
                    .sum()
            },
            0.0,
            safe_t,
            EPSILON,
        )
        .integral
    }
}

/// Calculates the favoritism score based on the provided inputs.
///
/// The formula combines integrals of proximity and emotional support, gift value,
/// compliments, personality traits, and various other factors, scaled by a
/// random perturbation `r`.
///
/// # Arguments
///
/// * `inputs` - A reference to a `FavoritismInputs` struct containing all necessary parameters.
///
/// # Returns
///
/// A `f64` representing the calculated favoritism score. Higher is better.
///
/// # Deprecation
///
/// This function is kept for backward compatibility. New code should use
/// `FavoritismCalculator::calculate`.
pub fn calculate_favoritism_score(inputs: &FavoritismInputs) -> f64 {
    let mut rng = rand::thread_rng();
    FavoritismCalculator::calculate(inputs, &mut rng)
}
