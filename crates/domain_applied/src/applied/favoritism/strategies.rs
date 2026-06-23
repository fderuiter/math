use super::types::FavoritismInputs;
use nalgebra::{DMatrix, DVector};
use pure_math::pure_math::analysis::integration::Integrator;
use rand::{Rng, RngCore};

const EPSILON: f64 = 1e-9;

/// Context provided to scoring strategies during evaluation.
///
/// We use `dyn RngCore` to be object-safe regarding the RNG, but we must stay generic
/// over `I` because `Integrator` is not object-safe.
pub struct ScoringContext<'a, I: Integrator + ?Sized> {
    pub rng: &'a mut dyn RngCore,
    pub integrator: &'a I,
    pub safe_x0: f64,
    pub safe_t: f64,
}

impl<'a, I: Integrator + ?Sized> ScoringContext<'a, I> {
    pub fn new(inputs: &FavoritismInputs, rng: &'a mut dyn RngCore, integrator: &'a I) -> Self {
        let safe_x0 = if inputs.time.x_0.abs() < EPSILON {
            EPSILON
        } else {
            inputs.time.x_0
        };
        let safe_t = inputs.time.t.max(0.0);

        Self {
            rng,
            integrator,
            safe_x0,
            safe_t,
        }
    }
}

/// The standard model for the Unified Favoritism Theory.
///
/// This struct composes multiple `ScoringStrategy` components to calculate
/// the final score. It allows for extension by modifying the list of strategies.
pub struct UnifiedFavoritismModel<I: Integrator + ?Sized> {
    pub numerator_strategies: Vec<Box<dyn ScoringStrategy<I>>>,
    pub denominator_strategies: Vec<Box<dyn ScoringStrategy<I>>>,
}

impl<I: Integrator + ?Sized> Default for UnifiedFavoritismModel<I> {
    fn default() -> Self {
        Self {
            numerator_strategies: vec![
                Box::new(ProximityStrategy),
                Box::new(EmotionalSupportStrategy),
                Box::new(GiftStrategy),
                Box::new(ComplimentStrategy),
                Box::new(ContactFrequencyStrategy),
                Box::new(PersonalityStrategy),
                Box::new(SocialMultiplierStrategy),
                Box::new(DecayStrategy),
                Box::new(StochasticStrategy),
            ],
            denominator_strategies: vec![Box::new(SiblingCompetitionStrategy)],
        }
    }
}

impl<I: Integrator + ?Sized> UnifiedFavoritismModel<I> {
    /// Calculates the favoritism score using the composed strategies.
    pub fn calculate(
        &self,
        inputs: &FavoritismInputs,
        rng: &mut dyn RngCore,
        integrator: &I,
    ) -> f64 {
        let mut ctx = ScoringContext::new(inputs, rng, integrator);

        let mut numerator = 1.0;
        for strategy in &self.numerator_strategies {
            numerator *= strategy.calculate(inputs, &mut ctx);
        }

        let mut denominator = 1.0;
        for strategy in &self.denominator_strategies {
            denominator *= strategy.calculate(inputs, &mut ctx);
        }

        numerator / denominator
    }
}

/// A strategy for calculating a component of the favoritism score.
pub trait ScoringStrategy<I: Integrator + ?Sized> {
    fn calculate(&self, inputs: &FavoritismInputs, ctx: &mut ScoringContext<I>) -> f64;
}

// --- Numerator Strategies ---

pub struct ProximityStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for ProximityStrategy {
    fn calculate(&self, _inputs: &FavoritismInputs, ctx: &mut ScoringContext<I>) -> f64 {
        ctx.integrator
            .integrate(|_t| 1.0 / ctx.safe_x0, 0.0, ctx.safe_t, EPSILON)
            .value
    }
}

pub struct EmotionalSupportStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for EmotionalSupportStrategy {
    fn calculate(&self, _inputs: &FavoritismInputs, ctx: &mut ScoringContext<I>) -> f64 {
        ctx.integrator
            .integrate(
                |_t| ctx.integrator.integrate(|_x| 8.0, 0.0, 1.0, EPSILON).value,
                0.0,
                ctx.safe_t,
                EPSILON,
            )
            .value
    }
}

pub struct GiftStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for GiftStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, _ctx: &mut ScoringContext<I>) -> f64 {
        let gift_matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![
            inputs.gifts.g_emotional,
            inputs.gifts.g_practical,
        ]));
        gift_matrix.determinant()
    }
}

pub struct ComplimentStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for ComplimentStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, _ctx: &mut ScoringContext<I>) -> f64 {
        inputs
            .compliments
            .compliments
            .dot(&inputs.compliments.compliment_weights)
    }
}

pub struct ContactFrequencyStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for ContactFrequencyStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, _ctx: &mut ScoringContext<I>) -> f64 {
        (1.0 + inputs.contact.f_initial).max(EPSILON).ln()
    }
}

pub struct PersonalityStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for PersonalityStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, _ctx: &mut ScoringContext<I>) -> f64 {
        inputs.personality.w_i * inputs.personality.intelligence
            + inputs.personality.w_es * inputs.personality.emotional_sensitivity
            + inputs.personality.w_w * inputs.personality.wealth
            + inputs.personality.w_t * inputs.personality.talent
    }
}

pub struct SocialMultiplierStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for SocialMultiplierStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, _ctx: &mut ScoringContext<I>) -> f64 {
        let mut multiplier = inputs.social.birth_order_weight * inputs.social.major_life_events;

        // h: Crisis Multiplier
        if inputs.social.helped_during_crisis {
            multiplier *= 1.5;
        }

        // s: Visibility Multiplier
        if inputs.social.active_on_social_media {
            multiplier *= 1.3;
        }

        multiplier
    }
}

pub struct DecayStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for DecayStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, _ctx: &mut ScoringContext<I>) -> f64 {
        (-inputs.contact.decay_constant * inputs.contact.time_since_last_contact).exp()
    }
}

pub struct StochasticStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for StochasticStrategy {
    fn calculate(&self, _inputs: &FavoritismInputs, ctx: &mut ScoringContext<I>) -> f64 {
        // Rng is implemented for &mut dyn RngCore, so we can call gen_range
        // However, gen_range is in the Rng trait which is not object safe?
        // Wait, gen_range is a default method on Rng.
        // If we have `dyn RngCore`, we can wrap it in something that implements `Rng`?
        // Actually, `dyn RngCore` implements `Rng`.
        ctx.rng.gen_range(0.9..1.1)
    }
}

// --- Denominator Strategies ---

pub struct SiblingCompetitionStrategy;
impl<I: Integrator + ?Sized> ScoringStrategy<I> for SiblingCompetitionStrategy {
    fn calculate(&self, inputs: &FavoritismInputs, ctx: &mut ScoringContext<I>) -> f64 {
        let sibling_proximity_integral = ctx
            .integrator
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
                ctx.safe_t,
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
