use math_explorer::applied::favoritism::{
    calculate_favoritism_score, try_calculate_favoritism_score, FavoritismInputs,
};

#[test]
fn test_favoritism_nan_input_legacy() {
    let mut inputs = FavoritismInputs::default();
    inputs.time.t = f64::NAN;

    // Legacy function should return NaN now (due to try_calculate wrapper)
    #[allow(deprecated)]
    let score = calculate_favoritism_score(&inputs);
    assert!(score.is_nan(), "Legacy function should return NaN for invalid input");
}

#[test]
fn test_favoritism_nan_input_safe() {
    let mut inputs = FavoritismInputs::default();
    inputs.time.t = f64::NAN;

    let result = try_calculate_favoritism_score(&inputs);
    assert!(result.is_err(), "Safe function should return Err for NaN input");
}

#[test]
fn test_favoritism_inf_input_safe() {
    let mut inputs = FavoritismInputs::default();
    inputs.time.t = f64::INFINITY;

    let result = try_calculate_favoritism_score(&inputs);
    assert!(result.is_err(), "Safe function should return Err for Inf input");
}

#[test]
fn test_favoritism_negative_time_safe() {
    let mut inputs = FavoritismInputs::default();
    inputs.time.t = -10.0;

    let result = try_calculate_favoritism_score(&inputs);
    assert!(result.is_err(), "Safe function should return Err for negative time");
}

#[test]
fn test_favoritism_personality_nan() {
    let mut inputs = FavoritismInputs::default();
    inputs.personality.intelligence = f64::NAN;

    #[allow(deprecated)]
    let score = calculate_favoritism_score(&inputs);
    assert!(score.is_nan(), "Legacy function should return NaN for personality NaN");

    let result = try_calculate_favoritism_score(&inputs);
    assert!(result.is_err(), "Safe function should return Err for personality NaN");
}

#[test]
fn test_favoritism_valid_input() {
    let inputs = FavoritismInputs::default();
    let result = try_calculate_favoritism_score(&inputs);
    assert!(result.is_ok());
    let score = result.unwrap();
    assert!(score.is_finite());
    assert!(score > 0.0);
}
