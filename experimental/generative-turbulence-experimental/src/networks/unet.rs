//! # U-Net Architecture
//!
//! This module provides a time-conditioned U-Net implementation. It is used
//! as the backbone for the neural operator, the discriminator, and the
//! diffusion model's score network. For super-resolution tasks, it includes
//! a final upsampling path using PixelShuffle.

use tch::{
    nn::{self, Module, Path, Sequential},
    Tensor,
};
use super::time_embedding::TimeEmbedding;

// Helper function for a standard 2D convolutional block.
fn conv_block(p: &Path, c_in: i64, c_out: i64) -> Sequential {
    let conv_config = nn::ConvConfig {
        padding: 1,
        ..Default::default()
    };
    nn::seq()
        .add(nn::conv2d(p / "c1", c_in, c_out, 3, conv_config))
        .add_fn(|xs| xs.relu())
        .add(nn::conv2d(p / "c2", c_out, c_out, 3, conv_config))
        .add_fn(|xs| xs.relu())
}

// The down-sampling path of the U-Net.
#[derive(Debug)]
struct DownBlock {
    conv: Sequential,
    time_mlp: Option<Sequential>,
}

impl DownBlock {
    fn new(p: &Path, c_in: i64, c_out: i64, time_emb_dim: Option<i64>) -> Self {
        let conv = conv_block(p, c_in, c_out);
        let time_mlp = time_emb_dim.map(|emb_dim| nn::seq()
                .add_fn(|xs| xs.silu())
                .add(nn::linear(&(p / "t_mlp"), emb_dim, c_out, Default::default())));
        DownBlock { conv, time_mlp }
    }

    fn forward(&self, xs: &Tensor, t: Option<&Tensor>) -> (Tensor, Tensor) {
        let mut features = self.conv.forward(xs);
        if let (Some(t_emb), Some(mlp)) = (t, &self.time_mlp) {
            let t_emb = mlp.forward(t_emb);
            features += t_emb.unsqueeze(-1).unsqueeze(-1);
        }
        let pooled = features.max_pool2d([2, 2], [2, 2], [0, 0], [1, 1], false);
        (features, pooled)
    }
}

// The up-sampling path of the U-Net.
#[derive(Debug)]
struct UpBlock {
    conv: Sequential,
    time_mlp: Option<Sequential>,
}

impl UpBlock {
    fn new(p: &Path, c_in: i64, c_out: i64, time_emb_dim: Option<i64>) -> Self {
        let conv = conv_block(&(p / "conv"), c_in, c_out);
        let time_mlp = time_emb_dim.map(|emb_dim| nn::seq()
                .add_fn(|xs| xs.silu())
                .add(nn::linear(&(p / "t_mlp"), emb_dim, c_out, Default::default())));
        UpBlock { conv, time_mlp }
    }

    fn forward(&self, xs: &Tensor, skip: &Tensor, t: Option<&Tensor>) -> Tensor {
        let size = skip.size();
        if size.len() != 4 {
            panic!("UpBlock expects 4D skip connection (N, C, H, W), got {:?}", size);
        }
        let h = size[2];
        let w = size[3];
        let upsampled = xs.upsample_nearest2d([h, w], None, None);
        let combined = Tensor::cat(&[skip, &upsampled], 1);
        let mut features = self.conv.forward(&combined);
        if let (Some(t_emb), Some(mlp)) = (t, &self.time_mlp) {
            let t_emb = mlp.forward(t_emb);
            features += t_emb.unsqueeze(-1).unsqueeze(-1);
        }
        features
    }
}

// An upsampling block using PixelShuffle
#[derive(Debug)]
struct UpsampleBlock {
    conv: nn::Conv2D,
}

impl UpsampleBlock {
    fn new(p: &Path, c_in: i64) -> Self {
        let conv_config = nn::ConvConfig { padding: 1, ..Default::default() };
        // We need 4x channels for a 2x shuffle
        let conv = nn::conv2d(p, c_in, c_in * 4, 3, conv_config);
        UpsampleBlock { conv }
    }
}

impl Module for UpsampleBlock {
    fn forward(&self, xs: &Tensor) -> Tensor {
        self.conv.forward(xs).pixel_shuffle(2)
    }
}


/// A builder for constructing a `UNet`.
///
/// This implements the **Builder Pattern** to mitigate construction risks
/// caused by having multiple `i64` parameters with the same type.
#[derive(Debug, Clone, Copy)]
pub struct UNetBuilder {
    c_in: i64,
    c_out: i64,
    c_init: i64,
    time_emb_dim: Option<i64>,
}

impl Default for UNetBuilder {
    fn default() -> Self {
        Self {
            c_in: 3,
            c_out: 3,
            c_init: 64,
            time_emb_dim: None,
        }
    }
}

impl UNetBuilder {
    /// Creates a new builder with default parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of input channels.
    pub fn c_in(mut self, c_in: i64) -> Self {
        self.c_in = c_in;
        self
    }

    /// Sets the number of output channels.
    pub fn c_out(mut self, c_out: i64) -> Self {
        self.c_out = c_out;
        self
    }

    /// Sets the initial number of channels for the internal convolutional blocks.
    pub fn c_init(mut self, c_init: i64) -> Self {
        self.c_init = c_init;
        self
    }

    /// Sets the dimension of the time embedding. If `None`, time conditioning is disabled.
    pub fn time_emb_dim(mut self, time_emb_dim: Option<i64>) -> Self {
        self.time_emb_dim = time_emb_dim;
        self
    }

    /// Builds the `UNet` module.
    ///
    /// # Arguments
    /// * `p` - The path to the variable store.
    pub fn build(self, p: &Path) -> UNet {
        let time_embedding = self.time_emb_dim.map(|dim| TimeEmbedding::new(&(p / "time_emb"), dim));
        let time_dim_for_blocks = self.time_emb_dim;

        let down1 = DownBlock::new(&(p / "d1"), self.c_in, self.c_init, time_dim_for_blocks);
        let down2 = DownBlock::new(&(p / "d2"), self.c_init, self.c_init * 2, time_dim_for_blocks);
        let down3 = DownBlock::new(&(p / "d3"), self.c_init * 2, self.c_init * 4, time_dim_for_blocks);
        let down4 = DownBlock::new(&(p / "d4"), self.c_init * 4, self.c_init * 8, time_dim_for_blocks);

        let bottleneck = conv_block(&(p / "bn"), self.c_init * 8, self.c_init * 16);

        let up1 = UpBlock::new(&(p / "u1"), self.c_init * 16 + self.c_init * 8, self.c_init * 8, time_dim_for_blocks);
        let up2 = UpBlock::new(&(p / "u2"), self.c_init * 8 + self.c_init * 4, self.c_init * 4, time_dim_for_blocks);
        let up3 = UpBlock::new(&(p / "u3"), self.c_init * 4 + self.c_init * 2, self.c_init * 2, time_dim_for_blocks);
        let up4 = UpBlock::new(&(p / "u4"), self.c_init * 2 + self.c_init, self.c_init, time_dim_for_blocks);

        // Upsampling path for super-resolution (3 blocks for 8x)
        let upsample1 = UpsampleBlock::new(&(p / "up1"), self.c_init);
        let upsample2 = UpsampleBlock::new(&(p / "up2"), self.c_init);
        let upsample3 = UpsampleBlock::new(&(p / "up3"), self.c_init);

        let final_conv = nn::conv2d(p / "final", self.c_init, self.c_out, 3, nn::ConvConfig{ padding: 1, ..Default::default() });

        UNet {
            time_embedding,
            down1,
            down2,
            down3,
            down4,
            bottleneck,
            up1,
            up2,
            up3,
            up4,
            upsample1,
            upsample2,
            upsample3,
            final_conv,
        }
    }
}

/// A U-Net model, configurable for 2D inputs and optional time conditioning.
#[derive(Debug)]
pub struct UNet {
    time_embedding: Option<TimeEmbedding>,
    down1: DownBlock,
    down2: DownBlock,
    down3: DownBlock,
    down4: DownBlock,
    bottleneck: Sequential,
    up1: UpBlock,
    up2: UpBlock,
    up3: UpBlock,
    up4: UpBlock,
    upsample1: UpsampleBlock,
    upsample2: UpsampleBlock,
    upsample3: UpsampleBlock,
    final_conv: nn::Conv2D,
}

impl UNet {
    /// The forward pass for the U-Net.
    pub fn forward_with_time(&self, xs: &Tensor, time: Option<&Tensor>) -> Tensor {
        let t = self.time_embedding.as_ref().zip(time).map(|(te, t)| te.forward(t));

        let (s1, p1) = self.down1.forward(xs, t.as_ref());
        let (s2, p2) = self.down2.forward(&p1, t.as_ref());
        let (s3, p3) = self.down3.forward(&p2, t.as_ref());
        let (s4, p4) = self.down4.forward(&p3, t.as_ref());

        let bn = self.bottleneck.forward(&p4);

        let u1 = self.up1.forward(&bn, &s4, t.as_ref());
        let u2 = self.up2.forward(&u1, &s3, t.as_ref());
        let u3 = self.up3.forward(&u2, &s2, t.as_ref());
        let u4 = self.up4.forward(&u3, &s1, t.as_ref());

        let up1 = self.upsample1.forward(&u4);
        let up2 = self.upsample2.forward(&up1);
        let up3 = self.upsample3.forward(&up2);

        self.final_conv.forward(&up3)
    }
}

impl Module for UNet {
    /// Default forward pass, assumes no time conditioning.
    fn forward(&self, xs: &Tensor) -> Tensor {
        self.forward_with_time(xs, None)
    }
}
