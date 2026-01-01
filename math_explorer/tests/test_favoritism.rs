use math_explorer::applied::favoritism::{calculate_favoritism_score, FavoritismInputs};
use math_explorer::applied::favoritism::scoring::FavoritismCalculator;
use rand::SeedableRng;

#[test]
fn test_favoritism_defaults() {
    let inputs = FavoritismInputs::default();
    #[allow(deprecated)]
    let score = calculate_favoritism_score(&inputs);
    // Since there is a random factor 0.9..1.1, the score will vary, but should be positive.
    assert!(score > 0.0);
    assert!(score.is_finite());
}

#[test]
fn test_favoritism_deterministic() {
    let inputs = FavoritismInputs::default();
    // Use a fixed seed for deterministic behavior
    let rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut calculator = FavoritismCalculator::new(rng);

    let score1 = calculator.calculate(&inputs);

    // Create another calculator with the same seed
    let rng2 = rand::rngs::StdRng::seed_from_u64(42);
    let mut calculator2 = FavoritismCalculator::new(rng2);
    let score2 = calculator2.calculate(&inputs);

    assert_eq!(score1, score2, "Scores should be identical with the same seed");

    // Ensure the score is within reasonable bounds (sanity check)
    assert!(score1 > 0.0);
}
