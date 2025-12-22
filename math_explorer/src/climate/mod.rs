//! # Climate Modeling (CERA)
//!
//! The `climate` module implements the **CERA (Climate-invariant Encoding through Representation Alignment)** framework.
//! This architecture is designed to improve the generalization of machine learning models across different climate scenarios
//! by learning a "climate-invariant" latent space.
//!
//! ## Core Concept
//!
//! Traditional climate models often fail to generalize when the underlying statistical distribution of the climate changes
//! (e.g., training on "Cool" climate data and testing on "Warm" climate data). CERA addresses this by:
//!
//! 1.  **Autoencoding**: Compressing input data into a latent representation.
//! 2.  **Representation Alignment**: Forcing the latent representation of different climates (Control vs. Warm) to be statistically similar using Earth Mover's Distance (EMD) loss.
//! 3.  **Prediction**: Using the aligned latent space to predict future states.
//!
//! ## Architecture
//!
//! The framework consists of three main components:
//!
//! - **Autoencoder** (`autoencoder::Autoencoder`): Compresses input fields (e.g., temperature maps) into a lower-dimensional latent vector.
//! - **Predictor** (`predictor::Predictor`): A neural network that takes the latent vector and predicts the target output.
//! - **Cera Model** (`cera::Cera`): The container that orchestrates the data flow between the Autoencoder and Predictor.
//!
//! ## Example: "Hello World"
//!
//! Here is how to initialize and train a simple CERA model on synthetic data.
//!
//! ```rust
//! use math_explorer::climate::config::CeraConfig;
//! use math_explorer::climate::cera::Cera;
//! use math_explorer::climate::training::CeraTrainer;
//! use nalgebra::DMatrix;
//!
//! // 1. Configure the model
//! let config = CeraConfig {
//!     learning_rate: 0.001,
//!     lambda_pred: 0.1,    // Importance of prediction accuracy
//!     lambda_emd: 0.01,    // Importance of climate invariance
//!     epochs: 1,           // Keep it short for the example
//!     batch_size: 4,
//!     in_channels: 2,      // e.g., Temperature, Humidity
//!     latent_channels: 3,  // Compressed representation size
//!     aligned_channels: 2, // Part of latent space to align
//!     num_levels: 10,      // Vertical levels or time steps
//!     output_size: 20,     // Size of prediction target
//! };
//!
//! // 2. Initialize the model
//! let mut cera = Cera::new(config).expect("Failed to create CERA model");
//!
//! // 3. Generate synthetic data (Batch Size * Num Levels, Channels)
//! let n_samples = 8;
//! let num_levels = 10;
//! let input_rows = n_samples * num_levels;
//!
//! // "Control" climate data
//! // Note: We use a simple closure for randomness to avoid external crate dependency in example
//! let mut seed = 123456789u64;
//! let mut rng = || {
//!     seed = (1103515245 * seed + 12345) % 2147483648;
//!     (seed as f32) / 2147483648.0
//! };
//!
//! let control_inputs = DMatrix::from_fn(input_rows, 2, |_, _| rng());
//! let control_targets = DMatrix::from_fn(n_samples, 20, |_, _| rng());
//!
//! // "Warm" climate data (distribution shift)
//! let warm_inputs = DMatrix::from_fn(input_rows, 2, |_, _| rng() + 1.0);
//!
//! // 4. Train the model
//! // Note: This uses a reference trainer implementation.
//! let mut trainer = CeraTrainer::new(&mut cera);
//! trainer.train(&control_inputs, &control_targets, &warm_inputs);
//!
//! // 5. Make a prediction
//! let test_inputs = DMatrix::from_fn(4 * num_levels, 2, |_, _| rng() + 0.5);
//! let prediction = cera.predict(&test_inputs);
//!
//! println!("Prediction shape: ({}, {})", prediction.nrows(), prediction.ncols());
//! assert_eq!(prediction.ncols(), 20);
//! ```

pub mod autoencoder;
pub mod config;
pub mod cera;
pub mod loss;
pub mod predictor;
pub mod training;
pub mod tensor_ops;
