use math_explorer::physics::medical::optical_motion::{
    physics::{calculate_lambertian_order, calculate_received_power, calculate_photocurrent, calculate_tia_output},
    processing::{weighted_average_height, snr_improvement_factor, LockInAmplifier, calculate_time_delay},
    validation::{percentage_error, root_mean_square_error, pearson_correlation, dice_similarity_coefficient, time_shift_error},
    calibration::LinearCalibrator,
};
use nalgebra::Point3;
use std::f64::consts::PI;

#[test]
fn test_physical_principles() {
    // Lambertian Order
    let phi_half = PI / 3.0; // 60 degrees
    let n = calculate_lambertian_order(phi_half);
    // cos(60) = 0.5. -ln(2) / ln(0.5) = -ln(2) / -ln(2) = 1.0.
    assert!((n - 1.0).abs() < 1e-6);

    // Received Power
    let a = 1e-4; // 1 cm^2 = 1e-4 m^2
    let p_t = 0.1; // 100 mW
    let d = 1.0; // 1 meter
    let phi = 0.0; // 0 degrees
    let theta = 0.0; // 0 degrees
    let gamma = 2.0; // free space
    // n = 1.
    // Pd = (2 * A * Pt) / (2 * pi * 1) * 1 * 1 = (A * Pt) / pi
    // Pd = 1e-5 / pi
    let pd = calculate_received_power(n, a, p_t, d, phi, theta, gamma);
    let expected_pd = (a * p_t) / PI;
    assert!((pd - expected_pd).abs() < 1e-9);

    // Photocurrent
    // i_d = 1e-9 (1 nA), R_pd = 0.5 A/W, Pr = 1e-6 (1 uW)
    let i_d = 1e-9;
    let r_pd = 0.5;
    let p_r = 1e-6;
    let i_pd = calculate_photocurrent(p_r, i_d, r_pd);
    assert!((i_pd - (1e-9 + 0.5e-6)).abs() < 1e-12);

    // TIA Output
    let g_pd = 1e5; // 100 kOhm
    let v_sig = calculate_tia_output(i_pd, g_pd);
    assert!((v_sig - (i_pd * 1e5)).abs() < 1e-9);
}

#[test]
fn test_wah_and_snr() {
    let vertices = vec![
        Point3::new(0.0, 0.0, 10.0),
        Point3::new(1.0, 0.0, 12.0),
        Point3::new(0.0, 1.0, 8.0),
    ];
    let wah = weighted_average_height(&vertices);
    assert!((wah - 10.0).abs() < 1e-6);

    let snr_boost = snr_improvement_factor(400);
    assert!((snr_boost - 20.0).abs() < 1e-6);
}

#[test]
fn test_lock_in_amplifier() {
    let lia = LockInAmplifier::new(1.0, 10.0, 1.0);
    // Ideal case: Signal in phase, phase_diff = 0
    let v_s = 2.0;
    let _v_r = 1.0; // stored in LIA as reference_amplitude
    let (vx, vy) = lia.mix_and_filter(v_s, 0.0);
    // Vx = (2*1)/2 * cos(0) = 1.0
    // Vy = (2*1)/2 * sin(0) = 0.0
    assert!((vx - 1.0).abs() < 1e-6);
    assert!(vy.abs() < 1e-6);

    let (r, theta) = lia.calculate_magnitude_phase(vx, vy);
    assert!((r - 1.0).abs() < 1e-6);
    assert!(theta.abs() < 1e-6);

    // Scaled Output
    // R_scaled = (Vfs/S) * R = (10/1) * 1 = 10.0
    let output = lia.scale_output(r);
    assert!((output - 10.0).abs() < 1e-6);
}

#[test]
fn test_time_delay() {
    // Create a signal and a delayed version
    let sample_rate = 100.0;
    let n = 100;
    let mut signal1 = Vec::new();
    let mut signal2 = Vec::new();

    // Signal: sin wave
    for i in 0..n {
        let t = i as f64 / sample_rate;
        signal1.push((2.0 * PI * 5.0 * t).sin());
        // Delay by 10 samples (0.1s)
        let t_delayed = (i as f64 - 10.0) / sample_rate;
        signal2.push((2.0 * PI * 5.0 * t_delayed).sin());
    }

    // The delay calculation assumes circular shift or padding.
    // For pure sin wave, phase shift is detected.
    // FFT correlation is circular correlation.
    let delay = calculate_time_delay(&signal1, &signal2, sample_rate);
    // Expected delay is 0.1s.
    // Note: Due to noise or edge effects, it might be close.
    assert!((delay - 0.1).abs() < 0.02); // 0.02s tolerance (2 samples)
}

#[test]
fn test_validation_metrics() {
    let measured = vec![1.0, 2.0, 3.0];
    let reference = vec![1.1, 1.9, 3.1];

    // Percentage Error (using single value function for demonstration)
    let pe = percentage_error(measured[0], reference[0]);
    // (1.0 - 1.1) / 1.0 * 100 = -10%
    assert!((pe + 10.0).abs() < 1e-6);

    // RMSE
    // Errors: -0.1, 0.1, -0.1
    // Sq Errors: 0.01, 0.01, 0.01. Mean: 0.01. Sqrt: 0.1
    let rmse = root_mean_square_error(&measured, &reference);
    assert!((rmse - 0.1).abs() < 1e-6);

    // Pearson
    // Perfect correlation despite scaling/shifting?
    // x: 1, 2, 3. y: 1.1, 1.9, 3.1. Line is y = x + noise.
    // It should be high.
    let r = pearson_correlation(&measured, &reference);
    assert!(r > 0.9);

    // Dice
    let mask_a = vec![true, true, false, false];
    let mask_b = vec![true, false, true, false];
    // A: {0, 1}, B: {0, 2}. Int: {0}.
    // Size A: 2, Size B: 2.
    // DSC = 2*1 / (2+2) = 0.5.
    let dsc = dice_similarity_coefficient(&mask_a, &mask_b);
    assert!((dsc - 0.5).abs() < 1e-6);

    // Time Shift Error
    let err = time_shift_error(&measured, &reference);
    // Sum sq diffs: 0.01 + 0.01 + 0.01 = 0.03
    assert!((err - 0.03).abs() < 1e-6);
}

#[test]
fn test_calibration() {
    let voltages = vec![1.0, 2.0, 3.0];
    let distances = vec![10.0, 20.0, 30.0];
    // y = 10x
    let calibrator = LinearCalibrator::fit(&voltages, &distances).unwrap();
    let d = calibrator.calibrate(1.5);
    assert!((d - 15.0).abs() < 1e-6);
}
