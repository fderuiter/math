/// The `chaos` module explores Deterministic Chaos, covering continuous flows, discrete maps,
/// and methods to quantify chaos (Lyapunov exponents and Fractal Dimensions).
///
/// Chaos theory studies the behavior of dynamical systems that are highly sensitive to initial conditions.
/// This sensitivity, often referred to as the butterfly effect, implies that small differences in initial
/// states yield widely diverging outcomes for such dynamical systems, rendering long-term prediction impossible
/// in general.

/// Discrete Chaos (The Logistic Map)
pub mod logistic {
    /// A struct representing the Logistic Map, a classic example of how complex, chaotic behavior
    /// can arise from very simple non-linear dynamical equations.
    ///
    /// The map is defined by the recurrence relation:
    /// $x_{n+1} = r x_n (1 - x_n)$
    pub struct LogisticMap {
        /// The growth rate parameter $r$.
        /// - For $r < 3$, the population eventually settles into a stable value.
        /// - For $3 < r < 3.57$, the population oscillates between 2, 4, 8, ... values (period doubling).
        /// - For $r > 3.57$, the behavior becomes chaotic.
        pub r: f64,
        /// The current state value $x$, where $0 \le x \le 1$.
        pub state: f64,
    }

    impl LogisticMap {
        /// Creates a new LogisticMap instance.
        pub fn new(r: f64, initial_state: f64) -> Self {
            LogisticMap { r, state: initial_state }
        }
    }

    impl Iterator for LogisticMap {
        type Item = f64;

        fn next(&mut self) -> Option<Self::Item> {
            self.state = self.r * self.state * (1.0 - self.state);
            Some(self.state)
        }
    }

    /// Generates points for a bifurcation diagram.
    ///
    /// This function iterates over a range of $r$ values. For each $r$, it iterates the map
    /// to discard transients and then captures the attractor states.
    ///
    /// # Arguments
    ///
    /// * `r_start` - The starting value of the parameter $r$.
    /// * `r_end` - The ending value of the parameter $r$.
    /// * `steps` - The number of discrete steps to take from `r_start` to `r_end`.
    ///
    /// # Returns
    ///
    /// A vector of tuples `(r, x)`, where `r` is the parameter value and `x` is a visited state
    /// on the attractor.
    pub fn generate_bifurcation_diagram(r_start: f64, r_end: f64, steps: usize) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        let step_size = (r_end - r_start) / steps as f64;

        for i in 0..=steps {
            let r = r_start + step_size * i as f64;
            let mut map = LogisticMap::new(r, 0.5); // Start with a generic seed like 0.5

            // Discard transient
            for _ in 0..100 {
                map.next();
            }

            // Collect attractor points
            for _ in 0..50 {
                if let Some(x) = map.next() {
                    points.push((r, x));
                }
            }
        }

        points
    }
}

/// Continuous Chaos (The Lorenz System)
pub mod lorenz {
    use nalgebra::Vector3;

    /// Represents the state of the Lorenz system $(x, y, z)$.
    #[derive(Debug, Clone, Copy)]
    pub struct LorenzState {
        /// The 3D state vector.
        pub vec: Vector3<f64>,
    }

    impl LorenzState {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            LorenzState {
                vec: Vector3::new(x, y, z),
            }
        }
    }

    /// The Lorenz System simulator.
    ///
    /// The Lorenz equations are:
    /// $\dot{x} = \sigma(y - x)$
    /// $\dot{y} = x(\rho - z) - y$
    /// $\dot{z} = xy - \beta z$
    ///
    /// These equations originally modeled atmospheric convection but became the seminal example of deterministic chaos.
    #[derive(Debug, Clone, Copy)]
    pub struct LorenzSystem {
        pub sigma: f64,
        pub rho: f64,
        pub beta: f64,
        pub state: LorenzState,
    }

    impl LorenzSystem {
        /// Creates a new LorenzSystem with standard chaotic constants: $\sigma=10, \rho=28, \beta=8/3$.
        pub fn default_chaotic(initial_state: LorenzState) -> Self {
            LorenzSystem {
                sigma: 10.0,
                rho: 28.0,
                beta: 8.0 / 3.0,
                state: initial_state,
            }
        }

        /// Calculates the derivative at a given state.
        fn derivatives(&self, state: &Vector3<f64>) -> Vector3<f64> {
            let x = state.x;
            let y = state.y;
            let z = state.z;

            let dx = self.sigma * (y - x);
            let dy = x * (self.rho - z) - y;
            let dz = x * y - self.beta * z;

            Vector3::new(dx, dy, dz)
        }

        /// Advances the system by time `dt` using the Runge-Kutta 4 (RK4) method.
        ///
        /// RK4 is chosen over Euler integration because chaotic systems are extremely sensitive to errors;
        /// Euler's method introduces local truncation errors that accumulate rapidly, leading to
        /// false trajectories.
        pub fn step(&mut self, dt: f64) {
            let y = self.state.vec;

            let k1 = self.derivatives(&y);
            let k2 = self.derivatives(&(y + k1 * (dt * 0.5)));
            let k3 = self.derivatives(&(y + k2 * (dt * 0.5)));
            let k4 = self.derivatives(&(y + k3 * dt));

            self.state.vec = y + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0);
        }
    }
}

/// Quantifying Chaos (Lyapunov Exponents)
pub mod metrics {
    use super::logistic::LogisticMap;
    use super::lorenz::LorenzSystem;
    use nalgebra::Vector3;

    /// Calculates the Lyapunov Exponent for the Logistic Map.
    ///
    /// The Lyapunov exponent $\lambda$ measures the average rate of divergence of nearby trajectories.
    /// - $\lambda > 0$: Chaotic.
    /// - $\lambda \le 0$: Regular (stable fixed point or limit cycle).
    ///
    /// Formula: $\lambda \approx \frac{1}{n} \sum_{i=0}^{n-1} \ln | f'(x_i) |$
    /// where $f'(x) = r(1 - 2x)$.
    pub fn logistic_lyapunov(r: f64, x0: f64, n: usize) -> f64 {
        let mut map = LogisticMap::new(r, x0);
        let mut sum_logs = 0.0;

        for _ in 0..n {
            let x = map.state;
            let derivative = r * (1.0 - 2.0 * x);
            // Handle singularity at x = 0.5 (f'(x) = 0).
            // In numerical simulations, if we hit exactly 0.5, log is -inf.
            // We'll treat it as a very large negative number or skip,
            // but effectively it means superstable orbit.
            // Here we just use the raw value, but callers should avoid x0=0.5.
            sum_logs += derivative.abs().ln();
            map.next();
        }

        sum_logs / n as f64
    }

    /// Calculates the largest Lyapunov Exponent for the Lorenz system using a simplified Wolf's algorithm.
    ///
    /// This method tracks two trajectories separated by a tiny distance and measures how fast they diverge.
    /// To prevent the distance from becoming too large (which would invalidate the local linearization assumption),
    /// the second trajectory is periodically renormalized (reset) towards the first one.
    ///
    /// # Arguments
    /// * `system` - The Lorenz system configuration (constants and initial state).
    /// * `time_step` - The simulation time step `dt`.
    /// * `iterations` - Number of renormalization steps to perform.
    /// * `evolution_time_per_step` - Time to evolve before measuring and renormalizing (should be a multiple of time_step).
    ///
    /// # Returns
    /// Estimated largest Lyapunov exponent.
    pub fn lorenz_lyapunov(
        mut system: LorenzSystem,
        time_step: f64,
        iterations: usize,
        evolution_time: f64,
    ) -> Result<f64, String> {
        let d0 = 1e-8;
        let steps_per_iter = (evolution_time / time_step).round() as usize;

        if steps_per_iter == 0 {
            return Err("evolution_time must be greater than time_step".to_string());
        }

        // Create shadow system
        let mut shadow_system = LorenzSystem {
            state: super::lorenz::LorenzState {
                vec: system.state.vec + Vector3::new(d0, 0.0, 0.0), // Perturb x slightly
            },
            ..system
        };

        // If distance is different from d0 (due to vector direction), normalize it strictly to d0
        let initial_diff = shadow_system.state.vec - system.state.vec;
        shadow_system.state.vec = system.state.vec + initial_diff.normalize() * d0;

        let mut sum_log_divergence = 0.0;
        let mut total_time = 0.0;

        for _ in 0..iterations {
            // Evolve both systems
            for _ in 0..steps_per_iter {
                system.step(time_step);
                shadow_system.step(time_step);
            }

            // Measure distance
            let dist_vec = shadow_system.state.vec - system.state.vec;
            let d_t = dist_vec.norm();

            if d_t == 0.0 {
                return Err("Trajectories converged completely (distance 0), cannot compute log.".to_string());
            }

            sum_log_divergence += (d_t / d0).ln();
            total_time += evolution_time;

            // Rescale: Reset shadow system to be distance d0 away from system
            // along the direction of the current separation.
            shadow_system.state.vec = system.state.vec + dist_vec.normalize() * d0;
        }

        Ok(sum_log_divergence / total_time)
    }
}

/// Fractal Dimension (Correlation Dimension)
pub mod fractals {
    use nalgebra::Vector3;

    /// Calculates the Correlation Sum $C(\epsilon)$ for the Grassberger-Procaccia algorithm.
    ///
    /// The Correlation Dimension is a measure of the dimensionality of the space occupied by a set of random points,
    /// often referred to as a type of fractal dimension.
    ///
    /// $C(\epsilon) = \frac{1}{N^2} \sum_{i, j} \Theta(\epsilon - |x_i - x_j|)$
    /// where $\Theta$ is the Heaviside step function.
    ///
    /// Note: This implementation returns the normalized count (proportion of pairs closer than $\epsilon$).
    /// The actual dimension would be the slope of $\ln(C(\epsilon))$ vs $\ln(\epsilon)$.
    pub fn correlation_dimension(trajectory: &[Vector3<f64>], epsilon: f64) -> f64 {
        let n = trajectory.len();
        if n < 2 {
            return 0.0;
        }

        let mut count = 0;
        let epsilon_sq = epsilon * epsilon;

        for i in 0..n {
            for j in (i + 1)..n {
                let dist_sq = (trajectory[i] - trajectory[j]).norm_squared();
                if dist_sq < epsilon_sq {
                    count += 1;
                }
            }
        }

        // Multiply by 2 because of symmetry (pair (i,j) and (j,i)), and divide by total possible pairs N*(N-1)
        // C(eps) = (2 * count) / (N * (N - 1))

        (2.0 * count as f64) / ((n * (n - 1)) as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;
    use approx::assert_relative_eq;

    #[test]
    fn test_logistic_chaos_lyapunov() {
        // For r=3.9, the map is chaotic, so Lyapunov exponent should be positive.
        // Theoretical value is approx 0.496.
        // We use x0=0.1 to avoid starting at the critical point x=0.5.
        let lambda = metrics::logistic_lyapunov(3.9, 0.1, 1000);
        assert!(lambda > 0.0, "Lyapunov exponent for r=3.9 should be positive, got {}", lambda);
    }

    #[test]
    fn test_logistic_stability_lyapunov() {
        // For r=2.5, the map is stable (fixed point at 0.6), so Lyapunov exponent should be negative.
        // f'(x*) = 2.5(1 - 1.2) = 2.5(-0.2) = -0.5. ln(0.5) approx -0.693.
        // We use x0=0.1.
        let lambda = metrics::logistic_lyapunov(2.5, 0.1, 1000);
        assert!(lambda < 0.0, "Lyapunov exponent for r=2.5 should be negative, got {}", lambda);
        // We can be more precise
        assert!((lambda - -0.6931).abs() < 0.1, "Expected approx -0.693, got {}", lambda);
    }

    #[test]
    fn test_lorenz_boundedness() {
        // Run Lorenz system and ensure it stays within reasonable bounds.
        // The Lorenz attractor is contained within a specific region of space.
        let state = lorenz::LorenzState::new(1.0, 1.0, 1.0);
        let mut system = lorenz::LorenzSystem::default_chaotic(state);

        let dt = 0.01;
        for _ in 0..1000 {
            system.step(dt);
            let s = system.state.vec;
            assert!(s.x.abs() < 100.0, "x diverged: {}", s.x);
            assert!(s.y.abs() < 100.0, "y diverged: {}", s.y);
            assert!(s.z.abs() < 100.0, "z diverged: {}", s.z);
        }
    }

    #[test]
    fn test_bifurcation_diagram_generation() {
        let points = logistic::generate_bifurcation_diagram(3.0, 4.0, 10);
        // steps=10 means 11 values of r. Each has 50 points. Total 550 points.
        assert_eq!(points.len(), 11 * 50);
        // Check bounds
        for (r, x) in points {
            assert!(r >= 3.0 && r <= 4.0);
            assert!(x >= 0.0 && x <= 1.0);
        }
    }

    #[test]
    fn test_correlation_dimension_simple() {
        // Create a line of points: (0,0,0), (1,0,0), (2,0,0) ...
        // Dimension should be 1. But we just test the C(epsilon) calculation here.
        let mut traj = Vec::new();
        for i in 0..10 {
            traj.push(na::Vector3::new(i as f64, 0.0, 0.0));
        }

        // Epsilon = 1.1. Pairs with dist < 1.1 are adjacent points.
        // Pairs (0,1), (1,2), ..., (8,9). There are 9 such pairs.
        // Total pairs N(N-1) = 10*9 = 90.
        // Count = 9 * 2 = 18. (symmetric)
        // C = 18 / 90 = 0.2.

        let c = fractals::correlation_dimension(&traj, 1.1);
        assert_relative_eq!(c, 0.2);
    }
}
