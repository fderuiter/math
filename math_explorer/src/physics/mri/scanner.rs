//! Spatial Encoding and k-Space Trajectories.

use nalgebra::{Vector3, DMatrix};
use num_complex::Complex;
use std::f64::consts::PI;
use super::proton;
use super::reconstruction::{ReconstructionStrategy, simulate_signal_2d};

/// Calculates the accumulated phase $\phi(\vec{r}, t)$ given a spatial position and accumulated gradient.
///
/// Formula: $\phi(\vec{r}, t) = \gamma \vec{r} \cdot \int_0^t \vec{G}(\tau) d\tau$
///
/// # Arguments
/// * `position` - Spatial position $\vec{r}$ in meters.
/// * `gradient_integral` - Time integral of the gradient vector $\int \vec{G} dt$ in T·s/m.
///
/// # Returns
/// * Phase angle in radians.
pub fn accumulated_phase(position: Vector3<f64>, gradient_integral: Vector3<f64>) -> f64 {
    proton::GYROMAGNETIC_RATIO * position.dot(&gradient_integral)
}

/// Calculates the current k-space coordinate.
///
/// Formula: $\vec{k}(t) = \frac{\gamma}{2\pi} \int_0^t \vec{G}(\tau) d\tau$
///
/// # Arguments
/// * `gradient_integral` - Time integral of the gradient vector $\int \vec{G} dt$ in T·s/m.
///
/// # Returns
/// * k-space coordinate vector in cycles/meter ($m^{-1}$).
pub fn k_space_coordinate(gradient_integral: Vector3<f64>) -> Vector3<f64> {
    (proton::GYROMAGNETIC_RATIO / (2.0 * PI)) * gradient_integral
}

/// A high-level abstraction for an MRI Scanner.
///
/// This struct acts as a facade, composing the physics simulation (Forward Problem)
/// and the image reconstruction (Inverse Problem).
///
/// It strictly adheres to **Dependency Inversion**: it depends on the `ReconstructionStrategy`
/// trait, not concrete implementations.
pub struct MriScanner {
    reconstructor: Box<dyn ReconstructionStrategy>,
}

impl MriScanner {
    /// Creates a new MRI Scanner with a specific reconstruction strategy.
    ///
    /// # Arguments
    /// * `reconstructor` - The strategy to use for image reconstruction.
    pub fn new(reconstructor: Box<dyn ReconstructionStrategy>) -> Self {
        Self { reconstructor }
    }

    /// Performs a scan simulation (Forward Problem).
    ///
    /// # Arguments
    /// * `density` - The ground-truth spin density.
    ///
    /// # Returns
    /// * The raw k-space signal.
    pub fn scan(&self, density: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
        // In a real scanner, this would involve Bloch simulation over time.
        // For this abstraction, we use the analytical signal equation.
        simulate_signal_2d(density)
    }

    /// Reconstructs an image from k-space data (Inverse Problem).
    ///
    /// # Arguments
    /// * `k_space` - The raw k-space data.
    ///
    /// # Returns
    /// * The reconstructed image.
    pub fn reconstruct(&self, k_space: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
        self.reconstructor.reconstruct(k_space)
    }
}
