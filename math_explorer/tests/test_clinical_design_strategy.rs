use math_explorer::applied::clinical_trials::design::{
    stratified_randomization_with_rng, AllocationStrategy, BlockRandomization, Group, Patient,
    SimpleRandomization,
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[test]
fn test_simple_randomization_deterministic() {
    let mut rng1 = ChaCha8Rng::seed_from_u64(42);
    let mut rng2 = ChaCha8Rng::seed_from_u64(42);

    let strategy = SimpleRandomization::new();

    let result1 = strategy.allocate(&mut rng1, 10).unwrap();
    let result2 = strategy.allocate(&mut rng2, 10).unwrap();

    assert_eq!(result1, result2, "Same seed should produce same assignments");
}

#[test]
fn test_block_randomization_balance() {
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    let block_size = 4;
    let strategy = BlockRandomization::new(block_size).unwrap();

    // Allocate 12 patients (3 full blocks)
    let assignments = strategy.allocate(&mut rng, 12).unwrap();
    assert_eq!(assignments.len(), 12);

    let t_count = assignments
        .iter()
        .filter(|&&g| g == Group::Treatment)
        .count();
    let c_count = assignments.iter().filter(|&&g| g == Group::Control).count();

    assert_eq!(t_count, 6);
    assert_eq!(c_count, 6);
}

#[test]
fn test_block_randomization_invalid_size() {
    let res = BlockRandomization::new(3);
    assert!(res.is_err());

    let res_zero = BlockRandomization::new(0);
    assert!(res_zero.is_err());
}

#[test]
fn test_stratified_randomization_swappable_strategy() {
    let patients = vec![
        Patient {
            id: "p1".to_string(),
            stratum: "A".to_string(),
        },
        Patient {
            id: "p2".to_string(),
            stratum: "A".to_string(),
        },
        Patient {
            id: "p3".to_string(),
            stratum: "B".to_string(),
        },
        Patient {
            id: "p4".to_string(),
            stratum: "B".to_string(),
        },
    ];

    let mut rng = ChaCha8Rng::seed_from_u64(999);

    // 1. Use Block Strategy
    let block_strategy = BlockRandomization::new(2).unwrap();
    let block_res =
        stratified_randomization_with_rng(&patients, &block_strategy, &mut rng).unwrap();

    // With block size 2, each stratum (size 2) should be balanced (1T, 1C)
    let a_groups: Vec<_> = block_res
        .iter()
        .filter(|(id, _)| patients.iter().find(|p| &p.id == *id).unwrap().stratum == "A")
        .map(|(_, g)| *g)
        .collect();
    let b_groups: Vec<_> = block_res
        .iter()
        .filter(|(id, _)| patients.iter().find(|p| &p.id == *id).unwrap().stratum == "B")
        .map(|(_, g)| *g)
        .collect();

    let a_t = a_groups.iter().filter(|&&g| g == Group::Treatment).count();
    let b_t = b_groups.iter().filter(|&&g| g == Group::Treatment).count();

    assert_eq!(a_t, 1);
    assert_eq!(b_t, 1);

    // 2. Use Simple Strategy (swapped in!)
    let simple_strategy = SimpleRandomization::new();
    let simple_res =
        stratified_randomization_with_rng(&patients, &simple_strategy, &mut rng).unwrap();
    assert_eq!(simple_res.len(), 4);
}
