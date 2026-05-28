//! # Vector-Quantized Variational Autoencoder (VQ-VAE)
//!
//! This module implements the VQ-VAE used for comparison in the paper.
//! It features a discrete latent space (codebook) and skip connections.

use tch::nn::VarStore;
use crate::networks::unet::{UNet, UNetBuilder};

/// Represents the VQ-VAE model.
pub struct VqVae {
    pub model: UNet, // The UNet architecture serves as the encoder and decoder with skip connections
    // A field for the codebook will be added here.
}

impl VqVae {
    /// Creates a new VQ-VAE model.
    pub fn new(vs: &VarStore, c_in: i64, c_out: i64, c_init: i64) -> Self {
        // The VAE in the paper is not time-conditioned, so we pass None.
        let model = UNetBuilder::new()
            .c_in(c_in)
            .c_out(c_out)
            .c_init(c_init)
            .time_emb_dim(None)
            .build(&vs.root());
        VqVae { model }
    }
}
