use nalgebra::{Matrix4, Vector3};
use rand::Rng;
use std::f64::consts::PI;

/// Represents a single ray: r(t) = o + t * d
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

/// A bundle of rays corresponding to an image grid.
pub struct RayBundle {
    pub rays: Vec<Ray>,
    pub width: usize,
    pub height: usize,
}

/// 1.1 Ray Bundle Generation
/// Input: Camera Pose matrix P (4x4), Image resolution (H, W).
/// Output: Ray Bundle (origins and directions).
pub fn generate_ray_bundle(
    pose: &Matrix4<f64>,
    width: usize,
    height: usize,
    fov_y: f64,
) -> RayBundle {
    let mut rays = Vec::with_capacity(width * height);
    let aspect_ratio = width as f64 / height as f64;

    // Assuming standard pinhole camera model
    // NDC coordinates: [-1, 1]

    let half_height = (fov_y / 2.0).tan();
    let half_width = aspect_ratio * half_height;

    // Camera center is the translation part of the pose (assuming pose is camera-to-world)
    // o = P * [0, 0, 0, 1]^T
    let origin = pose.transform_point(&nalgebra::Point3::origin()).coords;

    for y in 0..height {
        for x in 0..width {
            // Normalized pixel coordinates (NDC)
            // Pixel centers
            let u = (x as f64 + 0.5) / width as f64;
            let v = (y as f64 + 0.5) / height as f64;

            // Screen space (assuming -z is forward in camera space)
            // x corresponds to u, y corresponds to v.
            // In computer graphics, typically +y is up.
            // Let's map u [0,1] -> [-half_width, half_width]
            // Let's map v [0,1] -> [-half_height, half_height] (or vice versa depending on y-axis direction)

            let screen_x = (2.0 * u - 1.0) * half_width;
            let screen_y = (1.0 - 2.0 * v) * half_height; // Invert y so 0 is top

            // Direction in camera space: point on screen - origin (0,0,0)
            // Assuming camera looks down -z
            let dir_cam = Vector3::new(screen_x, screen_y, -1.0);

            // Transform direction to world space
            // d = Normalize(P * [x, y, -1, 0]^T) - but P includes translation which we don't want for direction vectors
            // So we use the rotation part of P.
            let dir_world = pose.transform_vector(&dir_cam).normalize();

            rays.push(Ray {
                origin,
                direction: dir_world,
            });
        }
    }

    RayBundle {
        rays,
        width,
        height,
    }
}

/// 1.2 Stratified Sampling
/// Input: Near plane t_n, Far plane t_f, Number of samples N.
/// Output: Sample points along rays.
pub fn stratified_sampling(t_near: f64, t_far: f64, n_samples: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let bin_size = (t_far - t_near) / n_samples as f64;
    let mut samples = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let bin_start = t_near + i as f64 * bin_size;
        let t = bin_start + rng.r#gen::<f64>() * bin_size;
        samples.push(t);
    }
    samples
}

/// 1.3 Positional Encoding
/// gamma(p)
pub fn positional_encoding(p: f64, l: usize) -> Vec<f64> {
    let mut encoded = Vec::with_capacity(2 * l);
    for i in 0..l {
        let freq = 2.0_f64.powi(i as i32) * PI;
        encoded.push((freq * p).sin());
        encoded.push((freq * p).cos());
    }
    encoded
}

/// Trait for the MLP Query
pub trait NeRFModel {
    fn query(&self, pos: &Vector3<f64>, dir: &Vector3<f64>) -> (f64, Vector3<f64>);
}

/// 1.4 Volume Integration
/// Input: Densities, Colors, Interval distances.
/// Output: Rendered Color.
pub fn volume_integration(
    densities: &[f64],
    colors: &[Vector3<f64>],
    deltas: &[f64],
) -> Vector3<f64> {
    let mut final_color = Vector3::zeros();
    let mut transmittance = 1.0;

    for i in 0..densities.len() {
        let sigma = densities[i];
        let color = colors[i];
        let delta = deltas[i];

        let alpha = 1.0 - (-sigma * delta).exp();
        let weight = transmittance * alpha;

        final_color += weight * color;
        transmittance *= 1.0 - alpha;

        if transmittance < 1e-4 {
            break;
        }
    }

    final_color
}

#[cfg(test)]
#[path = "tests_rendering.rs"]
mod tests;
