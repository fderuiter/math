//! Discrete Fourier Transform (DFT) implementations.
//!
//! Provides fast algorithms (FFT) for discrete signal processing.

use super::traits::{SpectralTransform, TransformError};
use num_complex::Complex64;
use rustfft::FftPlanner;

/// A Fast Fourier Transform (FFT) implementation using the `rustfft` crate.
///
/// This struct maintains a planner to optimize repeated transforms of the same size.
///
/// # complexity
/// * Time: $O(N \log N)$
/// * Space: $O(N)$
pub struct FastFourierTransform {
    planner: FftPlanner<f64>,
}

impl FastFourierTransform {
    /// Creates a new FFT processor.
    pub fn new() -> Self {
        Self {
            planner: FftPlanner::new(),
        }
    }
}

impl Default for FastFourierTransform {
    fn default() -> Self {
        Self::new()
    }
}

impl SpectralTransform for FastFourierTransform {
    fn forward(&mut self, input: &[Complex64]) -> Result<Vec<Complex64>, TransformError> {
        let len = input.len();
        if len == 0 {
            return Err(TransformError::EmptyInput);
        }

        let fft = self.planner.plan_fft_forward(len);
        let mut buffer = input.to_vec();

        // rustfft processes in-place
        fft.process(&mut buffer);

        Ok(buffer)
    }

    fn inverse(&mut self, input: &[Complex64]) -> Result<Vec<Complex64>, TransformError> {
        let len = input.len();
        if len == 0 {
            return Err(TransformError::EmptyInput);
        }

        let fft = self.planner.plan_fft_inverse(len);
        let mut buffer = input.to_vec();

        // rustfft processes in-place
        fft.process(&mut buffer);

        // Normalize by 1/N to match standard definition of IDFT
        let factor = 1.0 / len as f64;
        for x in buffer.iter_mut() {
            *x *= factor;
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_fft_forward_inverse_identity() -> Result<(), TransformError> {
        let mut fft = FastFourierTransform::new();
        // Create a simple signal: sine wave
        let n = 32;
        let signal: Vec<Complex64> = (0..n)
            .map(|i| {
                let theta = 2.0 * PI * i as f64 / n as f64;
                Complex64::new(theta.sin(), 0.0)
            })
            .collect();

        // Forward
        let spectrum = fft.forward(&signal)?;

        // Inverse
        let reconstructed = fft.inverse(&spectrum)?;

        // Check closeness
        for (orig, recon) in signal.iter().zip(reconstructed.iter()) {
            assert!((orig.re - recon.re).abs() < 1e-10);
            assert!((orig.im - recon.im).abs() < 1e-10);
        }
        Ok(())
    }

    #[test]
    fn test_fft_impulse() -> Result<(), TransformError> {
        let mut fft = FastFourierTransform::new();
        let n = 8;
        let mut signal = vec![Complex64::new(0.0, 0.0); n];
        signal[0] = Complex64::new(1.0, 0.0); // Impulse at t=0

        // FFT of impulse is constant 1
        let spectrum = fft.forward(&signal)?;
        for val in spectrum {
            assert!((val.re - 1.0).abs() < 1e-10);
            assert!(val.im.abs() < 1e-10);
        }
        Ok(())
    }
}
