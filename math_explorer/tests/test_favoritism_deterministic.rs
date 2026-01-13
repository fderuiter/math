use math_explorer::applied::favoritism::{calculate_favoritism_score_with_rng, FavoritismInputs};
use rand::rngs::StdRng;
use rand::SeedableRng;

#[test]
fn test_favoritism_deterministic() {
    let inputs = FavoritismInputs::default();

    // Seed 1
    let mut rng1 = StdRng::seed_from_u64(42);
    let score1 = calculate_favoritism_score_with_rng(&inputs, &mut rng1);

    // Seed 2 (same seed)
    let mut rng2 = StdRng::seed_from_u64(42);
    let score2 = calculate_favoritism_score_with_rng(&inputs, &mut rng2);

    // Seed 3 (different seed)
    let mut rng3 = StdRng::seed_from_u64(999);
    let score3 = calculate_favoritism_score_with_rng(&inputs, &mut rng3);

    // Check consistency
    assert_eq!(score1, score2, "Scores with same seed must be identical");

    // Check variation (extremely unlikely to be identical with different seeds due to r factor)
    assert_ne!(score1, score3, "Scores with different seeds should vary");
}
