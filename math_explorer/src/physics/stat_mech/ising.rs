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
/// // Note: This is a probabilistic process.
/// for _ in 0..100_000 {
///     lattice.metropolis_step(temp, j_coupling, h_field);
/// }
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
        Self::new_with_rng(width, height, &mut rng)
    }

    /// Creates a new random spin lattice using a provided RNG.
    pub fn new_with_rng<R: Rng + ?Sized>(width: usize, height: usize, rng: &mut R) -> Self {
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
    #[deprecated(since = "0.2.0", note = "Use MetropolisSolver::step instead")]
    pub fn metropolis_step(&mut self, temperature: f64, j_coupling: f64, h_field: f64) {
        let rng = rand::thread_rng();
        let mut solver = MetropolisSolver::new(rng);
        solver.step(self, temperature, j_coupling, h_field);
    }

    /// Performs multiple Metropolis steps efficiently using a lookup table and batching.
    ///
    /// This method is significantly faster than calling `metropolis_step` in a loop.
    #[deprecated(since = "0.2.0", note = "Use MetropolisSolver::evolve instead")]
    pub fn evolve(&mut self, steps: usize, temperature: f64, j_coupling: f64, h_field: f64) {
        let rng = rand::thread_rng();
        let mut solver = MetropolisSolver::new(rng);
        solver.evolve(self, steps, temperature, j_coupling, h_field);
    }
}

/// A trait for strategies that evolve the Ising Model.
pub trait IsingSolver {
    /// Performs one step of the simulation.
    fn step(&mut self, lattice: &mut SpinLattice, temperature: f64, j_coupling: f64, h_field: f64);

    /// Evolves the simulation for a number of steps.
    fn evolve(
        &mut self,
        lattice: &mut SpinLattice,
        steps: usize,
        temperature: f64,
        j_coupling: f64,
        h_field: f64,
    );
}

/// A Metropolis-Hastings solver for the Ising Model.
///
/// Wraps an RNG to allow for deterministic simulations.
pub struct MetropolisSolver<R> {
    rng: R,
}

impl<R: Rng> MetropolisSolver<R> {
    /// Creates a new solver with the given RNG.
    pub fn new(rng: R) -> Self {
        Self { rng }
    }
}

impl<R: Rng> IsingSolver for MetropolisSolver<R> {
    fn step(&mut self, lattice: &mut SpinLattice, temperature: f64, j_coupling: f64, h_field: f64) {
        let x = self.rng.gen_range(0..lattice.width);
        let y = self.rng.gen_range(0..lattice.height);

        let s = lattice.get(x, y);

        // Neighbors
        let left_x = (x + lattice.width - 1) % lattice.width;
        let right_x = (x + 1) % lattice.width;
        let up_y = (y + lattice.height - 1) % lattice.height;
        let down_y = (y + 1) % lattice.height;

        let sum_neighbors = lattice.get(left_x, y) as f64
            + lattice.get(right_x, y) as f64
            + lattice.get(x, up_y) as f64
            + lattice.get(x, down_y) as f64;

        let delta_e = 2.0 * s as f64 * (j_coupling * sum_neighbors + h_field);

        let should_flip = if delta_e < 0.0 {
            true
        } else {
            let beta = 1.0 / (KB * temperature);
            let prob = (-beta * delta_e).exp();
            self.rng.r#gen::<f64>() < prob
        };

        if should_flip {
            lattice.set(x, y, -s);
        }
    }

    fn evolve(
        &mut self,
        lattice: &mut SpinLattice,
        steps: usize,
        temperature: f64,
        j_coupling: f64,
        h_field: f64,
    ) {
        let beta = 1.0 / (KB * temperature);

        // Precompute probabilities
        let mut lut = [[0.0; 5]; 2];

        for (s_idx, lut_row) in lut.iter_mut().enumerate() {
            let s_val = if s_idx == 0 { -1.0 } else { 1.0 };
            for (sum_idx, entry) in lut_row.iter_mut().enumerate() {
                let sum_val = (sum_idx as f64 * 2.0) - 4.0;
                let delta_e = 2.0 * s_val * (j_coupling * sum_val + h_field);

                if delta_e <= 0.0 {
                    *entry = 1.1; // Always accept
                } else {
                    *entry = (-beta * delta_e).exp();
                }
            }
        }

        let width = lattice.width;
        let height = lattice.height;

        for _ in 0..steps {
            let x = self.rng.gen_range(0..width);
            let y = self.rng.gen_range(0..height);

            let idx = y * width + x;
            let s = lattice.spins[idx];

            let left_x = if x == 0 { width - 1 } else { x - 1 };
            let right_x = if x == width - 1 { 0 } else { x + 1 };
            let up_y = if y == 0 { height - 1 } else { y - 1 };
            let down_y = if y == height - 1 { 0 } else { y + 1 };

            let neighbor_sum = lattice.spins[y * width + left_x] as i32
                + lattice.spins[y * width + right_x] as i32
                + lattice.spins[up_y * width + x] as i32
                + lattice.spins[down_y * width + x] as i32;

            let s_idx = if s == -1 { 0 } else { 1 };
            let sum_idx = ((neighbor_sum + 4) / 2) as usize;

            let prob = lut[s_idx][sum_idx];

            if prob > 1.0 || self.rng.r#gen::<f64>() < prob {
                lattice.spins[idx] = -s;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::stat_mech::KB;

    use rand::SeedableRng;
    use rand::rngs::StdRng;

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
        let mut solver = MetropolisSolver::new(rand::thread_rng());
        solver.evolve(&mut lattice, steps, t_high, j_val, h_val);

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

    #[test]
    fn test_ising_deterministic() {
        let mut rng = StdRng::seed_from_u64(42);
        let width = 10;
        let height = 10;
        let mut lattice = SpinLattice::new_with_rng(width, height, &mut rng);

        // Capture initial state
        let initial_spins = lattice.spins.clone();

        let mut solver = MetropolisSolver::new(rng);
        // Run a few steps
        solver.evolve(&mut lattice, 100, 1.0, 1.0, 0.0);

        // Ensure state changed
        assert_ne!(lattice.spins, initial_spins);

        // Re-run with same seed
        let mut rng2 = StdRng::seed_from_u64(42);
        let mut lattice2 = SpinLattice::new_with_rng(width, height, &mut rng2);
        let mut solver2 = MetropolisSolver::new(rng2);
        solver2.evolve(&mut lattice2, 100, 1.0, 1.0, 0.0);

        assert_eq!(
            lattice.spins, lattice2.spins,
            "Simulation should be deterministic"
        );
    }
}
