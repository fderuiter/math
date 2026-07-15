use nalgebra::DMatrix;

/// Module 3.1: Text Encoding
/// Input: Text Prompt y.
/// Operation: Pass text through CLIP Text Encoder.
/// Output: Text Embedding Vectors e_text.
pub trait TextEncoder {
    #[verified_engine::verified]
    #[allow(missing_docs)]
    fn encode(&self, text: &str) -> DMatrix<f64>;
}

/// Module 3.2: U-Net Forward Pass
/// Input: Noisy Latent z_t, Timestep embedding t, Text Embedding e_text.
/// Operation: The U-Net predicts the noise component.
/// Output: Predicted Noise Tensor epsilon_hat.
pub trait DiffusionModel {
    /// Predicts noise given noisy latent, timestep, and text embeddings.
    /// Returns predicted noise epsilon.
    #[verified_engine::verified]
    fn predict_noise(
        &self,
        latent: &DMatrix<f64>,
        timestep: usize,
        text_embeddings: &DMatrix<f64>,
    ) -> DMatrix<f64>;
}

/// Module 3.3: Classifier-Free Guidance (CFG)
/// Input: Predicted Noise with text (cond), Predicted Noise without text (uncond).
/// Operation: Amplify the signal that aligns with the text.
/// s is the guidance scale.
/// Output: Guided Noise Prediction.
#[verified_engine::verified]
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
