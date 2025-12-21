use math_explorer::applied::clinical_trials::{design, sample_size, hypothesis_testing, analysis, survival_analysis};
use rand::{SeedableRng, rngs::StdRng};
use math_explorer::applied::clinical_trials::design::AllocationStrategy;

#[test]
fn test_simple_randomization() {
    let n = 100;
    let assignments = design::simple_randomization(n);
    assert_eq!(assignments.len(), n);
    // Cannot assert exact split due to randomness, but can check it runs.
}

#[test]
fn test_block_randomization() {
    let n = 20;
    let block_size = 4;
    let assignments = design::block_randomization(n, block_size).unwrap();
    assert_eq!(assignments.len(), n);

    // Check overall balance
    let treatment_count = assignments.iter().filter(|&&g| g == design::Group::Treatment).count();
    let control_count = assignments.iter().filter(|&&g| g == design::Group::Control).count();
    assert_eq!(treatment_count, 10);
    assert_eq!(control_count, 10);
}

#[test]
fn test_sample_size_calculation() {
    // Example: alpha=0.05, power=0.8, delta=5, sigma=10
    // n = 2 * 100 * (1.96 + 0.84)^2 / 25
    // n ~= 200 * 7.84 / 25 ~= 62.72 -> 63
    let n = sample_size::calculate_sample_size_means(0.05, 0.8, 5.0, 10.0).unwrap();
    assert_eq!(n, 63);
}

#[test]
fn test_t_test_independent() {
    let group1 = vec![10.0, 12.0, 11.0, 13.0, 10.5]; // Mean 11.3
    let group2 = vec![15.0, 16.0, 14.0, 15.5, 14.5]; // Mean 15.0

    let result = hypothesis_testing::t_test_independent(&group1, &group2, 0.05).unwrap();
    assert!(result.is_significant);
    assert!(result.p_value < 0.05);
}

#[test]
fn test_chi_square_2x2() {
    // 50 cured, 50 not (Control) vs 70 cured, 30 not (Treatment)
    // Should be significant
    let result = hypothesis_testing::chi_square_2x2(70, 30, 50, 50, 0.05).unwrap();
    assert!(result.is_significant);
}

#[test]
fn test_risk_metrics() {
    // Treatment: 10 events, 90 no events (Risk = 0.1)
    // Control: 20 events, 80 no events (Risk = 0.2)
    // RR = 0.5
    // OR = (10/90) / (20/80) = (1/9) / (1/4) = 4/9 = 0.444...
    let metrics = analysis::calculate_risk_metrics(10, 90, 20, 80, 0.05).unwrap();

    assert!((metrics.relative_risk - 0.5).abs() < 1e-4);
    assert!((metrics.odds_ratio - 0.4444).abs() < 1e-4);
}

#[test]
fn test_kaplan_meier() {
    use survival_analysis::Observation;
    let obs = vec![
        Observation { time: 1.0, event_occurred: true },
        Observation { time: 2.0, event_occurred: true },
        Observation { time: 3.0, event_occurred: false }, // censored
        Observation { time: 4.0, event_occurred: true },
    ];

    let curve = survival_analysis::kaplan_meier(&obs);

    // t=1: 1 event, 4 at risk. S(1) = 1 * (3/4) = 0.75
    // t=2: 1 event, 3 at risk. S(2) = 0.75 * (2/3) = 0.50
    // t=3: 0 event, 2 at risk. S(3) = 0.50 * (2/2) = 0.50
    // t=4: 1 event, 1 at risk. S(4) = 0.50 * (0/1) = 0.0

    assert_eq!(curve[0].time, 1.0);
    assert!((curve[0].survival_probability - 0.75).abs() < 1e-9);

    assert_eq!(curve[1].time, 2.0);
    assert!((curve[1].survival_probability - 0.50).abs() < 1e-9);

    assert_eq!(curve[2].time, 3.0);
    assert!((curve[2].survival_probability - 0.50).abs() < 1e-9);
}

#[test]
fn test_deterministic_randomization() {
    let mut rng = StdRng::seed_from_u64(42);
    let strategy = design::BlockRandomization::new(4);
    let n = 8;

    // With seed 42, we expect a specific shuffle.
    // Block 1: [T, T, C, C] -> shuffled
    // Block 2: [T, T, C, C] -> shuffled

    let assignments = strategy.allocate(n, &mut rng).unwrap();

    assert_eq!(assignments.len(), 8);
    let t_count = assignments.iter().filter(|&&g| g == design::Group::Treatment).count();
    let c_count = assignments.iter().filter(|&&g| g == design::Group::Control).count();

    assert_eq!(t_count, 4);
    assert_eq!(c_count, 4);

    // Verify repeatability
    let mut rng2 = StdRng::seed_from_u64(42);
    let assignments2 = strategy.allocate(n, &mut rng2).unwrap();
    assert_eq!(assignments, assignments2);
}
