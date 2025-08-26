use math_explorer::win_ratio::{bmi, sample_win_ratio, probability_win_ratio, simulation};

const FLOAT_TOLERANCE: f64 = 1e-3;

#[test]
fn test_calculate_bmi() {
    let weight_kg = 70.0;
    let height_m = 1.75;
    let expected_bmi = 22.857;
    let calculated_bmi = bmi::calculate_bmi(weight_kg, height_m);
    assert!((calculated_bmi - expected_bmi).abs() < FLOAT_TOLERANCE);

    let weight_kg = 80.0;
    let height_m = 1.60;
    let expected_bmi = 31.25;
    let calculated_bmi = bmi::calculate_bmi(weight_kg, height_m);
    assert!((calculated_bmi - expected_bmi).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_sample_win_ratio() {
    let counts = sample_win_ratio::WinLossCounts::new(10, 20, 5, 15);
    let n_w = counts.n_wins();
    let n_l = counts.n_losses();

    assert_eq!(n_w, 35);
    assert_eq!(n_l, 15);

    // Test win ratio
    let expected_ratio = 35.0 / 15.0;
    let ratio = sample_win_ratio::calculate_sample_win_ratio(&counts);
    assert!((ratio - expected_ratio).abs() < FLOAT_TOLERANCE);

    // Test confidence interval
    let (lower, upper) = sample_win_ratio::calculate_confidence_interval(n_w, n_l);
    let expected_lower = 1.342;
    let expected_upper = 4.781;
    assert!((lower - expected_lower).abs() < FLOAT_TOLERANCE);
    assert!((upper - expected_upper).abs() < FLOAT_TOLERANCE);

    // Test significance test statistic
    let statistic = sample_win_ratio::calculate_significance_test_statistic(n_w, n_l);
    let expected_statistic = 3.086;
    assert!((statistic - expected_statistic).abs() < FLOAT_TOLERANCE);
}

#[test]
fn test_probability_win_ratio_with_simulation() {
    let params = simulation::SimulationParams::new(0.1, 0.2, 0.8, 0.7);
    let c = 5.0;
    let error_tolerance = 1e-6;

    // Theoretical win ratio from simulation parameters
    let expected_pr_w = simulation::win_ratio_parameter(&params);

    // Create closures for the survival and pdf functions from the simulation module
    let s0 = |t: f64| simulation::marginal_survival_t_control(t, &params);
    let s1 = |t: f64| simulation::marginal_survival_t_treatment(t, &params);
    let pdf_t0 = |t: f64| simulation::pdf_t_control(t, &params);
    let pdf_t1 = |t: f64| simulation::pdf_t_treatment(t, &params);

    let g0_given_c = |x: f64| simulation::conditional_survival_x_given_t_control(x, c, &params);
    let g1_given_c = |x: f64| simulation::conditional_survival_x_given_t_treatment(x, c, &params);
    let pdf_x0_given_c = |x: f64| simulation::pdf_x_given_t_control(x, c, &params);
    let pdf_x1_given_c = |x: f64| simulation::pdf_x_given_t_treatment(x, c, &params);

    let s0_at_c = s0(c);
    let s1_at_c = s1(c);

    // Calculate win and loss probabilities using numerical integration
    let win_prob = probability_win_ratio::calculate_win_probability(
        s1, pdf_t0, g1_given_c, pdf_x0_given_c, s0_at_c, s1_at_c, c, error_tolerance
    );
    let loss_prob = probability_win_ratio::calculate_loss_probability(
        s0, pdf_t1, g0_given_c, pdf_x1_given_c, s0_at_c, s1_at_c, c, error_tolerance
    );

    // Calculate probability win ratio
    let calculated_pr_c = probability_win_ratio::calculate_probability_win_ratio(win_prob, loss_prob);

    // The probability win ratio PR(c) should be close to the theoretical parameter PR_W
    // when c is large enough for the integrals to stabilize.
    // The value might not be exactly the same due to the time limit c and numerical precision.
    // Let's assert they are reasonably close.
    // A tolerance of 0.1 might be needed depending on the choice of c and parameters.
    let comparison_tolerance = 0.1;
    assert!((calculated_pr_c - expected_pr_w).abs() < comparison_tolerance,
            "Calculated PR(c) = {}, Expected PR_W = {}", calculated_pr_c, expected_pr_w);
}
