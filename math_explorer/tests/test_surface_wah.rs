use math_explorer::physics::medical::radar_gating::surface::{BiQuadraticSurface, BiQuadraticSurface as BQS};
use nalgebra::Point3;

#[test]
fn test_wah_calculation() {
    // Create a flat mesh at z=10
    let mesh = vec![
        Point3::new(0.0, 0.0, 10.0),
        Point3::new(1.0, 0.0, 10.0),
        Point3::new(0.0, 1.0, 10.0),
        Point3::new(1.0, 1.0, 10.0),
    ];

    // Calculate WAH for all points
    let wah = BQS::weighted_average_height(&mesh, |_| true).unwrap();
    assert_eq!(wah, 10.0);
}

#[test]
fn test_surface_fitting() {
    // Points on plane z = 2x + 3y + 1
    // c0=1, c1=2, c2=3, others 0
    let points = vec![
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(1.0, 0.0, 3.0),
        Point3::new(0.0, 1.0, 4.0),
        Point3::new(1.0, 1.0, 6.0),
        Point3::new(2.0, 0.0, 5.0),
        Point3::new(0.0, 2.0, 7.0),
    ];

    let surface = BiQuadraticSurface::fit(&points).expect("Fit failed");

    // Check coeffs
    // Allow some tolerance
    let c = surface.coefficients;
    assert!((c[0] - 1.0).abs() < 1e-5); // c0
    assert!((c[1] - 2.0).abs() < 1e-5); // c1
    assert!((c[2] - 3.0).abs() < 1e-5); // c2
}
