//! # Residual-in-Residual Dense Block (RRDB)
//!
//! This module implements the RRDB, a powerful building block for deep
//! super-resolution networks like ESRGAN. It consists of nested residual
//! connections and dense blocks.

use tch::{
    nn::{self, Module, Path},
    Tensor,
};

const BETA: f64 = 0.2; // Scaling factor for residual connections

/// A single Residual Dense Block, the building block for the RRDB.
#[derive(Debug)]
struct ResidualDenseBlock {
    conv1: nn::Conv2D,
    conv2: nn::Conv2D,
    conv3: nn::Conv2D,
    conv4: nn::Conv2D,
    conv5: nn::Conv2D,
}

impl ResidualDenseBlock {
    fn new(p: &Path, c_in: i64, c_growth: i64) -> Self {
        let conv_config = nn::ConvConfig { padding: 1, ..Default::default() };
        let conv1 = nn::conv2d(p / "c1", c_in, c_growth, 3, conv_config);
        let conv2 = nn::conv2d(p / "c2", c_in + c_growth, c_growth, 3, conv_config);
        let conv3 = nn::conv2d(p / "c3", c_in + 2 * c_growth, c_growth, 3, conv_config);
        let conv4 = nn::conv2d(p / "c4", c_in + 3 * c_growth, c_growth, 3, conv_config);
        let conv5 = nn::conv2d(p / "c5", c_in + 4 * c_growth, c_in, 3, conv_config);
        ResidualDenseBlock { conv1, conv2, conv3, conv4, conv5 }
    }
}

impl Module for ResidualDenseBlock {
    fn forward(&self, xs: &Tensor) -> Tensor {
        let x1 = self.conv1.forward(xs).leaky_relu();
        let x2 = self.conv2.forward(&Tensor::cat(&[xs, &x1], 1)).leaky_relu();
        let x3 = self.conv3.forward(&Tensor::cat(&[xs, &x1, &x2], 1)).leaky_relu();
        let x4 = self.conv4.forward(&Tensor::cat(&[xs, &x1, &x2, &x3], 1)).leaky_relu();
        let x5 = self.conv5.forward(&Tensor::cat(&[xs, &x1, &x2, &x3, &x4], 1));
        xs + x5 * BETA
    }
}


/// A Residual-in-Residual Dense Block, composed of multiple ResidualDenseBlocks.
#[derive(Debug)]
pub struct RRDB {
    rdb1: ResidualDenseBlock,
    rdb2: ResidualDenseBlock,
    rdb3: ResidualDenseBlock,
}

impl RRDB {
    pub fn new(p: &Path, c_in: i64, c_growth: i64) -> Self {
        let rdb1 = ResidualDenseBlock::new(&(p / "rdb1"), c_in, c_growth);
        let rdb2 = ResidualDenseBlock::new(&(p / "rdb2"), c_in, c_growth);
        let rdb3 = ResidualDenseBlock::new(&(p / "rdb3"), c_in, c_growth);
        RRDB { rdb1, rdb2, rdb3 }
    }
}

impl Module for RRDB {
    fn forward(&self, xs: &Tensor) -> Tensor {
        let out1 = self.rdb1.forward(xs);
        let out2 = self.rdb2.forward(&out1);
        let out3 = self.rdb3.forward(&out2);
        xs + out3 * BETA
    }
}
