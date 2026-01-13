//! Radar-based Respiratory Gating.
//!
//! This module implements a pipeline for using mmWave radar (e.g., TI IWR6843) to monitor
//! patient breathing for respiratory gating in Radiation Therapy (LINAC).
//!
//! # Pipeline
//!
//! 1.  **Physics**: Signal processing for FMCW radar (Range/Doppler).
//! 2.  **CZT**: Chirp Z-Transform for high-resolution range/doppler zooming.
//! 3.  **Geometry**: Coordinate transformation from Sensor Frame to Patient Frame.
//! 4.  **Surface**: Bi-Quadratic polynomial fitting to smooth point cloud noise.
//! 5.  **Tracking**: Kalman Filter for temporal smoothing and velocity estimation.
//! 6.  **Gating**: Schmidt Trigger logic with latency compensation for beam control.

pub mod czt;
pub mod gating;
pub mod geometry;
pub mod physics;
pub mod surface;
pub mod tracking;

pub use czt::chirp_z_transform;
pub use gating::GatingLogic;
pub use geometry::{AngleFftConfig, SensorToPatientTransform, SphericalPoint};
pub use physics::{C, FmcwConfig};
pub use surface::BiQuadraticSurface;
pub use tracking::TrackingFilter;
