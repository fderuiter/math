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
//!
//! # Advanced Processing (Fonzi Stack)
//!
//! *   **MIMO**: Beamforming ("Digital Lens") for spatial filtering.
//! *   **Super-Resolution**: MUSIC algorithm for sub-resolution targets.
//! *   **Clutter Removal**: Elliptical filtering for static clutter rejection.
//! *   **Phase**: Differential phase unwrapping for sub-mm displacement.

pub mod physics;
pub mod czt;
pub mod geometry;
pub mod surface;
pub mod tracking;
pub mod gating;
pub mod phase;
pub mod mimo;
pub mod super_resolution;
pub mod clutter;

pub use physics::{FmcwConfig, C};
pub use czt::chirp_z_transform;
pub use czt::SpatialCztConfig;
pub use geometry::{SphericalPoint, AngleFftConfig, SensorToPatientTransform};
pub use surface::BiQuadraticSurface;
pub use tracking::{TrackingFilter, KalmanFilter, KalmanModel, ConstantVelocityModel};
pub use gating::GatingLogic;
pub use phase::PhaseUnwrapper;
pub use mimo::Beamformer;
pub use super_resolution::MusicEstimator;
pub use clutter::EllipticalFilter;
