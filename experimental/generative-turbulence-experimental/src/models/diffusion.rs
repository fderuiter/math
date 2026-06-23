//! # Conditional Diffusion Model
//!
//! This module implements the conditional score-based diffusion model used for
//! reconstructing turbulent flow fields from sparse observations.

use crate::networks::unet::{UNet, UNetBuilder};
use tch::nn::VarStore;

/// Represents the conditional diffusion model.
pub struct DiffusionModel {
    pub score_network: UNet,
}

const TIME_EMB_DIM: i64 = 64;

impl DiffusionModel {
    /// Creates a new diffusion model.
    ///
    /// # Arguments
    /// * `vs` - The variable store for the score network.
    /// * `c_in` - Number of input channels (e.g., 8 for masked flow + mask).
    /// * `c_out` - Number of output channels (e.g., 4 for the flow variables).
    /// * `c_init` - Number of initial channels in the U-Net.
    pub fn new(vs: &VarStore, c_in: i64, c_out: i64, c_init: i64) -> Self {
        // The score network is a U-Net, conditioned on noise level (sigma) and sparse data.
        let score_network = UNetBuilder::new()
            .c_in(c_in)
            .c_out(c_out)
            .c_init(c_init)
            .time_emb_dim(Some(TIME_EMB_DIM))
            .build(&vs.root());

        DiffusionModel { score_network }
    }
}
