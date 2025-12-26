//! # Climate Modeling: CERA Framework
//!
//! The **CERA (Climate-invariant Encoding through Representation Alignment)** framework addresses a critical
//! challenge in climate science: **Generalization**.
//!
//! Models trained on one climate scenario (e.g., current conditions) often fail when applied to another
//! (e.g., +2°C warming) due to distribution shifts. CERA solves this by forcing the model to learn
//! a "Climate-Invariant" latent space.
//!
//! ## Architecture
//!
//! CERA consists of three main components:
//! 1.  **Encoder**: Compresses raw climate data into a latent representation.
//! 2.  **Decoder**: Reconstructs the original data from the latent representation (ensuring information retention).
//! 3.  **Predictor**: Predicts a target variable (e.g., precipitation) from the latent representation (ensuring utility).
//!
//! The key innovation is the **Alignment Loss**, which penalizes differences in the latent distributions
//! of different climate scenarios.
//!
//! ```mermaid
//! graph TD
//!     subgraph Inputs
//!     D1[Data: Climate A]
//!     D2[Data: Climate B]
//!     end
//!
//!     subgraph CERA_Model
//!     Enc[Encoder]
//!     Lat[Latent Space Z]
//!     Dec[Decoder]
//!     Pred[Predictor]
//!     end
//!
//!     D1 --> Enc
//!     D2 --> Enc
//!     Enc --> Lat
//!
//!     Lat --> Dec --> Rec1[Reconstruction A]
//!     Lat --> Dec --> Rec2[Reconstruction B]
//!     Lat --> Pred --> P1[Prediction]
//!
//!     style Lat fill:#f9f,stroke:#333,stroke-width:2px,color:black
//! ```
//!
//! ## Example: "Hello World"
//!
//! Initialize a CERA model configuration and perform a dummy training step.
//!
//! ```rust
//! use math_explorer::climate::cera::Cera;
//! use math_explorer::climate::config::CeraConfig;
//! use nalgebra::DMatrix;
//!
//! fn main() {
//!     // 1. Configure the model
//!     let config = CeraConfig {
//!         in_channels: 10,
//!         latent_channels: 4,
//!         aligned_channels: 2,
//!         num_levels: 1, // Treat as 1 level for flat input
//!         output_size: 1, // Regression target
//!         learning_rate: 0.001,
//!         lambda_emd: 0.1,
//!         lambda_pred: 1.0,
//!         epochs: 1,
//!         batch_size: 5,
//!     };
//!
//!     let mut cera = Cera::new(config).expect("Failed to create CERA model");
//!
//!     // 2. Create dummy data (Batch size 5, Input dim 10)
//!     // The encoder outputs a reconstruction of the same shape as input
//!     let data_source = DMatrix::from_element(5, 10, 0.5);
//!
//!     // The predictor outputs data of shape (batch_size, output_size)
//!     let data_target = DMatrix::from_element(5, 1, 0.6);
//!
//!     // Labels are not used in this simplified call but kept for API match
//!     let labels = DMatrix::from_element(5, 1, 1.0);
//!
//!     // 3. Forward pass (returns total loss)
//!     // In a real loop, you would call `train_step`.
//!     let loss = cera.train_step(&data_source, &data_target, &labels);
//!
//!     println!("Training Step Loss: {:.4}", loss);
//! }
//! ```

pub mod autoencoder;
pub mod config;
pub mod cera;
pub mod loss;
pub mod predictor;
pub mod training;
pub mod tensor_ops;
