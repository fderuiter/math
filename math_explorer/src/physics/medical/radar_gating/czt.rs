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
/// # Arguments
///
/// * `signal` - The time-domain input signal ($x_n$).
/// * `start_freq` - The starting frequency of the zoom window ($f_0$).
/// * `bandwidth` - The bandwidth of the zoom window ($B$).
/// * `sample_rate` - The sampling rate of the input signal ($f_s$).
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
