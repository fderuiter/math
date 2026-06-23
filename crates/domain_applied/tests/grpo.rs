

use domain_applied::applied::grpo::formulas::*;

#[test]
fn test_response_level_advantage() {
    let rewards = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let advantage = response_level_advantage(&rewards, 3.0);
    assert!((advantage - 0.0).abs() < 1e-9);

    let advantage_high = response_level_advantage(&rewards, 5.0);
    assert!(advantage_high > 0.0);

    let advantage_low = response_level_advantage(&rewards, 1.0);
    assert!(advantage_low < 0.0);
}

#[test]
fn test_clipped_surrogate_objective() {
    let pi_thetas = vec![1.1, 1.3];
    let pi_theta_olds = vec![1.0, 1.0];
    let advantages = vec![2.0, 2.0];
    let epsilon = 0.2;
    let beta = 0.1;
    let kl_divergence = 0.5;

    // Calculation for first element: min(1.1 * 2.0, 1.1 * 2.0) = 2.2
    // Calculation for second element: min(1.3 * 2.0, 1.2 * 2.0) = 2.4
    // sum = 2.2 + 2.4 = 4.6
    // G = 2
    // L_GRPO = -(1/2) * 4.6 + 0.1 * 0.5 = -2.3 + 0.05 = -2.25
    let objective = clipped_surrogate_objective(
        &pi_thetas,
        &pi_theta_olds,
        &advantages,
        epsilon,
        beta,
        kl_divergence,
    );
    assert!((objective - (-2.25)).abs() < 1e-9);
}

#[test]
fn test_uncertainty_reward() {
    let reward_max = uncertainty_reward(0.5);
    assert!((reward_max - 1.0).abs() < 1e-9);

    let reward_min = uncertainty_reward(0.0);
    assert!((reward_min - 0.0).abs() < 1e-9);

    let reward_min_2 = uncertainty_reward(1.0);
    assert!((reward_min_2 - 0.0).abs() < 1e-9);
}

#[test]
fn test_pairwise_distance_bleu() {
    let dist = pairwise_distance_bleu("hello world", "hello world");
    assert!((dist - 0.0).abs() < 1e-9);

    let dist_diff = pairwise_distance_bleu("hello world", "goodbye world");
    assert!(dist_diff > 0.0);
}

#[test]
fn test_repetition_penalty() {
    let penalty = repetition_penalty(10, 100, 1.0);
    assert!((penalty - 0.1).abs() < 1e-9);
}

#[test]
fn test_composite_reward() {
    let reward = composite_reward(0.8, 0.1);
    assert!((reward - 0.7).abs() < 1e-9);

    let reward_zero = composite_reward(0.1, 0.8);
    assert!((reward_zero - 0.0).abs() < 1e-9);
}

#[test]
fn test_binary_reward() {
    assert_eq!(binary_reward(true), 1);
    assert_eq!(binary_reward(false), 0);
}

#[test]
fn test_solver_empirical_accuracy() {
    let responses = vec![
        "apple".to_string(),
        "banana".to_string(),
        "apple".to_string(),
    ];
    let accuracy = solver_empirical_accuracy(&responses, "apple");
    assert!((accuracy - 2.0 / 3.0).abs() < 1e-9);

    let accuracy_no_match = solver_empirical_accuracy(&responses, "orange");
    assert!((accuracy_no_match - 0.0).abs() < 1e-9);
}

#[test]
fn test_kl_divergence_lower_bound() {
    let bound = kl_divergence_lower_bound(0.5, 1.0);
    assert!((bound - 0.125).abs() < 1e-9);

    let bound_zero_p = kl_divergence_lower_bound(0.0, 1.0);
    assert!((bound_zero_p - 0.0).abs() < 1e-9);
}
