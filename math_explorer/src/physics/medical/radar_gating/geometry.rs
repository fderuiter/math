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
    /// Number of RX antennas.
    pub n_rx_antennas: usize,
    /// Distance between antennas in meters (usually lambda/2).
    pub antenna_spacing: f64,
    /// Operating wavelength (lambda).
    pub wavelength: f64,
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
    /// Note: The TI convention defines $y$ as the depth (range direction) for the *derived* Cartesian frame here.
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

    /// Estimates the Angle of Arrival (AoA) from phase difference.
    ///
    /// $$ \theta = \sin^{-1} \left( \frac{\lambda \Delta \phi}{2 \pi l} \right) $$
    ///
    /// # Arguments
    /// * `phase_diff` - Phase difference between two antennas in radians.
    pub fn angle_of_arrival(&self, phase_diff: f64) -> f64 {
        // (lambda * delta_phi) / (2 * pi * l)
        let val = (self.wavelength * phase_diff) / (2.0 * std::f64::consts::PI * self.antenna_spacing);
        val.clamp(-1.0, 1.0).asin()
    }

    /// Calculates the angular resolution.
    ///
    /// $$ \theta_{res} = \frac{\lambda}{N_{RX} l \cos(\theta)} $$
    ///
    /// # Arguments
    /// * `theta` - The look angle (0 for boresight).
    pub fn angle_resolution(&self, theta: f64) -> f64 {
        self.wavelength / (self.n_rx_antennas as f64 * self.antenna_spacing * theta.cos())
    }

    /// Calculates the required angular separation to distinguish thoracic vs abdominal motion.
    ///
    /// $$ \Delta\theta_{\text{req}} \approx \arctan\left(\frac{d_{\text{TAA}}}{R}\right) $$
    ///
    /// # Arguments
    /// * `vertical_distance` - Distance between thoracic and abdominal regions ($d_{\text{TAA}}$).
    /// * `range` - Sensor range ($R$).
    pub fn required_angular_separation(vertical_distance: f64, range: f64) -> f64 {
        (vertical_distance / range).atan()
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
