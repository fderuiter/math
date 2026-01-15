use math_explorer::applied::game_theory::mechanism_design::MechanismDesign;
use rand::SeedableRng;
use rand::rngs::StdRng;
use statrs::distribution::Uniform;

#[test]
fn test_revenue_simulation_deterministic() {
    let dist = Uniform::new(0.0, 1.0).unwrap();
    let n_bidders = 2;
    let n_simulations = 100;

    // Seed 1
    let mut rng1 = StdRng::seed_from_u64(12345);
    let rev1 = MechanismDesign::simulate_optimal_revenue_with_rng(
        &dist,
        n_bidders,
        n_simulations,
        &mut rng1,
    );

    // Seed 2
    let mut rng2 = StdRng::seed_from_u64(12345);
    let rev2 = MechanismDesign::simulate_optimal_revenue_with_rng(
        &dist,
        n_bidders,
        n_simulations,
        &mut rng2,
    );

    // Seed 3
    let mut rng3 = StdRng::seed_from_u64(67890);
    let rev3 = MechanismDesign::simulate_optimal_revenue_with_rng(
        &dist,
        n_bidders,
        n_simulations,
        &mut rng3,
    );

    assert_eq!(
        rev1, rev2,
        "Simulations with same seed must return identical revenue"
    );
    assert_ne!(rev1, rev3, "Simulations with different seeds should vary");
}
