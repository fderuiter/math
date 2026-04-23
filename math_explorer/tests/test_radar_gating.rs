use approx::assert_relative_eq;
use math_explorer::physics::medical::radar_gating::{
    gating::GatingLogic,
    geometry::{AngleFftConfig, SphericalPoint},
    physics::{C, FmcwConfig},
    surface::BiQuadraticSurface,
    tracking::{ConstantVelocityModel, TrackingFilter},
};
use nalgebra::Point3;

#[test]
fn test_radar_physics() {
    let config = FmcwConfig::iwr6843_default();

    // Check Range Resolution: c / 2B = 3e8 / 8e9 = 0.0375 m
    let resolution = config.range_resolution();
    assert_relative_eq!(resolution, 0.037474, epsilon = 1e-4);

    // Check Velocity from Phase
    // v = (dPhi * lambda) / (4 * pi * Tc)
    // Let's create a synthetic phase for 1 m/s velocity
    // 1 = (dPhi * lambda) / (4 * pi * Tc)
    // dPhi = (4 * pi * Tc) / lambda
    let lambda = C / 60.0e9;
    let tc = 50.0e-6;
    let expected_dphi = (4.0 * std::f64::consts::PI * tc) / lambda;

    let v = config.velocity_from_phase(expected_dphi);
    assert_relative_eq!(v, 1.0, epsilon = 1e-6);
}

#[test]
fn test_coordinate_transformation() {
    let fft_config = AngleFftConfig {
        n_fft_azimuth: 64,
        n_fft_elevation: 32,
    };

    // Test Point: Range=10m, AzIndex=0, ElIndex=0
    // w_x = 0, w_z = 0
    // x = 0, z = 0, y = 10
    let p0 = SphericalPoint {
        range: 10.0,
        azimuth_index: 0,
        elevation_index: 0,
    };
    let c0 = fft_config.spherical_to_cartesian(&p0);
    assert_relative_eq!(c0.x, 0.0);
    assert_relative_eq!(c0.y, 10.0);
    assert_relative_eq!(c0.z, 0.0);

    // Test Point with angles
    // AzIndex = 16 (=> w_x = 32/64 = 0.5)
    // ElIndex = 0
    // x = 10 * 0.5 = 5.0
    // z = 0
    // y = sqrt(100 - 25 - 0) = sqrt(75) = 8.66...
    let p1 = SphericalPoint {
        range: 10.0,
        azimuth_index: 16,
        elevation_index: 0,
    };
    let c1 = fft_config.spherical_to_cartesian(&p1);
    assert_relative_eq!(c1.x, 5.0);
    assert_relative_eq!(c1.y, (75.0_f64).sqrt());
    assert_relative_eq!(c1.z, 0.0);
}

#[test]
fn test_surface_fitting() {
    // Generate synthetic data on a perfect surface: z = 1 + 0.1x^2 + 0.1y^2
    let mut points = Vec::new();
    for x in -2..=2 {
        for y in -2..=2 {
            let x = x as f64;
            let y = y as f64;
            let z = 1.0 + 0.1 * x.powi(2) + 0.1 * y.powi(2);
            points.push(Point3::new(x, y, z));
        }
    }

    let surface = BiQuadraticSurface::fit(&points).expect("Fitting failed");
    let coeffs = surface.coefficients;

    // z = c0 + c1*x + c2*y + c3*xy + c4*x^2 + c5*y^2
    // Expect: c0=1, c4=0.1, c5=0.1, others 0
    assert_relative_eq!(coeffs[0], 1.0, epsilon = 1e-6); // c0
    assert_relative_eq!(coeffs[1], 0.0, epsilon = 1e-6); // c1
    assert_relative_eq!(coeffs[2], 0.0, epsilon = 1e-6); // c2
    assert_relative_eq!(coeffs[3], 0.0, epsilon = 1e-6); // c3
    assert_relative_eq!(coeffs[4], 0.1, epsilon = 1e-6); // c4
    assert_relative_eq!(coeffs[5], 0.1, epsilon = 1e-6); // c5
}

#[test]
fn test_kalman_tracking() {
    let model = ConstantVelocityModel::new(1e-4, 1e-2);
    let mut filter = TrackingFilter::new(0.0, 0.1, model);

    // Simulate motion: A(t) = 1.0 * t (velocity = 1.0)
    for i in 0..100 {
        filter.predict();
        let true_pos = 1.0 * (i as f64 * 0.1);
        filter.update(true_pos);
    }

    assert_relative_eq!(filter.velocity(), 1.0, epsilon = 0.1);
}

#[test]
fn test_gating_logic() {
    let mut gating = GatingLogic::new(10.0, 1.0, 0.0); // Threshold 10, Hysteresis 1

    // Start below threshold (Beam ON expected)
    // Case 1: Amplitude 8.0 < 9.0 (10-1).
    let is_on = gating.evaluate(8.0, 0.0);
    assert!(is_on);

    // Case 2: Amplitude rises to 10.5. Inside hysteresis [9, 11]. Should stay ON.
    let is_on = gating.evaluate(10.5, 0.0);
    assert!(is_on);

    // Case 3: Amplitude rises to 11.5 > 11.0 (10+1). Should turn OFF.
    let is_on = gating.evaluate(11.5, 0.0);
    assert!(!is_on);

    // Case 4: Amplitude falls to 9.5. Inside hysteresis. Should stay OFF.
    let is_on = gating.evaluate(9.5, 0.0);
    assert!(!is_on);

    // Case 5: Amplitude falls to 8.5 < 9.0. Should turn ON.
    let is_on = gating.evaluate(8.5, 0.0);
    assert!(is_on);
}
