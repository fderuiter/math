//! Coordinate Transformations for Radar Data.
//!
//! Handles the conversion from raw spherical coordinates (Sensor Frame) to Cartesian coordinates (Patient Frame).
//!
//! # TI IWR6843 Coordinate Convention
//! - **r**: Depth (range).
//! - **x**: Horizontal azimuth.
//! - **z**: Vertical elevation.
//!
//! Note: This differs from standard mathematical spherical conventions.

use nalgebra::{Point3, Rotation3, Translation3};

/// A raw detection point in spherical/sensor space.
#[derive(Debug, Clone, Copy)]
pub struct SphericalPoint {
    /// Range (distance) in meters.
    pub range: f64,
    /// Azimuth FFT bin index.
    pub azimuth_index: isize,
    /// Elevation FFT bin index.
    pub elevation_index: isize,
}

/// Configuration for the Angle-FFT processing.
#[derive(Debug, Clone, Copy)]
pub struct AngleFftConfig {
    /// Size of the Azimuth FFT (e.g., 64).
    pub n_fft_azimuth: usize,
    /// Size of the Elevation FFT (e.g., 32).
    pub n_fft_elevation: usize,
}

impl AngleFftConfig {
    /// Converts a raw spherical point to Cartesian coordinates in the Sensor Frame.
    ///
    /// Let $w_x$ and $w_z$ be the normalized sine values derived from indices:
    /// $$ w_x = \frac{2 \cdot I_{az}}{N_{az}}, \quad w_z = \frac{2 \cdot I_{el}}{N_{el}} $$
    ///
    /// The Cartesian coordinates $(x, y, z)$ are:
    /// $$ x = r \cdot w_x $$
    /// $$ z = r \cdot w_z $$
    /// $$ y = \sqrt{r^2 - x^2 - z^2} $$
    ///
    /// Note: The TI convention defines $y$ as the depth (range direction) for the *derived* Cartesian frame here,
    /// but the prompt specifies $x, y, z$ as:
    /// "x = r * wx"
    /// "z = r * wz"
    /// "y = sqrt(r^2 - x^2 - z^2)"
    ///
    /// Usually in standard TI radar SDK:
    /// - x is horizontal
    /// - y is depth (range)
    /// - z is vertical
    pub fn spherical_to_cartesian(&self, point: &SphericalPoint) -> Point3<f64> {
        let w_x = (2.0 * point.azimuth_index as f64) / self.n_fft_azimuth as f64;
        let w_z = (2.0 * point.elevation_index as f64) / self.n_fft_elevation as f64;

        let x = point.range * w_x;
        let z = point.range * w_z;

        // Ensure the term under sqrt is non-negative (handle numerical noise or invalid points)
        let y_sq = point.range.powi(2) - x.powi(2) - z.powi(2);
        let y = if y_sq > 0.0 { y_sq.sqrt() } else { 0.0 };

        Point3::new(x, y, z)
    }
}

/// Represents the rigid body transformation from Sensor Frame to Patient Frame.
#[derive(Debug, Clone)]
pub struct SensorToPatientTransform {
    /// Rotation matrix ($R_{3\times3}$).
    pub rotation: Rotation3<f64>,
    /// Translation vector ($T_{3\times1}$).
    pub translation: Translation3<f64>,
}

impl SensorToPatientTransform {
    /// Creates a new transformation from a rotation and translation.
    pub fn new(rotation: Rotation3<f64>, translation: Translation3<f64>) -> Self {
        Self {
            rotation,
            translation,
        }
    }

    /// Transforms a point from Sensor Frame ($P_S$) to Patient Frame ($P_P$).
    ///
    /// $$ P_P = R \cdot P_S + T $$
    pub fn transform_point(&self, point_sensor: &Point3<f64>) -> Point3<f64> {
        self.translation * (self.rotation * point_sensor)
    }
}
