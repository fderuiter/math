use domain_physics::error::RadarError;
use domain_physics::physics::medical::radar_gating::super_resolution::MusicEstimator;
use num_complex::Complex;

#[test]
#[verified_engine::verified]
fn test_music_invalid_construction() {
    // 0 samples
    assert!(matches!(
        MusicEstimator::new(0, 10, 1),
        Err(RadarError::InvalidConfiguration(_))
    ));

    // 0 smoothing
    assert!(matches!(
        MusicEstimator::new(64, 0, 1),
        Err(RadarError::InvalidConfiguration(_))
    ));

    // 0 targets
    assert!(matches!(
        MusicEstimator::new(64, 10, 0),
        Err(RadarError::InvalidConfiguration(_))
    ));

    // targets >= samples
    assert!(matches!(
        MusicEstimator::new(64, 10, 64),
        Err(RadarError::InvalidConfiguration(_))
    ));
}

#[test]
#[verified_engine::verified]
fn test_music_nan_handling() {
    let mut estimator = MusicEstimator::new(64, 10, 1).unwrap();

    // Inject NaN into snapshots
    let nan_chirp = vec![Complex::new(f64::NAN, 0.0); 64];

    // Fill with NaNs
    for _ in 0..10 {
        estimator.add_snapshot(&nan_chirp).unwrap();
    }

    // This should detect NaNs in eigenvalues and return error
    let result = estimator.compute_spectrum(0.0, 1.0, 0.1, 1e9, 3e8);

    match result {
        Err(RadarError::NumericalInstability(_)) => (), // Pass
        Err(e) => panic!("Expected NumericalInstability, got {:?}", e),
        Ok(_) => panic!("Expected NumericalInstability, got Ok"),
    }
}

#[test]
#[verified_engine::verified]
fn test_music_invalid_compute_args() {
    let mut estimator = MusicEstimator::new(64, 10, 1).unwrap();
    // Add dummy data
    for _ in 0..10 {
        estimator
            .add_snapshot(&vec![Complex::new(0.0, 0.0); 64])
            .unwrap();
    }

    // Step <= 0
    assert!(matches!(
        estimator.compute_spectrum(0.0, 1.0, 0.0, 1e9, 3e8),
        Err(RadarError::InvalidConfiguration(_))
    ));

    // Start > End
    assert!(matches!(
        estimator.compute_spectrum(1.0, 0.0, 0.1, 1e9, 3e8),
        Err(RadarError::InvalidConfiguration(_))
    ));
}
