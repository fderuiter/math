//! # Adversarially Trained Neural Operator (adv-NO)
//!
//! This module implements the adv-NO model, which consists of a generator
//! (a U-Net based Neural Operator) and a discriminator. It is trained
//! adversarially to mitigate spectral bias.

use tch::nn::VarStore;
use crate::applied::generative_turbulence::networks::unet::UNet;

/// Represents the Adversarially Trained Neural Operator (adv-NO).
pub struct AdvNO {
    pub generator: UNet,
    pub discriminator: UNet,
}

const TIME_EMB_DIM: i64 = 64;

impl AdvNO {
    /// Creates a new adv-NO model.
    ///
    /// # Arguments
    /// * `vs_gen` - The variable store for the generator.
    /// * `vs_disc` - The variable store for the discriminator.
    /// * `c_in` - Number of input channels for the generator.
    /// * `c_out` - Number of output channels for the generator.
    /// * `c_init` - Number of initial channels in the U-Nets.
    pub fn new(vs_gen: &VarStore, vs_disc: &VarStore, c_in: i64, c_out: i64, c_init: i64) -> Self {
        let generator = UNet::new(
            &vs_gen.root(),
            c_in,
            c_out,
            c_init,
            Some(TIME_EMB_DIM),
        );
        // The discriminator takes the generator's output as input.
        let discriminator = UNet::new(
            &vs_disc.root(),
            c_out, // Input channels for discriminator is output from generator
            1,     // Discriminator outputs a single value (real/fake)
            c_init,
            Some(TIME_EMB_DIM),
        );
        AdvNO { generator, discriminator }
    }
}
