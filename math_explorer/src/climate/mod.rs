//! The `climate` module provides a Rust implementation of the CERA (Climate-invariant
//! Encoding through Representation Alignment) framework, designed for improving the
//! generalization of machine learning models to different climate scenarios.

pub mod autoencoder;
pub mod cera;
pub mod loss;
pub mod predictor;
pub mod tensor_ops;
