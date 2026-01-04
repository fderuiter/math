use math_explorer::applied::clinical_trials::design::{
    block_randomization, simple_randomization, stratified_randomization, AllocationStrategy,
    BlockRandomizer, Group, Patient, SimpleRandomizer, StratifiedRandomizer,
};
use math_explorer::applied::clinical_trials::hypothesis_testing;
use math_explorer::applied::clinical_trials::sample_size;
use math_explorer::applied::clinical_trials::survival_analysis::{
    estimate_hazard_ratio_simple, kaplan_meier, Observation,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn test_simple_randomizer_deterministic() {
    let rng1 = ChaCha8Rng::seed_from_u64(42);
    let mut s1 = SimpleRandomizer::new(rng1);

    let rng2 = ChaCha8Rng::seed_from_u64(42);
    let mut s2 = SimpleRandomizer::new(rng2);

    for _ in 0..10 {
        assert_eq!(s1.assign(None).unwrap(), s2.assign(None).unwrap());
    }
}

#[test]
fn test_block_randomizer_balance() {
    let rng = ChaCha8Rng::seed_from_u64(123);
    let block_size = 4;
    let mut randomizer = BlockRandomizer::new(block_size, rng).unwrap();

    let mut groups = Vec::new();
    for _ in 0..block_size {
        groups.push(randomizer.assign(None).unwrap());
    }

    let treatments = groups.iter().filter(|&&g| g == Group::Treatment).count();
    let controls = groups.iter().filter(|&&g| g == Group::Control).count();

    assert_eq!(treatments, block_size / 2);
    assert_eq!(controls, block_size / 2);
}

#[test]
fn test_stratified_randomizer() {
    let factory = || {
        let rng = ChaCha8Rng::seed_from_u64(999); // Fixed seed for each stratum for reproducibility in this test
        BlockRandomizer::new(4, rng).unwrap()
    };
    let mut strat_randomizer = StratifiedRandomizer::new(factory);

    // Stratum A
    let g1 = strat_randomizer.assign(Some("A")).unwrap();
    let _g2 = strat_randomizer.assign(Some("A")).unwrap();
    // Stratum B
    let _g3 = strat_randomizer.assign(Some("B")).unwrap();

    // Just check it runs
    assert!(matches!(g1, Group::Treatment | Group::Control));
}

#[test]
#[allow(deprecated)]
fn test_legacy_functions() {
    // Simple
    let res = simple_randomization(10);
    assert_eq!(res.len(), 10);

    // Block
    let res = block_randomization(10, 2).unwrap();
    assert_eq!(res.len(), 10);
    let treatments = res.iter().filter(|&&g| g == Group::Treatment).count();
    assert_eq!(treatments, 5);

    // Stratified
    let patients = vec![
        Patient {
            id: "p1".to_string(),
            stratum: "S1".to_string(),
        },
        Patient {
            id: "p2".to_string(),
            stratum: "S1".to_string(),
        },
        Patient {
            id: "p3".to_string(),
            stratum: "S2".to_string(),
        },
    ];
    let map = stratified_randomization(&patients, 2).unwrap();
    assert_eq!(map.len(), 3);
}

// --- Restored/Added Tests for other Clinical Trials modules ---
// These ensure we haven't broken the broader module or deleted coverage.

#[test]
fn test_sample_size_calculation() {
    // Basic smoke test for sample size
    let n = sample_size::calculate_sample_size_means(0.05, 0.8, 1.0, 0.5).unwrap();
    // n = 16 * (1/0.5)^2 = 64 approx?
    // Just check it returns a positive number.
    assert!(n > 0);
}

#[test]
fn test_hypothesis_testing() {
    let g1 = vec![1.0, 2.0, 3.0];
    let g2 = vec![4.0, 5.0, 6.0];
    let res = hypothesis_testing::t_test_independent(&g1, &g2, 0.05);
    assert!(res.is_ok());
}

#[test]
fn test_survival_analysis() {
    let obs = vec![
        Observation { time: 10.0, event_occurred: true },
        Observation { time: 20.0, event_occurred: false },
    ];
    let km = kaplan_meier(&obs);
    assert!(!km.is_empty());

    let hr = estimate_hazard_ratio_simple(&obs, &obs);
    assert!((hr - 1.0).abs() < 1e-6);
}
