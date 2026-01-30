//! MUSIC (Multiple Signal Classification) Algorithm for FMCW Radar.
//!
//! This module implements the "Math" Solution: Super-Resolution processing.
//! It allows distinguishing between reflection points (e.g., skin vs. clothes) closer than the
//! physical range resolution limit ($\Delta R = c/2B$) by exploiting the eigen-structure of the
//! signal covariance matrix.

use super::error::RadarError;
use nalgebra::{DMatrix, DVector};
use num_complex::Complex;
use std::f64::consts::PI;

/// Implements the MUSIC algorithm for high-resolution range estimation.
pub struct MusicEstimator {
    /// Number of signal snapshots (chirps) to estimate covariance.
    snapshot_count: usize,
    /// Number of samples per chirp ($N$).
    samples_per_chirp: usize,
    /// Estimated number of targets/signals ($P$).
    signal_subspace_dim: usize,
    /// Signal covariance matrix buffer.
    snapshots: Vec<DVector<Complex<f64>>>,
}

impl MusicEstimator {
    /// Creates a new MUSIC Estimator.
    ///
    /// # Arguments
    ///
    /// * `samples_per_chirp` - Length of the FMCW chirp ($N$).
    /// * `smoothing_factor` - Number of snapshots to average for covariance matrix stability (e.g., 10).
    /// * `num_targets` - Expected number of reflecting surfaces (e.g., 1 for just skin, 2 for skin+clothes).
    pub fn new(samples_per_chirp: usize, smoothing_factor: usize, num_targets: usize) -> Self {
        Self {
            snapshot_count: smoothing_factor,
            samples_per_chirp,
            signal_subspace_dim: num_targets,
            snapshots: Vec::with_capacity(smoothing_factor),
        }
    }

    /// Adds a chirp snapshot to the estimator.
    ///
    /// Once enough snapshots are collected, the covariance matrix is robust.
    /// Ideally, call `compute_spectrum` after filling the buffer.
    pub fn add_snapshot(&mut self, chirp: &[Complex<f64>]) -> Result<(), RadarError> {
        if chirp.len() != self.samples_per_chirp {
            return Err(RadarError::ChirpLengthMismatch {
                expected: self.samples_per_chirp,
                actual: chirp.len(),
            });
        }
        let vec = DVector::from_iterator(chirp.len(), chirp.iter().cloned());

        if self.snapshots.len() >= self.snapshot_count {
            self.snapshots.remove(0);
        }
        self.snapshots.push(vec);
        Ok(())
    }

    /// Computes the MUSIC Pseudospectrum over a range of distances.
    ///
    /// $$ P(R) = \frac{1}{a(R)^H E_n E_n^H a(R)} $$
    ///
    /// # Arguments
    ///
    /// * `start_range` - Start distance in meters.
    /// * `end_range` - End distance in meters.
    /// * `step_range` - Resolution step in meters (e.g., 0.001 for 1mm).
    /// * `bandwidth` - Radar bandwidth ($B$) in Hz.
    /// * `c` - Speed of light (default ~3e8).
    ///
    /// # Returns
    ///
    /// A vector of `(Range, Power)` tuples.
    pub fn compute_spectrum(
        &self,
        start_range: f64,
        end_range: f64,
        step_range: f64,
        bandwidth: f64,
        c: f64,
    ) -> Result<Vec<(f64, f64)>, RadarError> {
        if self.snapshots.len() < self.snapshot_count {
            return Err(RadarError::InsufficientSnapshots {
                required: self.snapshot_count,
                actual: self.snapshots.len(),
            });
        }

        // 1. Estimate Covariance Matrix Rxx
        // Rxx = (1/K) * sum(x_k * x_k^H)
        let n = self.samples_per_chirp;
        let mut r_xx = DMatrix::<Complex<f64>>::zeros(n, n);

        for snap in &self.snapshots {
            // snap is column vector (Nx1)
            // snap * snap^H -> NxN matrix
            let outer = snap * snap.adjoint();
            r_xx += outer;
        }
        r_xx /= Complex::new(self.snapshots.len() as f64, 0.0); // normalize

        // 2. Eigen Decomposition
        // We use SVD or Eigendecomposition. Rxx is Hermitian.
        let eigen = r_xx.symmetric_eigen();

        // Eigenvalues are sorted in ascending order by default in nalgebra::symmetric_eigen?
        // Actually we need to check documentation or assume.
        // Usually noise eigenvalues are the smallest ones.
        // If we have P targets, the P largest eigenvalues correspond to signal.
        // The remaining N - P are noise.
        // `eigen.eigenvalues` is a vector.
        // `eigen.eigenvectors` is a matrix where columns are eigenvectors.

        // Sort eigenvalues and permute eigenvectors
        let mut pairs: Vec<(f64, usize)> = eigen
            .eigenvalues
            .iter()
            .enumerate()
            .map(|(i, &v)| (v, i))
            .collect();

        if pairs.iter().any(|(v, _)| v.is_nan()) {
            return Err(RadarError::NumericalInstability);
        }

        // Sort descending (largest first)
        pairs.sort_by(|a, b| b.0.total_cmp(&a.0));

        // 3. Extract Noise Subspace En
        // The last (N - P) eigenvectors correspond to noise.
        let num_noise_vectors = n - self.signal_subspace_dim;
        if num_noise_vectors == 0 {
            return Err(RadarError::InvalidSignalSubspace {
                samples: n,
                subspace: self.signal_subspace_dim,
            });
        }

        // Collect indices of noise eigenvectors (the smallest ones)
        let noise_indices: Vec<usize> = pairs
            .iter()
            .skip(self.signal_subspace_dim) // Skip the P largest (Signal)
            .map(|(_, i)| *i)
            .collect();

        // Construct En matrix (N x (N-P))
        // We can pre-compute En * En^H to speed up the loop: P_noise = En * En^H
        // Then denominator is a^H * P_noise * a
        let mut p_noise = DMatrix::<Complex<f64>>::zeros(n, n);

        for idx in noise_indices {
            let col = eigen.eigenvectors.column(idx);
            p_noise += col * col.adjoint();
        }

        // 4. Compute Spectrum P(R)
        // Optimization: Pre-allocate spectrum vector using robust integer steps
        // Round to nearest integer to handle floating point imprecision
        let steps = ((end_range - start_range) / step_range).round() as usize;
        let mut spectrum = Vec::with_capacity(steps + 1);

        // Constants for steering vector
        // a(R)[n] = exp(j * (4pi * B * R / c) * (n / N))
        // Let alpha = (4pi * B * R) / (c * N)
        // phase[n] = alpha * n
        let constant_factor = (4.0 * PI * bandwidth) / (c * n as f64);

        // Profiler Optimization: Reuse buffers to avoid allocation in the hot loop
        let mut a_vec = DVector::from_element(n, Complex::new(0.0, 0.0));
        let mut tmp = DVector::from_element(n, Complex::new(0.0, 0.0));
        let mut r = start_range;

        // Optimization: Use a fixed loop count to guarantee vector length consistency,
        // but use incremental addition for 'r' to avoid int-to-float conversion overhead in loop body.
        for _ in 0..=steps {
            let alpha = constant_factor * r;

            // Construct steering vector a(R) in-place
            for k in 0..n {
                let phase = alpha * (k as f64);
                a_vec[k] = Complex::new(0.0, phase).exp();
            }

            // Denominator D = a^H * P_noise * a

            // Optimization: Use `mul_to` to avoid allocating `tmp` every iteration
            p_noise.mul_to(&a_vec, &mut tmp);

            // Optimization: Use `dotc` (conjugate dot product) to compute a^H * tmp directly as a scalar
            // This avoids allocating a 1x1 DMatrix.
            let den_complex = a_vec.dotc(&tmp);

            let den = den_complex.norm(); // Should be real, take norm to be safe

            // P(R) = 1 / D
            // Add small epsilon to avoid division by zero
            let power = 1.0 / (den + 1e-12);

            spectrum.push((r, power));
            r += step_range;
        }

        Ok(spectrum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Complex;

    #[test]
    fn test_music_estimator_nan_handling() {
        let n = 4;
        let mut estimator = MusicEstimator::new(n, 2, 1);

        // Add valid snapshots
        let snap1 = vec![Complex::new(1.0, 0.0); n];
        let snap2 = vec![Complex::new(0.0, 1.0); n];
        estimator.add_snapshot(&snap1).unwrap();
        estimator.add_snapshot(&snap2).unwrap();

        // Normally we can't easily force eigen decomposition to produce NaNs without
        // invalid input or matrix properties.
        // However, we can assert that with valid input it doesn't return NumericalInstability.
        let result = estimator.compute_spectrum(0.0, 1.0, 0.1, 1e9, 3e8);
        assert!(result.is_ok());
    }
}
