//! Clutter Removal using Elliptical Filtering / Signal Superposition.
//!
//! This module implements the "Noise" Solution: Elliptical Filtering.
//! It removes static clutter (reflections from tables, walls) by estimating the center of the
//! signal locus in the IQ plane. Since breathing motion creates an arc (part of a circle/ellipse)
//! in the IQ plane, and static clutter adds a constant offset, finding and subtracting this
//! offset (the "center") reveals the true motion.

use num_complex::Complex;
use std::collections::VecDeque;

/// Filters static clutter by dynamically estimating the signal center.
pub struct EllipticalFilter {
    /// Buffer of recent samples for estimation.
    buffer: VecDeque<Complex<f64>>,
    /// Window size for estimation (e.g., covering 1-2 full breaths).
    window_size: usize,
    /// Estimated center (Static Clutter component).
    center: Complex<f64>,
}

impl EllipticalFilter {
    /// Creates a new Elliptical Filter.
    ///
    /// # Arguments
    ///
    /// * `window_size` - Number of samples to keep for statistics (e.g., 200 samples @ 20Hz = 10s).
    pub fn new(window_size: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(window_size),
            window_size,
            center: Complex::new(0.0, 0.0),
        }
    }

    /// Processes a sample, updates the clutter estimate, and returns the cleaned sample.
    ///
    /// The filter uses a "Signal Superposition" approach (averaging / bounding box)
    /// to estimate the center of the elliptical trace formed by the breathing motion.
    ///
    /// # Arguments
    ///
    /// * `sample` - Raw complex sample ($I + jQ$).
    ///
    /// # Returns
    ///
    /// The clutter-removed sample ($sample - center$).
    pub fn filter(&mut self, sample: Complex<f64>) -> Complex<f64> {
        if self.buffer.len() >= self.window_size {
            self.buffer.pop_front();
        }
        self.buffer.push_back(sample);

        // Update center estimate
        self.update_center();

        // Return cleaned sample
        sample - self.center
    }

    /// Updates the center estimate based on the current buffer.
    ///
    /// This implementation uses a robust bounding-box center method, which approximates
    /// the center of the ellipse defined by the signal extrema.
    /// $$ C_I = \frac{\max(I) + \min(I)}{2}, \quad C_Q = \frac{\max(Q) + \min(Q)}{2} $$
    fn update_center(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let mut min_i = f64::INFINITY;
        let mut max_i = f64::NEG_INFINITY;
        let mut min_q = f64::INFINITY;
        let mut max_q = f64::NEG_INFINITY;

        for s in &self.buffer {
            if s.re < min_i { min_i = s.re; }
            if s.re > max_i { max_i = s.re; }
            if s.im < min_q { min_q = s.im; }
            if s.im > max_q { max_q = s.im; }
        }

        let center_i = (min_i + max_i) / 2.0;
        let center_q = (min_q + max_q) / 2.0;

        self.center = Complex::new(center_i, center_q);
    }

    /// Returns the current estimated clutter vector.
    pub fn get_clutter_estimate(&self) -> Complex<f64> {
        self.center
    }
}

/// CA-CFAR (Cell Averaging Constant False Alarm Rate) Noise Estimation.
///
/// Estimates the noise level ($E$) around a cell by averaging the values of reference units.
///
/// $$ E = \frac{1}{N} \sum_{i=1}^{N} x_i $$
///
/// # Arguments
/// * `reference_cells` - A slice containing the values of the reference cells ($x_i$).
pub fn ca_cfar_noise_estimation(reference_cells: &[f64]) -> f64 {
    let n = reference_cells.len();
    if n == 0 {
        return 0.0;
    }
    let sum: f64 = reference_cells.iter().sum();
    sum / n as f64
}
