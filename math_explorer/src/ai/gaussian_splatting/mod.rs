//! # 3D Gaussian Splatting (3DGS)
//!
//! This module implements **3D Gaussian Splatting**, a rasterization technique for real-time view synthesis
//! of 3D scenes.
//!
//! ## 📖 Overview
//!
//! Unlike traditional NeRFs (which use volumetric ray marching), 3DGS represents a scene as a cloud of
//! **3D Gaussians** (ellipsoids). Each Gaussian has:
//! *   Position (Mean)
//! *   Covariance (Shape/Rotation)
//! *   Opacity (Alpha)
//! *   Color (Spherical Harmonics)
//!
//! To render an image, these 3D Gaussians are **splatted** (projected) onto the 2D image plane and
//! alpha-blended from back to front.
//!
//! ## 🧩 Modules
//!
//! *   [`structs`]: Defines `Gaussian3D`, `Scene`, and camera parameters.
//! *   [`projection`]: Handles the math of projecting a 3D ellipsoid into 2D screen space (Jacobians, covariance).
//! *   [`rendering`]: The rasterizer that sorts and blends Gaussians to form an image.
//! *   [`optimization`]: Logic for adaptive density control (cloning/splitting Gaussians).

pub mod structs;
pub mod projection;
pub mod rendering;
pub mod optimization;

pub use structs::{Gaussian3D, Gaussian2D, Scene};
