//! Radar-based Respiratory Gating.
//!
//! This module implements a pipeline for using mmWave radar (e.g., TI IWR6843) to monitor
//! patient breathing for respiratory gating in Radiation Therapy (LINAC).
//!
//! # Pipeline
//!
//! 1.  **Physics**: Signal processing for FMCW radar (Range/Doppler).
//! 2.  **Geometry**: Coordinate transformation from Sensor Frame to Patient Frame.
//! 3.  **Surface**: Bi-Quadratic polynomial fitting to smooth point cloud noise.
//! 4.  **Tracking**: Kalman Filter for temporal smoothing and velocity estimation.
//! 5.  **Gating**: Schmidt Trigger logic with latency compensation for beam control.

pub mod physics;
pub mod geometry;
pub mod surface;
pub mod tracking;
pub mod gating;

pub use physics::{FmcwConfig, C};
pub use geometry::{SphericalPoint, AngleFftConfig, SensorToPatientTransform};
pub use surface::BiQuadraticSurface;
pub use tracking::TrackingFilter;
pub use gating::GatingLogic;
