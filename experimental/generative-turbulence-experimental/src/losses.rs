//! # Loss Functions for Generative Turbulence Models
//!
//! This module implements the specialized loss functions required for training
//! the models described in the paper. This includes adversarial losses,
//! perceptual losses, and the score-matching loss for diffusion models.

use tch::{
    Kind, Tensor,
    nn::{self, Path},
};

/// A helper struct to extract features from a VGG19 model.
/// We manually define the layers to easily access intermediate features.
#[derive(Debug)]
pub struct Vgg19Features {
    conv1_1: nn::Conv2D,
    conv1_2: nn::Conv2D,
    conv2_1: nn::Conv2D,
    conv2_2: nn::Conv2D,
    conv3_1: nn::Conv2D,
    mean: Tensor,
    std: Tensor,
}

impl Vgg19Features {
    /// Creates a new Vgg19 feature extractor.
    /// It builds the first few blocks of VGG19. For this to work,
    /// a VarStore with pre-trained weights must be loaded separately.
    pub fn new(p: &Path, device: tch::Device) -> Result<Self, tch::TchError> {
        let p = p / "features";
        let conv_cfg = nn::ConvConfig {
            padding: 1,
            ..Default::default()
        };
        let conv1_1 = nn::conv2d(&p / "0", 3, 64, 3, conv_cfg);
        let conv1_2 = nn::conv2d(&p / "2", 64, 64, 3, conv_cfg);
        let conv2_1 = nn::conv2d(&p / "5", 64, 128, 3, conv_cfg);
        let conv2_2 = nn::conv2d(&p / "7", 128, 128, 3, conv_cfg);
        let conv3_1 = nn::conv2d(&p / "10", 128, 256, 3, conv_cfg);

        // ImageNet normalization stats
        let mean = Tensor::f_from_slice(&[0.485f32, 0.456f32, 0.406f32])?
            .view([1, 3, 1, 1])
            .to(device);
        let std = Tensor::f_from_slice(&[0.229f32, 0.224f32, 0.225f32])?
            .view([1, 3, 1, 1])
            .to(device);

        Ok(Vgg19Features {
            conv1_1,
            conv1_2,
            conv2_1,
            conv2_2,
            conv3_1,
            mean,
            std,
        })
    }

    /// Forward pass to extract feature maps.
    pub fn forward(&self, xs: &Tensor) -> Vec<Tensor> {
        let mut xs = (xs - &self.mean) / &self.std;
        let mut outputs = Vec::new();

        xs = xs.apply(&self.conv1_1).relu();
        outputs.push(xs.copy());

        xs = xs.apply(&self.conv1_2).relu();
        outputs.push(xs.copy());

        xs = xs.max_pool2d_default(2);

        xs = xs.apply(&self.conv2_1).relu();
        outputs.push(xs.copy());

        xs = xs.apply(&self.conv2_2).relu();
        outputs.push(xs.copy());

        xs = xs.max_pool2d_default(2);

        xs = xs.apply(&self.conv3_1).relu();
        outputs.push(xs.copy());

        outputs
    }
}

/// Computes the perceptual loss using a pre-trained VGG19 network.
pub fn perceptual_loss(pred: &Tensor, target: &Tensor, vgg_features: &Vgg19Features) -> Tensor {
    // The input to VGG must have 3 channels.
    // If grayscale, repeat. If more than 3 channels, take the first 3.
    let pred_3ch = match pred.size()[1] {
        1 => pred.repeat([1, 3, 1, 1]),
        3 => pred.copy(),
        _ => pred.slice(1, 0, 3, 1),
    };
    let target_3ch = match target.size()[1] {
        1 => target.repeat([1, 3, 1, 1]),
        3 => target.copy(),
        _ => target.slice(1, 0, 3, 1),
    };

    let pred_features = vgg_features.forward(&pred_3ch);
    let target_features = vgg_features.forward(&target_3ch);

    let weights = [0.1f32, 0.1f32, 1.0f32, 1.0f32, 1.0f32];
    let mut total_loss = Tensor::from(0.0f32).to(pred.device());

    for (i, (pred_f, target_f)) in pred_features.iter().zip(target_features.iter()).enumerate() {
        total_loss += pred_f.l1_loss(target_f, tch::Reduction::Mean) * Tensor::from(weights[i]);
    }
    total_loss
}

/// Computes the relativistic average GAN (RaGAN) loss.
pub fn ragan_loss(real_pred: &Tensor, fake_pred: &Tensor) -> Tensor {
    let real_avg = real_pred.mean(Kind::Float);
    let fake_avg = fake_pred.mean(Kind::Float);
    let real_loss = (real_pred - &fake_avg).binary_cross_entropy_with_logits(
        &Tensor::ones_like(real_pred),
        Option::<Tensor>::None,
        Option::<Tensor>::None,
        tch::Reduction::Mean,
    );
    let fake_loss = (fake_pred - &real_avg).binary_cross_entropy_with_logits(
        &Tensor::zeros_like(fake_pred),
        Option::<Tensor>::None,
        Option::<Tensor>::None,
        tch::Reduction::Mean,
    );
    real_loss + fake_loss
}

/// Computes the adversarial loss for the generator (adv-NO).
/// Implements the relativistic average GAN (RaGAN) loss.
pub fn generator_loss(real_pred: &Tensor, fake_pred: &Tensor) -> Tensor {
    ragan_loss(real_pred, fake_pred)
}

/// Computes the adversarial loss for the discriminator.
/// Implements the relativistic average GAN (RaGAN) loss.
pub fn discriminator_loss(real_pred: &Tensor, fake_pred: &Tensor) -> Tensor {
    ragan_loss(real_pred, fake_pred)
}

/// Computes the score-matching loss for the diffusion model.
pub fn score_matching_loss(score_net_out: &Tensor, noise: &Tensor, sigma: &Tensor) -> Tensor {
    (score_net_out + noise / sigma).square().mean(Kind::Float)
}
