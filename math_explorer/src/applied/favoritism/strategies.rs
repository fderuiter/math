use super::types::FavoritismInputs;
use crate::pure_math::analysis::integration::Integrator;
use nalgebra::{DMatrix, DVector};
use rand::Rng;

const EPSILON: f64 = 1e-9;

/// A trait for individual factors that contribute to the favoritism score.
///
/// This allows the scoring algorithm to be composed of independent, testable strategies.
pub trait ScoringFactor<R: Rng + ?Sized, I: Integrator + ?Sized> {
    /// Calculates the contribution of this factor.
    fn calculate(&self, inputs: &FavoritismInputs, rng: &mut R, integrator: &I) -> f64;
}

// --- Numerator Factors ---

/// Calculates the impact of physical proximity over time.
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

/// Calculates the impact of emotional support.
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

/// Calculates the impact of gifts (financial and emotional).
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

/// Calculates the impact of compliments.
pub struct ComplimentFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for ComplimentFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        inputs
            .compliments
            .compliments
            .dot(&inputs.compliments.compliment_weights)
    }
}

/// Calculates the impact of contact frequency.
pub struct ContactFrequencyFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for ContactFrequencyFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        (1.0 + inputs.contact.f_initial).max(EPSILON).ln()
    }
}

/// Calculates the impact of personality traits.
pub struct PersonalityFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for PersonalityFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        inputs.personality.w_i * inputs.personality.intelligence
            + inputs.personality.w_es * inputs.personality.emotional_sensitivity
            + inputs.personality.w_w * inputs.personality.wealth
            + inputs.personality.w_t * inputs.personality.talent
    }
}

/// Calculates the impact of birth order.
pub struct BirthOrderFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for BirthOrderFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        inputs.social.birth_order_weight
    }
}

/// Calculates the impact of major life events.
pub struct MajorLifeEventsFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for MajorLifeEventsFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        inputs.social.major_life_events
    }
}

/// Calculates the crisis multiplier (Hero Factor).
pub struct CrisisFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for CrisisFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        if inputs.social.helped_during_crisis {
            1.5
        } else {
            1.0
        }
    }
}

/// Calculates the visibility multiplier (Social Media).
pub struct SocialMediaFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for SocialMediaFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        if inputs.social.active_on_social_media {
            1.3
        } else {
            1.0
        }
    }
}

/// Calculates the decay factor due to lack of contact.
pub struct DecayFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for DecayFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
        (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp()
    }
}

/// Adds stochastic perturbation (Parental Mood).
pub struct PerturbationFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for PerturbationFactor {
    fn calculate(&self, _inputs: &FavoritismInputs, rng: &mut R, _integrator: &I) -> f64 {
        rng.gen_range(0.9..1.1)
    }
}

// --- Denominator Factors ---

/// Calculates the sibling proximity integral (Competition).
pub struct SiblingProximityFactor;

impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for SiblingProximityFactor {
    fn calculate(&self, inputs: &FavoritismInputs, _rng: &mut R, integrator: &I) -> f64 {
        let safe_t = inputs.time.t.max(0.0);

        let val = integrator
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

        // If result is effectively zero (no competition/siblings), return 1.0 to avoid division by zero
        if val.abs() < EPSILON {
            1.0
        } else {
            val
        }
    }
}
