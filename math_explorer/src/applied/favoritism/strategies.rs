use super::types::FavoritismInputs;
use crate::pure_math::analysis::integration::Integrator;
use nalgebra::{DMatrix, DVector};
use rand::Rng;

pub const EPSILON: f64 = 1e-9;

/// A strategy for calculating a component of the favoritism score.
///
/// This trait allows decomposing the monolithic scoring formula into isolated, testable factors.
pub trait ScoringFactor<R: Rng + ?Sized, I: Integrator + ?Sized> {
    /// Calculates the factor's contribution to the score.
    ///
    /// For numerator factors, this is a multiplier.
    /// For denominator factors, this is the divisor value.
    fn calculate(&self, inputs: &FavoritismInputs, rng: &mut R, integrator: &I) -> f64;
}

// --- Numerator Factors ---

/// Calculates the proximity integral.
///
/// $\int_0^t \frac{1}{x_0} dt$
pub struct ProximityFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for ProximityFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, integrator: &I) -> f64 {
        let safe_x0 = if inputs.time.x_0.abs() < EPSILON {
            EPSILON
        } else {
            inputs.time.x_0
        };
        let safe_t = inputs.time.t.max(0.0);

        integrator
            .integrate(|_t| 1.0 / safe_x0, 0.0, safe_t, EPSILON)
            .value
    }
}

/// Calculates the emotional support integral.
///
/// Uses a nested integration to simulate "depth" of support (satirically).
pub struct EmotionalSupportFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for EmotionalSupportFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, integrator: &I) -> f64 {
        let safe_t = inputs.time.t.max(0.0);

        integrator
            .integrate(
                |_t| integrator.integrate(|_x| 8.0, 0.0, 1.0, EPSILON).value,
                0.0,
                safe_t,
                EPSILON,
            )
            .value
    }
}

/// Calculates the value of gifts using matrix determinants.
pub struct GiftFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for GiftFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![
            inputs.gifts.g_emotional,
            inputs.gifts.g_practical,
        ]));
        gift_matrix.determinant()
    }
}

/// Calculates the score from compliments.
pub struct ComplimentFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for ComplimentFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        inputs
            .compliments
            .compliments
            .dot(&inputs.compliments.compliment_weights)
    }
}

/// Calculates the contact frequency term.
pub struct FrequencyFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for FrequencyFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        (1.0 + inputs.contact.f_initial).max(EPSILON).ln()
    }
}

/// Calculates the personality score.
pub struct PersonalityFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for PersonalityFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        inputs.personality.w_i * inputs.personality.intelligence
            + inputs.personality.w_es * inputs.personality.emotional_sensitivity
            + inputs.personality.w_w * inputs.personality.wealth
            + inputs.personality.w_t * inputs.personality.talent
    }
}

/// Aggregates various social multipliers (Birth Order, Life Events, Crisis, Social Media).
pub struct SocialFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for SocialFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        let birth_order = inputs.social.birth_order_weight;
        let life_events = inputs.social.major_life_events;

        // h: Crisis Multiplier (Hero Factor)
        let h = if inputs.social.helped_during_crisis {
            1.5
        } else {
            1.0
        };

        // s: Visibility Multiplier (Social Media)
        let s = if inputs.social.active_on_social_media {
            1.3
        } else {
            1.0
        };

        birth_order * life_events * h * s
    }
}

/// Calculates the memory decay factor.
pub struct DecayFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for DecayFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp()
    }
}

/// Adds stochastic perturbation (Parental Mood).
pub struct RandomnessFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for RandomnessFactor {
    fn calculate(&self, _inputs: &FavoritismInputs, rng: &mut R, _integrator: &I) -> f64 {
        rng.gen_range(0.9..1.1)
    }
}

// --- Denominator Factors ---

/// Calculates the sibling competition denominator.
pub struct SiblingCompetitionFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for SiblingCompetitionFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, integrator: &I) -> f64 {
        let safe_t = inputs.time.t.max(0.0);

        let sibling_proximity_integral = integrator
            .integrate(
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
            .value;

        if sibling_proximity_integral.abs() < EPSILON {
            1.0
        } else {
            sibling_proximity_integral
        }
    }
}
