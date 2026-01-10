use math_explorer::applied::favoritism::{
    calculate_favoritism_score, calculate_favoritism_score_with_rng, FavoritismInputs,
};
use rand::SeedableRng;
use rand::rngs::StdRng;

#[test]
fn test_favoritism_defaults() {
    let inputs = FavoritismInputs::default();
    let score = calculate_favoritism_score(&inputs);
    // Since there is a random factor 0.9..1.1, the score will vary, but should be positive.
    assert!(score > 0.0);
    assert!(score.is_finite());
}

#[test]
fn test_favoritism_deterministic() {
    let inputs = FavoritismInputs::default();
    // Use a fixed seed for deterministic testing
    let mut rng = StdRng::seed_from_u64(42);

    let score1 = calculate_favoritism_score_with_rng(&inputs, &mut rng);

    // Reset RNG to same state
    let mut rng2 = StdRng::seed_from_u64(42);
    let score2 = calculate_favoritism_score_with_rng(&inputs, &mut rng2);

    assert_eq!(score1, score2, "Scores should be identical with same seed");

    // Verify it's in the expected range
    assert!(score1 > 0.0);

    // Verify changes in input affect the score
    let mut inputs_favored = inputs.clone();
    inputs_favored.gifts.g_emotional = 10.0; // Maximum emotional gift

    let mut rng3 = StdRng::seed_from_u64(42);
    let score_favored = calculate_favoritism_score_with_rng(&inputs_favored, &mut rng3);

    assert!(score_favored > score1, "Better gifts should yield higher score");
}
