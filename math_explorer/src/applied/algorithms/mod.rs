//! # Algorithms & Data Structures
//!
//! Foundational tools for analysis, estimation, and optimization that support higher-level domains.
//!
//! Unlike the specialized `physics` or `biology` modules, this module provides generic
//! building blocks used across disciplines.
//!
//! ## 🗺️ Taxonomy
//!
//! ```mermaid
//! graph TD
//!     Algo[Algorithms]
//!     Algo --> Est[Estimation]
//!     Algo --> Sort[Sorting]
//!
//!     Est --> Kalman[Kalman Filter]
//!     Sort --> Merge[Merge Sort]
//!     Sort --> Quick[Quick Sort]
//!
//!     subgraph Estimation Loop
//!     Predict((Predict)) -->|Time Step| Prior[Prior Estimate]
//!     Prior --> Update((Update))
//!     Update -->|Measurement| Posterior[Posterior Estimate]
//!     Posterior --> Predict
//!     end
//! ```
//!
//! ## ⚡ Quick Start: 1D Tracking
//!
//! Track a vehicle moving at constant velocity using a Kalman Filter.
//!
//! ```rust
//! use math_explorer::applied::algorithms::kalman::{KalmanFilter, KalmanModel};
//! use nalgebra::{DMatrix, DVector};
//!
//! // Define a 1D Constant Velocity Model (Position, Velocity)
//! struct ConstantVelocity {
//!     process_noise: f64,
//!     measurement_noise: f64,
//! }
//!
//! impl KalmanModel for ConstantVelocity {
//!     fn transition_matrix(&self, dt: f64) -> DMatrix<f64> {
//!         // State: [Position, Velocity]
//!         // [1, dt]
//!         // [0, 1]
//!         DMatrix::from_row_slice(2, 2, &[1.0, dt, 0.0, 1.0])
//!     }
//!     fn measurement_matrix(&self) -> DMatrix<f64> {
//!         // Measure only Position [1, 0]
//!         DMatrix::from_row_slice(1, 2, &[1.0, 0.0])
//!     }
//!     fn process_noise(&self, _dt: f64) -> DMatrix<f64> {
//!         DMatrix::identity(2, 2) * self.process_noise
//!     }
//!     fn measurement_noise(&self) -> DMatrix<f64> {
//!         DMatrix::from_element(1, 1, self.measurement_noise)
//!     }
//! }
//!
//! fn main() {
//!     // 1. Initialize Model
//!     let model = ConstantVelocity { process_noise: 0.1, measurement_noise: 1.0 };
//!     let initial_state = DVector::from_vec(vec![0.0, 5.0]); // Pos=0, Vel=5
//!     let initial_cov = DMatrix::identity(2, 2);
//!
//!     // 2. Create Filter with dt=1.0s
//!     let mut kf = KalmanFilter::new(initial_state, initial_cov, model, 1.0);
//!
//!     // 3. Simulate (Predict -> Update)
//!     // Predict step: State moves from [0, 5] to [5, 5]
//!     kf.predict();
//!
//!     // Update step: Measurement is 4.8 (slightly off due to noise)
//!     let measurement = DVector::from_vec(vec![4.8]);
//!     kf.update(&measurement).unwrap();
//!
//!     println!("Estimated Position: {:.2}", kf.state[0]);
//!     println!("Estimated Velocity: {:.2}", kf.state[1]);
//! }
//! ```

pub mod kalman;
pub mod sorting;
