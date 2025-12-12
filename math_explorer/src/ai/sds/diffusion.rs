use rand::Rng;
use rand_distr::{Distribution, Normal};
use nalgebra::DMatrix;

const DEFAULT_BETA_START: f64 = 0.0001;
const DEFAULT_BETA_END: f64 = 0.02;

/// 2.1 Time Sampling
/// Input: None.
/// Operation: Select a random integer timestep t ~ U{1, ..., T}.
pub fn sample_timestep(max_timesteps: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=max_timesteps)
}

/// 2.2 Noise Schedule Lookup
/// Input: Timestep t.
/// Output: Signal scale (sqrt(alpha_bar)), Noise scale (sqrt(1 - alpha_bar)).
pub struct NoiseSchedule {
    pub betas: Vec<f64>,
    pub alphas: Vec<f64>,
    pub alpha_bars: Vec<f64>,
}

impl NoiseSchedule {
    pub fn new(timesteps: usize) -> Self {
        // Linear schedule for betas as an example (e.g. 1e-4 to 0.02)
        let beta_start = DEFAULT_BETA_START;
        let beta_end = DEFAULT_BETA_END;
        let mut betas = Vec::with_capacity(timesteps);
        let mut alphas = Vec::with_capacity(timesteps);
        let mut alpha_bars = Vec::with_capacity(timesteps);
        let mut alpha_bar_acc = 1.0;

        for i in 0..timesteps {
            let t_ratio = i as f64 / (timesteps - 1) as f64;
            let beta = beta_start + t_ratio * (beta_end - beta_start);
            let alpha = 1.0 - beta;
            alpha_bar_acc *= alpha;

            betas.push(beta);
            alphas.push(alpha);
            alpha_bars.push(alpha_bar_acc);
        }

        Self {
            betas,
            alphas,
            alpha_bars,
        }
    }

    pub fn get_scales(&self, t: usize) -> (f64, f64) {
        // t is 1-indexed in the prompt, let's assume 0-indexed internally or handle offset.
        // Prompt: t ~ U{1...T}. Let's say index t-1.
        let idx = if t == 0 { 0 } else { t - 1 };
        let idx = idx.min(self.alpha_bars.len() - 1);

        let alpha_bar = self.alpha_bars[idx];
        let signal_scale = alpha_bar.sqrt();
        let noise_scale = (1.0 - alpha_bar).sqrt();

        (signal_scale, noise_scale)
    }
}

/// 2.3 Noise Injection
/// Input: Rendered Image (flattened or tensor), Noise epsilon.
/// Operation: z_t = signal_scale * x + noise_scale * epsilon
pub fn inject_noise(
    image: &DMatrix<f64>,
    noise: &DMatrix<f64>,
    signal_scale: f64,
    noise_scale: f64,
) -> DMatrix<f64> {
    image * signal_scale + noise * noise_scale
}

/// Helper to generate Gaussian noise matching image dimensions
pub fn generate_noise(rows: usize, cols: usize) -> DMatrix<f64> {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(0.0, 1.0).unwrap();
    DMatrix::from_fn(rows, cols, |_, _| normal.sample(&mut rng))
}

#[cfg(test)]
#[path = "tests_diffusion.rs"]
mod tests;
