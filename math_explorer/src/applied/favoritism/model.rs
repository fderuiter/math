use super::strategies::*;
use super::types::FavoritismInputs;
use crate::pure_math::analysis::integration::Integrator;
use rand::Rng;

/// The Unified Favoritism Model.
///
/// Implements the Strategy Pattern to calculate favoritism scores by aggregating
/// various scoring factors.
pub struct UnifiedFavoritismModel<R: Rng + ?Sized, I: Integrator + ?Sized> {
    /// Factors that contribute positively (or multiplicatively) to the score.
    pub numerator_factors: Vec<Box<dyn ScoringFactor<R, I>>>,
    /// Factors that divide the score (e.g., competition).
    pub denominator_factors: Vec<Box<dyn ScoringFactor<R, I>>>,
}

impl<R: Rng + ?Sized, I: Integrator + ?Sized> UnifiedFavoritismModel<R, I> {
    /// Creates a new empty model.
    pub fn new() -> Self {
        Self {
            numerator_factors: Vec::new(),
            denominator_factors: Vec::new(),
        }
    }

    /// adds a numerator factor.
    pub fn add_numerator_factor(mut self, factor: Box<dyn ScoringFactor<R, I>>) -> Self {
        self.numerator_factors.push(factor);
        self
    }

    /// adds a denominator factor.
    pub fn add_denominator_factor(mut self, factor: Box<dyn ScoringFactor<R, I>>) -> Self {
        self.denominator_factors.push(factor);
        self
    }

    /// Creates the standard model with all default factors.
    pub fn standard() -> Self {
        Self::new()
            .add_numerator_factor(Box::new(ProximityFactor))
            .add_numerator_factor(Box::new(EmotionalSupportFactor))
            .add_numerator_factor(Box::new(GiftFactor))
            .add_numerator_factor(Box::new(ComplimentFactor))
            .add_numerator_factor(Box::new(FrequencyFactor))
            .add_numerator_factor(Box::new(PersonalityFactor))
            .add_numerator_factor(Box::new(SocialFactor))
            .add_numerator_factor(Box::new(DecayFactor))
            .add_numerator_factor(Box::new(RandomnessFactor))
            .add_denominator_factor(Box::new(SiblingCompetitionFactor))
    }

    /// Calculates the final favoritism score.
    pub fn calculate(
        &self,
        inputs: &FavoritismInputs,
        rng: &mut R,
        integrator: &I,
    ) -> f64 {
        let mut numerator = 1.0;
        for factor in &self.numerator_factors {
            numerator *= factor.calculate(inputs, rng, integrator);
        }

        let mut denominator = 1.0;
        for factor in &self.denominator_factors {
            denominator *= factor.calculate(inputs, rng, integrator);
        }

        // Prevent division by zero if denominator factors result in 0 (though SiblingFactor handles it)
        if denominator.abs() < EPSILON {
            denominator = 1.0;
        }

        numerator / denominator
    }
}
