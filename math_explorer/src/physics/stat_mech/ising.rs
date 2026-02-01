//! Ising Model Simulation.
//!
//! This module provides the [`SpinLattice`] struct to simulate a 2D Ising Model
//! using the Metropolis-Hastings algorithm. The Ising model is a mathematical model
//! of ferromagnetism in statistical mechanics. It consists of discrete variables (spins)
//! that can be in one of two states (+1 or -1), arranged in a graph lattice.
//!
//! # Phase Transitions
//!
//! The model exhibits a phase transition at the critical temperature $T_c$.
//! - **$T < T_c$**: Ferromagnetic phase (Spontaneous Magnetization).
//! - **$T > T_c$**: Paramagnetic phase (Disordered spins).

use super::KB;
use rand::Rng;

/// A 2D Spin Lattice for the Ising Model.
///
/// # Examples
///
/// Simulating a ferromagnetic phase transition (Low Temperature):
///
/// ```
/// use math_explorer::physics::stat_mech::ising::SpinLattice;
/// use math_explorer::physics::stat_mech::KB;
///
/// // 1. Setup system parameters
/// let width = 20;
/// let height = 20;
/// let j_coupling = 1.0; // Interaction energy (J > 0 for Ferromagnetism)
/// let h_field = 0.0;    // No external magnetic field
///
/// // 2. Set Temperature below Critical Point (Tc ~ 2.269 * J / KB)
/// // We choose T such that kB * T = 1.5 * J (Well within ordered phase)
/// let temp = 1.5 * j_coupling / KB;
///
/// // 3. Initialize Lattice
/// let mut lattice = SpinLattice::new(width, height);
///
/// // 4. Run Metropolis Simulation to reach equilibrium
/// // We use the optimized `evolve` method for better performance and stability.
/// // 300,000 steps ensures reliable convergence for CI.
/// lattice.evolve(300_000, temp, j_coupling, h_field);
///
/// // 5. Check Magnetization
/// let m = lattice.magnetization();
/// let max_m = (width * height) as i64;
///
/// // At low temp, spins align. M should be close to max_m or -max_m.
/// // We check that the absolute magnetization is significant (> 50% order).
/// let magnetization_ratio = (m as f64).abs() / (max_m as f64);
/// assert!(magnetization_ratio > 0.5, "System did not magnetize! Ratio: {}", magnetization_ratio);
/// ```
pub struct SpinLattice {
    pub width: usize,
    pub height: usize,
    pub spins: Vec<i8>, // Flattened 2D grid
}

impl SpinLattice {
    /// Creates a new random spin lattice.
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let count = width * height;
        let spins = (0..count)
            .map(|_| if rng.gen_bool(0.5) { 1 } else { -1 })
            .collect();
        SpinLattice {
            width,
            height,
            spins,
        }
    }

    /// Gets the spin at (x, y).
    pub fn get(&self, x: usize, y: usize) -> i8 {
        self.spins[y * self.width + x]
    }

    /// Sets the spin at (x, y).
    pub fn set(&mut self, x: usize, y: usize, val: i8) {
        self.spins[y * self.width + x] = val;
    }

    /// Calculates the total energy (Hamiltonian) of the lattice.
    ///
    /// Formula: H = -J * sum_<i,j> s_i s_j - h * sum_i s_i
    /// Uses Periodic Boundary Conditions.
    pub fn hamiltonian(&self, j_coupling: f64, h_field: f64) -> f64 {
        let mut interaction_sum = 0.0;
        let mut field_sum = 0.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let s = self.get(x, y);
                field_sum += s as f64;

                // Neighbors (Right and Down only to avoid double counting)
                let right_x = (x + 1) % self.width;
                let right_s = self.get(right_x, y);

                let down_y = (y + 1) % self.height;
                let down_s = self.get(x, down_y);

                interaction_sum += (s * right_s) as f64;
                interaction_sum += (s * down_s) as f64;
            }
        }

        -j_coupling * interaction_sum - h_field * field_sum
    }

    /// Calculates the total Magnetization.
    ///
    /// Formula: M = sum_i s_i
    pub fn magnetization(&self) -> i64 {
        self.spins.iter().map(|&s| s as i64).sum()
    }

    /// Performs one Metropolis algorithm step (one attempt).
    ///
    /// 1. Select a random site.
    /// 2. Calculate energy cost to flip Delta E.
    /// 3. Accept flip if Delta E < 0 OR with probability e^(-beta * Delta E).
    pub fn metropolis_step(&mut self, temperature: f64, j_coupling: f64, h_field: f64) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.width);
        let y = rng.gen_range(0..self.height);

        let s = self.get(x, y);

        // Neighbors for energy difference (all 4 neighbors needed for Delta E)
        let left_x = (x + self.width - 1) % self.width;
        let right_x = (x + 1) % self.width;
        let up_y = (y + self.height - 1) % self.height;
        let down_y = (y + 1) % self.height;

        let sum_neighbors = self.get(left_x, y) as f64
            + self.get(right_x, y) as f64
            + self.get(x, up_y) as f64
            + self.get(x, down_y) as f64;

        // Delta E = E_new - E_old
        // E_old_local = -J * s * sum_neighbors - h * s
        // E_new_local = -J * (-s) * sum_neighbors - h * (-s)
        // Delta E = 2 * J * s * sum_neighbors + 2 * h * s
        let delta_e = 2.0 * s as f64 * (j_coupling * sum_neighbors + h_field);

        let should_flip = if delta_e < 0.0 {
            true
        } else {
            let beta = 1.0 / (KB * temperature);
            let prob = (-beta * delta_e).exp();
            rng.r#gen::<f64>() < prob
        };

        if should_flip {
            self.set(x, y, -s);
        }
    }

    /// Performs multiple Metropolis steps efficiently using a lookup table and batching.
    ///
    /// This method is significantly faster than calling `metropolis_step` in a loop because:
    /// 1. It precomputes Boltzmann factors (`exp(-beta * dE)`) into a lookup table.
    /// 2. It reuses the random number generator, avoiding TLS overhead.
    pub fn evolve(&mut self, steps: usize, temperature: f64, j_coupling: f64, h_field: f64) {
        let mut rng = rand::thread_rng();
        let beta = 1.0 / (KB * temperature);

        // Precompute probabilities: lut[s_idx][sum_idx]
        // s_idx: 0 for s=-1, 1 for s=1
        // sum_idx: 0..5 for sum in {-4, -2, 0, 2, 4} mapped via (sum + 4) / 2
        let mut lut = [[0.0; 5]; 2];

        for (s_idx, lut_row) in lut.iter_mut().enumerate() {
            let s_val = if s_idx == 0 { -1.0 } else { 1.0 };
            for (sum_idx, entry) in lut_row.iter_mut().enumerate() {
                let sum_val = (sum_idx as f64 * 2.0) - 4.0;
                let delta_e = 2.0 * s_val * (j_coupling * sum_val + h_field);

                if delta_e <= 0.0 {
                    *entry = 1.1; // Always accept (value > 1.0 ensures check passes)
                } else {
                    *entry = (-beta * delta_e).exp();
                }
            }
        }

        let width = self.width;
        let height = self.height;

        for _ in 0..steps {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            // Manual inline of get() to help optimizer
            let idx = y * width + x;
            let s = self.spins[idx];

            // Neighbors
            // Use wrapping arithmetic or simple checks to avoid modulo if possible,
            // but for random access, modulo is robust.
            // Optimizing modulo:
            let left_x = if x == 0 { width - 1 } else { x - 1 };
            let right_x = if x == width - 1 { 0 } else { x + 1 };
            let up_y = if y == 0 { height - 1 } else { y - 1 };
            let down_y = if y == height - 1 { 0 } else { y + 1 };

            let neighbor_sum = self.spins[y * width + left_x] as i32
                + self.spins[y * width + right_x] as i32
                + self.spins[up_y * width + x] as i32
                + self.spins[down_y * width + x] as i32;

            // Map s (-1 or 1) to 0 or 1
            let s_idx = if s == -1 { 0 } else { 1 };
            // Map neighbor_sum (-4..4) to 0..4
            let sum_idx = ((neighbor_sum + 4) / 2) as usize;

            let prob = lut[s_idx][sum_idx];

            // If prob > 1.0, it was delta_e < 0, so flip.
            // Else compare with random.
            if prob > 1.0 || rng.r#gen::<f64>() < prob {
                self.spins[idx] = -s;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::stat_mech::KB;

    #[test]
    fn test_ising_disordered() {
        // High temperature limit. T >> J/k_B.
        // Magnetization should be near 0 on average.
        // J = 1.0 Joules (normalized).
        // T = very high. e.g. KB*T = 1000 * J.
        // T = 1000 * J / KB.
        let j_val = 1.0; // effectively unitless if we scale T
        let h_val = 0.0;
        let t_high = 100.0 * j_val / KB; // High Temp

        let width = 20;
        let height = 20;
        let mut lattice = SpinLattice::new(width, height);

        // Run many steps to equilibrate
        let steps = 10_000;
        for _ in 0..steps {
            lattice.metropolis_step(t_high, j_val, h_val);
        }

        let m = lattice.magnetization();
        let max_m = (width * height) as f64;
        let avg_m_per_spin = m as f64 / max_m;

        // In disordered phase, magnetization should be small (random fluctuations).
        // It won't be exactly 0, but should be close to 0 compared to 1.
        // <M> ~ 0.
        assert!(
            avg_m_per_spin.abs() < 0.3,
            "High T Ising should be disordered (M ~ 0). Got {}",
            avg_m_per_spin
        );
    }
}
