//! MRI Physics Simulation Module
//!
//! This module provides a rigorous simulation of Magnetic Resonance Imaging (MRI) physics,
//! covering Quantum Foundations, Classical Dynamics (Bloch Equations), Spatial Encoding,
//! and Image Reconstruction.
//!
//! # Domains
//!
//! 1. **Quantum Foundations**: Proton properties, Larmor frequency, and Boltzmann statistics.
//! 2. **Classical Dynamics**: Bloch equation simulation for magnetization vectors.
//! 3. **Spatial Encoding**: Gradient fields and k-space trajectory calculations.
//! 4. **Image Reconstruction**: Signal generation and Inverse Fourier Transform.

pub mod proton;
pub mod bloch;
pub mod scanner;
pub mod reconstruction;

// Re-export for backward compatibility and flattened API access
pub use bloch::BlochSimulator;
// Re-export MriScanner and Strategy
pub use scanner::MriScanner;
pub use reconstruction::{ReconstructionStrategy, DftReconstructor};

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{Vector3, DMatrix};
    use num_complex::Complex;
    use crate::pure_math::analysis::ode::RungeKutta4;
    use std::f64::consts::PI;
    use approx::assert_relative_eq;

    #[test]
    fn test_larmor_frequency() {
        // Verify Larmor frequency of Hydrogen at 1.5 Tesla
        // Expected: ~ 63.87 MHz -> 63.87e6 * 2pi rad/s
        let b0 = 1.5;
        let omega = proton::larmor_frequency(b0);
        let freq_hz = omega / (2.0 * PI);

        // 63.87 MHz is approx 63,857,000 Hz based on gamma/2pi = 42.57 MHz/T
        // 42.57 * 1.5 = 63.855
        // Using gamma = 2.675e8:
        // f = 2.675e8 * 1.5 / 2pi = 4.0125e8 / 6.283... = 63.856 MHz

        assert_relative_eq!(freq_hz, 63.856e6, epsilon = 1.0e5);
    }

    #[test]
    fn test_bloch_relaxation() {
        // Test T2 relaxation
        // Initialize M = [0, 1, 0], B = 0 (no precession), T1 = inf, T2 = 1.0
        let initial_m = Vector3::new(0.0, 1.0, 0.0);
        let m0 = 1.0;
        let mut bloch = BlochSimulator::new(initial_m, m0);

        let dt = 0.01;
        let t2 = 0.5; // T2 = 0.5s
        let t1 = 1e9; // Long T1
        let b_field = Vector3::zeros(); // No B field to isolate relaxation

        // Step for a total of 0.5 seconds (1 * T2)
        // M_y should decay to 1/e * initial
        let steps = (t2 / dt) as usize;
        for _ in 0..steps {
            bloch.step(dt, b_field, t1, t2);
        }

        let expected_y = (-1.0_f64).exp(); // e^-1 approx 0.3678

        // Euler integration is an approximation, so allow some error
        assert_relative_eq!(bloch.magnetization.y, expected_y, epsilon = 0.02);
        assert_relative_eq!(bloch.magnetization.x, 0.0);
        // z should recover towards m0=1 from 0? No, initial z=0.
        // dMz/dt = (M0 - Mz)/T1 approx 0.
        assert_relative_eq!(bloch.magnetization.z, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_bloch_rk4_accuracy() {
        // T2 relaxation with RK4 should be more accurate than Euler
        let initial_m = Vector3::new(0.0, 1.0, 0.0);
        let m0 = 1.0;
        let mut bloch = BlochSimulator::new(initial_m, m0);

        let dt = 0.1; // Large step size where Euler struggles
        let t2 = 1.0;
        let t1 = 1e9;
        let b_field = Vector3::zeros();

        // 1 second simulation
        let steps = (1.0 / dt) as usize;
        for _ in 0..steps {
            bloch.step_with(dt, b_field, t1, t2, &RungeKutta4);
        }

        let expected_y = (-1.0_f64).exp();

        // With dt=0.1, Euler error is visible. RK4 should be very close.
        assert_relative_eq!(bloch.magnetization.y, expected_y, epsilon = 1e-5);
    }

    #[test]
    fn test_mri_scanner_reconstruction() {
        // Create a synthetic image (10x10)
        let n = 10;
        let mut density = DMatrix::from_element(n, n, Complex::new(0.0, 0.0));
        // Add a "tumor" (high signal) at (2, 2)
        density[(2, 2)] = Complex::new(1.0, 0.0);
        // Add background noise
        density[(5, 5)] = Complex::new(0.1, 0.0);

        // Create Scanner with default DFT strategy
        let scanner = MriScanner::new(Box::new(DftReconstructor));

        // 1. Simulate Signal (Forward)
        let k_space = scanner.scan(&density);

        // 2. Reconstruct (Inverse)
        let reconstructed = scanner.reconstruct(&k_space);

        // Verify reconstruction matches input (ignoring scaling)
        // Note: Our DFT/IDFT might have scaling factors.
        // Usually, DFT * IDFT = N * I or similar.
        // Let's check relative structure.

        let val_2_2 = reconstructed[(2, 2)].norm();
        let val_5_5 = reconstructed[(5, 5)].norm();

        // The signal at (2, 2) should be roughly 10x signal at (5, 5)
        assert_relative_eq!(val_2_2 / val_5_5, 10.0, epsilon = 1.0);
    }
}
