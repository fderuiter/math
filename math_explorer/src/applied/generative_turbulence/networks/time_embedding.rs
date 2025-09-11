//! # Time Embedding Module
//!
//! This module provides a time embedding layer that converts a scalar time step
//! into a high-dimensional vector representation using sinusoidal embeddings,
//! as is common in Transformer models and diffusion models.

use tch::{
    nn::{self, Module, Path, Sequential},
    Kind, Tensor, Device,
};

/// A sinusoidal time embedding module.
#[derive(Debug)]
pub struct TimeEmbedding {
    mlp: Sequential,
    dim: i64,
}

impl TimeEmbedding {
    pub fn new(p: &Path, dim: i64) -> Self {
        let mlp = nn::seq()
            .add(nn::linear(p / "l1", dim, dim * 4, Default::default()))
            .add_fn(|xs| xs.silu())
            .add(nn::linear(p / "l2", dim * 4, dim, Default::default()));
        TimeEmbedding { mlp, dim }
    }
}

impl Module for TimeEmbedding {
    /// Takes a batch of time steps `t` (shape `[batch_size]`) and returns
    /// embeddings (shape `[batch_size, dim]`).
    fn forward(&self, t: &Tensor) -> Tensor {
        let half_dim = self.dim / 2;
        let freqs = {
            let arange = Tensor::arange(half_dim, (Kind::Float, t.device()));
            (arange * -(10000.0f64.ln() / half_dim as f64)).exp()
        };
        let args = t.unsqueeze(-1) * freqs.unsqueeze(0);
        let embedding = Tensor::cat(&[args.cos(), args.sin()], -1);
        self.mlp.forward(&embedding)
    }
}
