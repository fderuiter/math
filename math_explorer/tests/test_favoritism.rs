use math_explorer::applied::favoritism::{
    calculate_favoritism_score, FavoritismCalculator, FavoritismInputs,
};

#[test]
fn test_favoritism_defaults() {
    #[allow(deprecated)]
    let score = calculate_favoritism_score(&FavoritismInputs::default());
    // Since there is a random factor 0.9..1.1, the score will vary, but should be positive.
    assert!(score > 0.0);
    assert!(score.is_finite());
}

#[test]
fn test_favoritism_calculator_deterministic() {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    let inputs = FavoritismInputs::default();

    let mut rng = StdRng::seed_from_u64(42);
    let mut calculator = FavoritismCalculator::new(rng);
    let score1 = calculator.calculate(&inputs);

    let mut rng2 = StdRng::seed_from_u64(42);
    let mut calculator2 = FavoritismCalculator::new(rng2);
    let score2 = calculator2.calculate(&inputs);

    assert_eq!(score1, score2, "Scores should be identical with same seed");
}
