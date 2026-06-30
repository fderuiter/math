use nalgebra::{DMatrix, Matrix4, Vector3};
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

/// Module 1.1: Ray Bundle Generation
/// Input: Camera Pose matrix P (4x4), Image resolution (H, W).
/// Operation: For every pixel (u, v), compute the ray origin o and direction d.
/// Output: Ray Bundle R (origins and directions).
#[verified_engine::verified]
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

/// Module 1.2: Stratified Sampling
/// Input: Ray Bundle R, Near plane t_n, Far plane t_f, Number of samples N.
/// Operation: Divide the ray into N bins and sample a distance t_i uniformly within each bin.
/// Output: Sample points along rays r(t_i) = o + t_i * d.
#[verified_engine::verified]
pub fn stratified_sampling(t_near: f64, t_far: f64, n_samples: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    stratified_sampling_with_rng(t_near, t_far, n_samples, &mut rng)
}

/// Module 1.2: Stratified Sampling using an injected RNG.
#[verified_engine::verified]
pub fn stratified_sampling_with_rng<R: Rng + ?Sized>(
    t_near: f64,
    t_far: f64,
    n_samples: usize,
    rng: &mut R,
) -> Vec<f64> {
    let bin_size = (t_far - t_near) / n_samples as f64;
    let mut samples = Vec::with_capacity(n_samples);

    for i in 0..n_samples {
        let bin_start = t_near + i as f64 * bin_size;
        let t = bin_start + rng.r#gen::<f64>() * bin_size;
        samples.push(t);
    }
    samples
}

/// Module 1.3: MLP Query & Positional Encoding (Helper)
/// Operation 1 (Encoding): Map inputs to higher dimensions using gamma(.) (Fourier features).
#[verified_engine::verified]
pub fn positional_encoding(p: f64, l: usize) -> Vec<f64> {
    let mut encoded = Vec::with_capacity(2 * l);
    for i in 0..l {
        let freq = 2.0_f64.powi(i as i32) * PI;
        encoded.push((freq * p).sin());
        encoded.push((freq * p).cos());
    }
    encoded
}

/// Module 1.3: MLP Query Interface
/// Operation 2 (Inference): Pass through MLP.
/// Output: Raw Density sigma and Color c for every sample point.
pub trait NeRFModel {
    #[verified_engine::verified]
    fn query(&self, pos: &Vector3<f64>, dir: &Vector3<f64>) -> (f64, Vector3<f64>);
}

/// Module 1.4: Volume Integration (Compositing) Helper
/// Input: Densities sigma_i, Colors c_i, Interval distances delta_i.
/// Operation: Compute weights w_i and sum.
#[verified_engine::verified]
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

        // Alpha: alpha_i = 1 - exp(-sigma_i * delta_i)
        let alpha = 1.0 - (-sigma * delta).exp();

        // Weight w_i = T_i * alpha_i
        let weight = transmittance * alpha;

        // Pixel Color accumulation
        final_color += weight * color;

        // Transmittance: T_{i+1} = T_i * (1 - alpha_i)
        transmittance *= 1.0 - alpha;

        if transmittance < 1e-4 {
            break;
        }
    }

    final_color
}

/// Module 1.4: Volume Integration (Full Image)
/// Input: RayBundle, Model, sampling parameters.
/// Output: Rendered Image x_render (HxWx3).
/// Note: Returns a DMatrix of Vector3 for simplicity in this library context,
/// representing the HxW grid of RGB colors.
#[verified_engine::verified]
pub fn render_image<M: NeRFModel + ?Sized>(
    bundle: &RayBundle,
    model: &M,
    t_near: f64,
    t_far: f64,
    n_samples: usize,
) -> DMatrix<Vector3<f64>> {
    let mut rng = rand::thread_rng();
    render_image_with_rng(bundle, model, t_near, t_far, n_samples, &mut rng)
}

/// Module 1.4: Volume Integration (Full Image) using an injected RNG.
#[verified_engine::verified]
pub fn render_image_with_rng<M: NeRFModel + ?Sized, R: Rng + ?Sized>(
    bundle: &RayBundle,
    model: &M,
    t_near: f64,
    t_far: f64,
    n_samples: usize,
    rng: &mut R,
) -> DMatrix<Vector3<f64>> {
    let mut image_data = Vec::with_capacity(bundle.width * bundle.height);

    for ray in &bundle.rays {
        let ts = stratified_sampling_with_rng(t_near, t_far, n_samples, rng);
        let mut densities = Vec::with_capacity(n_samples);
        let mut colors = Vec::with_capacity(n_samples);
        let mut deltas = Vec::with_capacity(n_samples);

        for i in 0..n_samples {
            let t = ts[i];
            let pos = ray.origin + t * ray.direction;

            // Query model (Module 1.3)
            let (sigma, c) = model.query(&pos, &ray.direction);

            densities.push(sigma);
            colors.push(c);

            // Calculate delta (distance to next sample)
            // For the last sample, we can assume a default large distance or same as previous
            let next_t = if i < n_samples - 1 {
                ts[i + 1]
            } else if i > 0 {
                t + (t - ts[i - 1])
            } else {
                t + 1.0 // Default delta for single sample case
            };
            deltas.push(next_t - t);
        }

        let pixel_color = volume_integration(&densities, &colors, &deltas);
        image_data.push(pixel_color);
    }

    DMatrix::from_vec(bundle.height, bundle.width, image_data)
}

#[cfg(test)]
#[path = "tests_rendering.rs"]
mod tests;
