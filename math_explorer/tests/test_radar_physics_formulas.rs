use math_explorer::physics::medical::radar_gating::physics::FmcwConfig;

#[test]
fn test_fmcw_formulas() {
    let config = FmcwConfig::iwr6843_default();

    // Test Beat Frequency
    // f_b = S * 2R / c
    // S = 4e9 / 50e-6 = 80e12 Hz/s
    // R = 1.0 m
    // f_b = 80e12 * 2.0 / 3e8 = 160e12 / 3e8 = 533,333 Hz
    let beat = config.beat_frequency(1.0);
    assert!((beat - 533698.0).abs() < 1000.0, "Expected ~533kHz, got {}", beat);

    // Test Max Unambiguous Velocity
    // v_max = lambda / (4 * Tc)
    // lambda = 3e8 / 60e9 = 0.005 m (5 mm)
    // Tc = 50e-6
    // v_max = 0.005 / 200e-6 = 25 m/s
    let v_max = config.max_unambiguous_velocity();
    assert!((v_max - 25.0).abs() < 0.1, "Expected ~25 m/s, got {}", v_max);
}

#[test]
fn test_dielectric_phase_delay() {
    // d = 0.002 m (2mm mask)
    // lambda = 0.005 m
    // epsilon_r = 2.5 (plastic)
    // delay = 4pi * 0.002 / 0.005 * (sqrt(2.5) - 1)
    // = 1.6pi * (1.58 - 1) = 1.6pi * 0.58 ~ 2.9 rad
    let delay = math_explorer::physics::medical::radar_gating::physics::dielectric_phase_delay(0.002, 0.005, 2.5);
    assert!(delay > 0.0);
}
