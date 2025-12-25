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

use nalgebra::{Vector3, DMatrix};
use num_complex::Complex;
use std::f64::consts::PI;

/// Quantum Foundations of MRI.
pub mod proton {
    /// Gyromagnetic Ratio for Hydrogen in rad/s/T.
    /// $\gamma \approx 2.675 \times 10^8$ rad/s/T.
    pub const GYROMAGNETIC_RATIO: f64 = 2.675e8;

    /// Reduced Planck constant in J·s.
    pub const H_BAR: f64 = 1.0545718e-34;

    /// Boltzmann constant in J/K.
    pub const K_B: f64 = 1.380649e-23;

    /// Calculates the Larmor frequency $\omega_0$ for a given magnetic field $B_0$.
    ///
    /// # Arguments
    /// * `b0` - Magnetic field strength in Tesla.
    ///
    /// # Returns
    /// * Larmor frequency in rad/s.
    pub fn larmor_frequency(b0: f64) -> f64 {
        GYROMAGNETIC_RATIO * b0
    }

    /// Calculates the Boltzmann magnetization population ratio $N_-/N_+$.
    ///
    /// The ratio is given by $e^{-\frac{\hbar \gamma B_0}{k_B T}}$.
    ///
    /// # Arguments
    /// * `temperature` - Temperature in Kelvin.
    /// * `b0` - Magnetic field strength in Tesla.
    ///
    /// # Returns
    /// * Population ratio or an error if temperature is invalid (<= 0).
    pub fn boltzmann_ratio(temperature: f64, b0: f64) -> Result<f64, String> {
        if temperature <= 0.0 {
            return Err("Temperature must be positive".to_string());
        }
        let exponent = -(H_BAR * GYROMAGNETIC_RATIO * b0) / (K_B * temperature);
        Ok(exponent.exp())
    }
}

use crate::pure_math::analysis::ode::{OdeSystem, Solver, Euler};

/// Internal helper struct to define the Bloch equations as an OdeSystem.
struct BlochSystem {
    m0: f64,
    t1: f64,
    t2: f64,
    b_field: Vector3<f64>,
}

impl OdeSystem<Vector3<f64>> for BlochSystem {
    fn derivative(&self, _t: f64, state: &Vector3<f64>) -> Vector3<f64> {
        let gamma = proton::GYROMAGNETIC_RATIO;

        // Precession term: M x (gamma B)
        let precession = state.cross(&(self.b_field * gamma));

        // Relaxation terms
        // Transverse relaxation (x and y components decay with T2)
        let transverse_decay = Vector3::new(
            state.x / self.t2,
            state.y / self.t2,
            0.0
        );

        // Longitudinal relaxation (z component recovers to M0 with T1)
        let longitudinal_recovery = Vector3::new(
            0.0,
            0.0,
            (state.z - self.m0) / self.t1
        );

        // Total derivative dM/dt
        precession - transverse_decay - longitudinal_recovery
    }
}

/// Classical Dynamics Simulator using Bloch Equations.
pub struct BlochSimulator {
    /// Current magnetization vector $\vec{M} = (M_x, M_y, M_z)$.
    pub magnetization: Vector3<f64>,
    /// Equilibrium magnetization $M_0$ (aligned with z-axis).
    pub m0: f64,
}

impl BlochSimulator {
    /// Creates a new BlochSimulator.
    ///
    /// # Arguments
    /// * `initial_magnetization` - Initial state of $\vec{M}$.
    /// * `m0` - Equilibrium magnetization.
    pub fn new(initial_magnetization: Vector3<f64>, m0: f64) -> Self {
        Self {
            magnetization: initial_magnetization,
            m0,
        }
    }

    /// Performs a time-step update of the magnetization vector using a provided solver.
    ///
    /// This method allows dependency injection of the numerical integration strategy (e.g., Euler, RK4).
    ///
    /// # Arguments
    /// * `dt` - Time step in seconds.
    /// * `b_field` - Magnetic field vector $\vec{B}$ in Tesla.
    /// * `t1` - Longitudinal relaxation time in seconds.
    /// * `t2` - Transverse relaxation time in seconds.
    /// * `solver` - The numerical solver strategy to use.
    pub fn step_with<S>(&mut self, dt: f64, b_field: Vector3<f64>, t1: f64, t2: f64, solver: &S)
    where
        S: Solver<Vector3<f64>>,
    {
        let system = BlochSystem {
            m0: self.m0,
            t1,
            t2,
            b_field,
        };
        // Time is treated as 0.0 for the step since B is constant over the interval
        self.magnetization = solver.solve(&system, 0.0, &self.magnetization, dt);
    }

    /// Performs a time-step update of the magnetization vector using the Bloch equations.
    ///
    /// The coupled differential equations are:
    /// $\frac{d\vec{M}}{dt} = \vec{M} \times (\gamma \vec{B}) - \frac{M_x \hat{i} + M_y \hat{j}}{T_2} - \frac{(M_z - M_0)\hat{k}}{T_1}$
    ///
    /// Uses Euler integration for backward compatibility.
    ///
    /// # Arguments
    /// * `dt` - Time step in seconds.
    /// * `b_field` - Magnetic field vector $\vec{B}$ in Tesla.
    /// * `t1` - Longitudinal relaxation time in seconds.
    /// * `t2` - Transverse relaxation time in seconds.
    pub fn step(&mut self, dt: f64, b_field: Vector3<f64>, t1: f64, t2: f64) {
        self.step_with(dt, b_field, t1, t2, &Euler)
    }
}

/// Spatial Encoding and k-Space Trajectories.
pub mod scanner {
    use super::*;

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
}

/// Image Reconstruction algorithms.
pub mod reconstruction {
    use super::*;

    /// Helper to perform a separable 2D DFT/IDFT using Matrix Multiplication.
    ///
    /// Applies 1D transform to rows, then to columns.
    ///
    /// # Arguments
    /// * `matrix` - Input matrix.
    /// * `inverse` - If true, applies IDFT (positive phase). Else DFT (negative phase).
    fn separable_transform(matrix: &DMatrix<Complex<f64>>, inverse: bool) -> DMatrix<Complex<f64>> {
        let rows = matrix.nrows();
        let cols = matrix.ncols();
        let pi_factor = if inverse { 2.0 * PI } else { -2.0 * PI };

        // Optimization: Use Matrix Multiplication instead of nested loops.
        // The DFT is a linear transformation.
        // Row transform: T = M * W_cols
        // Col transform: R = W_rows * T
        // Total: R = W_rows * M * W_cols

        // 1. Construct Column DFT Matrix W_cols (CxC)
        // W[n, k] = exp(i * phase * n * k / N)
        // This handles the row-wise transform (matrix * W)
        let w_cols = DMatrix::from_fn(cols, cols, |n, k| {
            let phase = pi_factor * (n as f64) * (k as f64) / (cols as f64);
            Complex::from_polar(1.0, phase)
        });

        // 2. Intermediate step: Transform rows
        let temp = matrix * w_cols;

        // 3. Construct Row DFT Matrix W_rows (RxR)
        // W[k, n] = exp(i * phase * k * n / N)
        // This handles the column-wise transform (W * temp)
        let w_rows = DMatrix::from_fn(rows, rows, |k, n| {
            let phase = pi_factor * (k as f64) * (n as f64) / (rows as f64);
            Complex::from_polar(1.0, phase)
        });

        // 4. Final step: Transform columns
        w_rows * temp
    }

    /// Simulates the raw signal $S(k)$ measured from a 2D slice of spin density.
    ///
    /// Computes $S(k_x, k_y) = \sum_{x,y} \rho(x,y) e^{-i 2\pi (k_x x + k_y y)}$
    ///
    /// **Optimization:** Uses Matrix Multiplication ($O(N^3)$) which is heavily optimized by `nalgebra`.
    ///
    /// # Arguments
    /// * `density` - 2D matrix representing the spin density $\rho(x,y)$.
    ///
    /// # Returns
    /// * 2D matrix of k-space samples (raw signal), same dimensions as density.
    pub fn simulate_signal_2d(density: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
        separable_transform(density, false)
    }

    /// Performs a 2D Inverse Discrete Fourier Transform (IDFT) to reconstruct the image.
    ///
    /// Computes $\rho(x,y) = \sum_{k_x, k_y} S(k_x, k_y) e^{+i 2\pi (k_x x + k_y y)}$
    ///
    /// **Optimization:** Uses Matrix Multiplication ($O(N^3)$) which is heavily optimized by `nalgebra`.
    ///
    /// # Arguments
    /// * `k_space` - 2D matrix of k-space samples $S(k_x, k_y)$.
    ///
    /// # Returns
    /// * Reconstructed image density matrix $\rho(x,y)$.
    /// * Note: This implementation does not normalize by 1/N. Scale depends on definition.
    pub fn inverse_dft_2d(k_space: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
        separable_transform(k_space, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::RungeKutta4;
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
}
