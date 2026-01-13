//! Chirp Z-Transform (CZT) Implementation.
//!
//! This module implements the Chirp Z-Transform algorithm as verified in the Bressler et al. (2024) analysis.
//! It allows for high-resolution spectral analysis within a specific frequency band (zoom FFT),
//! overcoming the resolution limits of the standard FFT.

use num_complex::Complex;
use std::f64::consts::PI;

/// Calculates the Chirp Z-Transform (CZT) for a specific frequency band.
///
/// This implementation corresponds to the corrected Equation (2) from the Bressler verification:
/// $$ X_{k, CZT} = \sum_{n=0}^{N-1} x_n e^{-i 2\pi n \left( \frac{f_0 + B \frac{k}{K}}{f_s} \right)} $$
///
/// Alternate notation (from prompt):
/// $$ X_{k,CZT} = \sum_{n=0}^{N-1} x_n e^{-i 2\pi n (f_{0,norm} + \frac{Bk}{K f_s})} $$
/// where $f_{0,norm} = f_0 / f_s$.
///
/// # Arguments
///
/// * `signal` - The time-domain input signal ($x_n$).
/// * `start_freq` - The starting frequency of the zoom window ($f_0$) in Hz.
/// * `bandwidth` - The bandwidth of the zoom window ($B$) in Hz.
/// * `sample_rate` - The sampling rate of the input signal ($f_s$) in Hz.
/// * `output_bins` - The number of frequency bins in the output ($K$).
///
/// # Returns
///
/// A vector of complex numbers representing the frequency spectrum in the specified band.
pub fn chirp_z_transform(
    signal: &[Complex<f64>],
    start_freq: f64,
    bandwidth: f64,
    sample_rate: f64,
    output_bins: usize,
) -> Vec<Complex<f64>> {
    let mut output = Vec::with_capacity(output_bins);

    // Iterate over each output frequency bin 'k'
    for k in 0..output_bins {
        let mut sum = Complex::new(0.0, 0.0);

        // Calculate the specific frequency for this bin: f_k = f_0 + B * (k / K)
        // Note: The formula in the paper uses k/K for the step fraction.
        // Usually, the bandwidth B covers the range from start_freq to end_freq.
        // If B is the total width, the step size is B / output_bins.
        let freq_step_fraction = k as f64 / output_bins as f64;
        let target_freq = start_freq + bandwidth * freq_step_fraction;

        // Term inside the exponent: (f_k / f_s)
        let normalized_freq = target_freq / sample_rate;

        // Summation over time samples 'n'
        for (n, &x_n) in signal.iter().enumerate() {
            // exponent = -i * 2 * pi * n * normalized_freq
            let theta = -2.0 * PI * (n as f64) * normalized_freq;
            let basis = Complex::new(0.0, theta).exp();

            sum += x_n * basis;
        }

        output.push(sum);
    }

    output
}

/// Helper struct to configure CZT using Spatial Parameters ("Fonzi" Step 1).
pub struct SpatialCztConfig {
    /// Start distance in meters ($A$ parameter equivalent).
    pub start_distance: f64,
    /// Resolution step size in meters ($W$ parameter equivalent).
    pub step_distance: f64,
    /// Number of output bins ($K$).
    pub output_bins: usize,
    /// Radar Bandwidth in Hz ($B$).
    pub bandwidth: f64,
    /// Chirp Time in seconds ($T_c$).
    pub chirp_time: f64,
    /// Speed of light ($c$).
    pub c: f64,
}

impl SpatialCztConfig {
    /// Converts spatial parameters to frequency parameters and runs the CZT.
    ///
    /// # Arguments
    ///
    /// * `signal` - The time-domain input signal.
    /// * `sample_rate` - ADC sample rate ($f_s$).
    pub fn process(&self, signal: &[Complex<f64>], sample_rate: f64) -> Vec<Complex<f64>> {
        // Frequency per meter slope: S_f = (2 * B) / (c * T_c)
        let slope = (2.0 * self.bandwidth) / (self.c * self.chirp_time);

        // f_start = slope * start_distance
        let start_freq = slope * self.start_distance;

        // Total bandwidth of the window = slope * (step_size * bins)
        let zoom_bandwidth = slope * (self.step_distance * self.output_bins as f64);

        chirp_z_transform(signal, start_freq, zoom_bandwidth, sample_rate, self.output_bins)
    }
}
