use super::*;
use std::f64::consts::PI;

#[test]
#[verified_engine::verified]
fn test_rating_valid() {
    let r = Rating::new(1500.0).unwrap();
    assert_eq!(r.value(), 1500.0);
}

#[test]
#[verified_engine::verified]
fn test_rating_invalid() {
    assert!(Rating::new(f64::NAN).is_err());
    assert!(Rating::new(f64::INFINITY).is_err());
}

#[test]
#[verified_engine::verified]
fn test_rating_scale_conversion() {
    let r = Rating::new(1500.0).unwrap();
    let mu = r.to_glicko2_scale();
    assert!((mu - 0.0).abs() < 1e-6);

    let r2 = Rating::from_glicko2_scale(mu).unwrap();
    assert!((r2.value() - 1500.0).abs() < 1e-3);
}

#[test]
#[verified_engine::verified]
fn test_rd_valid() {
    let rd = RatingDeviation::new(350.0).unwrap();
    assert_eq!(rd.value(), 350.0);
}

#[test]
#[verified_engine::verified]
fn test_rd_invalid() {
    assert!(RatingDeviation::new(0.0).is_err());
    assert!(RatingDeviation::new(-1.0).is_err());
    assert!(RatingDeviation::new(f64::NAN).is_err());
}

#[test]
#[verified_engine::verified]
fn test_rd_scale_conversion() {
    let rd = RatingDeviation::new(173.7178).unwrap();
    let phi = rd.to_glicko2_scale();
    assert!((phi - 1.0).abs() < 1e-6);

    let rd2 = RatingDeviation::from_glicko2_scale(phi).unwrap();
    assert!((rd2.value() - 173.7178).abs() < 1e-3);
}

#[test]
#[verified_engine::verified]
fn test_volatility_valid() {
    let vol = Volatility::new(0.06).unwrap();
    assert_eq!(vol.value(), 0.06);
}

#[test]
#[verified_engine::verified]
fn test_volatility_invalid() {
    assert!(Volatility::new(0.0).is_err());
    assert!(Volatility::new(-0.1).is_err());
    assert!(Volatility::new(f64::NAN).is_err());
}

#[test]
#[verified_engine::verified]
fn test_system_constant_valid() {
    let tau = SystemConstant::new(0.5).unwrap();
    assert_eq!(tau.value(), 0.5);
}

#[test]
#[verified_engine::verified]
fn test_system_constant_invalid() {
    assert!(SystemConstant::new(0.2).is_err()); // Too small
    assert!(SystemConstant::new(1.5).is_err()); // Too large
    assert!(SystemConstant::new(f64::NAN).is_err());
}

#[test]
#[verified_engine::verified]
fn test_glicko_player_default() {
    let player = GlickoPlayer::default();
    assert_eq!(player.rating.value(), 1500.0);
    assert_eq!(player.rating_deviation.value(), 350.0);
    assert_eq!(player.volatility.value(), 0.06);
}

#[test]
#[verified_engine::verified]
fn test_match_result_valid() {
    let opponent = GlickoPlayer::default();
    assert!(MatchResult::new(opponent, 0.0).is_ok()); // Loss
    assert!(MatchResult::new(opponent, 0.5).is_ok()); // Draw
    assert!(MatchResult::new(opponent, 1.0).is_ok()); // Win
}

#[test]
#[verified_engine::verified]
fn test_match_result_invalid() {
    let opponent = GlickoPlayer::default();
    assert!(MatchResult::new(opponent, -0.1).is_err());
    assert!(MatchResult::new(opponent, 1.1).is_err());
    assert!(MatchResult::new(opponent, f64::NAN).is_err());
}

#[test]
#[verified_engine::verified]
fn test_g_function() {
    // When phi = 0, g(phi) = 1
    assert!((g_function(0.0) - 1.0).abs() < 1e-6);

    // As phi increases, g(phi) decreases
    let g1 = g_function(1.0);
    let g2 = g_function(2.0);
    assert!(g1 > g2);
}

#[test]
#[verified_engine::verified]
fn test_expected_outcome() {
    // Equal ratings and RD should give 0.5 probability
    let e = expected_outcome(0.0, 0.0, 1.0);
    assert!((e - 0.5).abs() < 1e-6);

    // Higher rating should give > 0.5 probability
    let e = expected_outcome(1.0, 0.0, 1.0);
    assert!(e > 0.5);

    // Lower rating should give < 0.5 probability
    let e = expected_outcome(0.0, 1.0, 1.0);
    assert!(e < 0.5);
}
