use crate::ai::sds::rendering::{
    NeRFModel, generate_ray_bundle, render_image, stratified_sampling, volume_integration,
};
use approx::assert_relative_eq;
use nalgebra::{Matrix4, Vector3};
use std::f64::consts::PI;

#[test]
fn test_generate_ray_bundle() {
    let width = 10;
    let height = 10;
    let fov_y = PI / 2.0; // 90 degrees
    let pose = Matrix4::identity();

    let bundle = generate_ray_bundle(&pose, width, height, fov_y);

    assert_eq!(bundle.rays.len(), width * height);

    // Center pixel should have direction roughly (0, 0, -1)
    let center_idx = (height / 2) * width + (width / 2);
    let center_ray = bundle.rays[center_idx];

    assert_relative_eq!(center_ray.origin.x, 0.0);
    assert_relative_eq!(center_ray.origin.y, 0.0);
    assert_relative_eq!(center_ray.origin.z, 0.0);

    // Approx check direction
    assert!(center_ray.direction.z < 0.0);
}

#[test]
fn test_stratified_sampling() {
    let t_near = 0.0;
    let t_far = 10.0;
    let n_samples = 10;

    let samples = stratified_sampling(t_near, t_far, n_samples);
    assert_eq!(samples.len(), n_samples);

    for (i, t) in samples.iter().enumerate().take(n_samples) {
        let bin_start = t_near + i as f64;
        let bin_end = t_near + (i + 1) as f64;
        assert!(*t >= bin_start && *t <= bin_end);
    }
}

#[test]
fn test_volume_integration() {
    // Test with a single opaque sample
    let densities = vec![1000.0]; // Opaque
    let colors = vec![Vector3::new(1.0, 0.0, 0.0)]; // Red
    let deltas = vec![0.1];

    let color = volume_integration(&densities, &colors, &deltas);

    // Should be close to red
    assert!(color.x > 0.9);
    assert!(color.y < 0.1);
    assert!(color.z < 0.1);
}

struct MockNeRF;
impl NeRFModel for MockNeRF {
    fn query(&self, _pos: &Vector3<f64>, _dir: &Vector3<f64>) -> (f64, Vector3<f64>) {
        // Return constant density and color (Green)
        (10.0, Vector3::new(0.0, 1.0, 0.0))
    }
}

#[test]
fn test_render_image() {
    let width = 4;
    let height = 4;
    let fov_y = PI / 2.0;
    let pose = Matrix4::identity();
    let bundle = generate_ray_bundle(&pose, width, height, fov_y);
    let model = MockNeRF;

    let image = render_image(&bundle, &model, 0.1, 2.0, 5);

    assert_eq!(image.nrows(), height);
    assert_eq!(image.ncols(), width);

    // Check a pixel, should be green-ish
    let pixel = image[(0, 0)];
    assert!(pixel.y > 0.8);
    assert!(pixel.x < 0.1);
    assert!(pixel.z < 0.1);
}
