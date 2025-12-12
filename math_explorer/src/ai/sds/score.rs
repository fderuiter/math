use nalgebra::DMatrix;

/// 3.1 Text Encoding Stub
/// In a real implementation, this would call CLIP.
pub trait TextEncoder {
    fn encode(&self, text: &str) -> DMatrix<f64>;
}

/// 3.2 U-Net Forward Pass Stub
/// In a real implementation, this would call a U-Net model.
pub trait DiffusionModel {
    /// Predicts noise given noisy latent, timestep, and text embeddings.
    /// Returns predicted noise epsilon.
    fn predict_noise(
        &self,
        latent: &DMatrix<f64>,
        timestep: usize,
        text_embeddings: &DMatrix<f64>,
    ) -> DMatrix<f64>;
}

/// 3.3 Classifier-Free Guidance (CFG)
/// Input: Predicted Noise with text (cond), Predicted Noise without text (uncond).
/// Operation: epsilon_final = epsilon_uncond + s * (epsilon_cond - epsilon_uncond)
pub fn classifier_free_guidance(
    noise_cond: &DMatrix<f64>,
    noise_uncond: &DMatrix<f64>,
    guidance_scale: f64,
) -> DMatrix<f64> {
    noise_uncond + (noise_cond - noise_uncond) * guidance_scale
}

#[cfg(test)]
#[path = "tests_score.rs"]
mod tests;
