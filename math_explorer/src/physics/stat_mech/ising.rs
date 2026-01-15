use super::KB;
use rand::Rng;

/// A 2D Spin Lattice for the Ising Model.
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
