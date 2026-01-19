use super::strategies::*;
use super::types::FavoritismInputs;
use crate::pure_math::analysis::integration::Integrator;
use rand::Rng;

/// The Unified Favoritism Model.
///
/// This struct aggregates multiple scoring factors to compute the final favoritism score.
/// By using the Strategy Pattern, individual factors can be added, removed, or swapped
/// without modifying the core calculation engine.
pub struct UnifiedFavoritismModel<R: Rng + ?Sized, I: Integrator + ?Sized> {
    /// Factors that positively contribute to the score (multiplicative).
    pub numerator_factors: Vec<Box<dyn ScoringFactor<R, I>>>,
    /// Factors that negatively contribute (divisors) or normalize the score.
    pub denominator_factors: Vec<Box<dyn ScoringFactor<R, I>>>,
}

impl<R: Rng + ?Sized, I: Integrator + ?Sized> UnifiedFavoritismModel<R, I> {
    /// Creates a new model with the standard set of factors defined by the Unified Favoritism Theory.
    pub fn new() -> Self {
        Self {
            numerator_factors: vec![
                Box::new(ProximityFactor),
                Box::new(EmotionalSupportFactor),
                Box::new(GiftFactor),
                Box::new(ComplimentFactor),
                Box::new(ContactFrequencyFactor),
                Box::new(PersonalityFactor),
                Box::new(BirthOrderFactor),
                Box::new(MajorLifeEventsFactor),
                Box::new(CrisisFactor),
                Box::new(SocialMediaFactor),
                Box::new(DecayFactor),
                Box::new(PerturbationFactor),
            ],
            denominator_factors: vec![
                Box::new(SiblingProximityFactor),
            ],
        }
    }

    /// Calculates the final favoritism score using the configured factors.
    pub fn calculate(&self, inputs: &FavoritismInputs, rng: &mut R, integrator: &I) -> f64 {
        let numerator: f64 = self
            .numerator_factors
            .iter()
            .map(|f| f.calculate(inputs, rng, integrator))
            .product();

        let denominator: f64 = self
            .denominator_factors
            .iter()
            .map(|f| f.calculate(inputs, rng, integrator))
            .product();

        if denominator == 0.0 {
            // SiblingProximityFactor guarantees non-zero (defaults to 1.0),
            // but this is a safety fallback for custom models.
            if numerator == 0.0 {
                0.0
            } else {
                f64::INFINITY
            }
        } else {
            numerator / denominator
        }
    }
}

impl<R: Rng + ?Sized, I: Integrator + ?Sized> Default for UnifiedFavoritismModel<R, I> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::integration::ClenshawCurtis;
    use rand::SeedableRng;

    // Define a custom factor for testing extensibility
    struct GrandchildFactor;
    impl<R: Rng + ?Sized, I: Integrator + ?Sized> ScoringFactor<R, I> for GrandchildFactor {
        fn calculate(&self, _inputs: &FavoritismInputs, _rng: &mut R, _integrator: &I) -> f64 {
            2.0 // Grandchildren double the score
        }
    }

    #[test]
    fn test_custom_factor_extension() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let integrator = ClenshawCurtis;
        let inputs = FavoritismInputs::default();

        let mut model = UnifiedFavoritismModel::new();
        let base_score = model.calculate(&inputs, &mut rng, &integrator);

        // Extend the model
        model.numerator_factors.push(Box::new(GrandchildFactor));

        // Reset RNG to ensure same perturbation
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let new_score = model.calculate(&inputs, &mut rng, &integrator);

        // Verification: New score should be exactly 2x base score
        // Use epsilon comparison for float equality
        assert!((new_score - base_score * 2.0).abs() < 1e-6);
    }
}
