//! The `chaos` module explores Deterministic Chaos, covering continuous flows, discrete maps,
//! and methods to quantify chaos (Lyapunov exponents and Fractal Dimensions).
//!
//! Chaos theory studies the behavior of dynamical systems that are highly sensitive to initial conditions.
//! This sensitivity, often referred to as the butterfly effect, implies that small differences in initial
//! states yield widely diverging outcomes for such dynamical systems, rendering long-term prediction impossible
//! in general.

/// Discrete Chaos (The Logistic Map)
///
/// This module implements the Logistic Map, a classic example of how complex, chaotic behavior
/// can arise from very simple non-linear dynamical equations.
///
/// The map is defined by the recurrence relation:
/// $x_{n+1} = r x_n (1 - x_n)$
pub mod logistic {
    /// Number of iterations to discard as transient before collecting attractor points.
    pub const TRANSIENT_STEPS: usize = 100;
    /// Number of points to collect for the attractor visualization.
    pub const ATTRACTOR_POINTS: usize = 50;

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
            for _ in 0..TRANSIENT_STEPS {
                map.next();
            }

            // Collect attractor points
            for _ in 0..ATTRACTOR_POINTS {
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
    use crate::pure_math::analysis::ode::{OdeSystem, RungeKutta4, Solver};

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
        /// The Prandtl number $\sigma$, representing the ratio of momentum diffusivity to thermal diffusivity.
        pub sigma: f64,
        /// The Rayleigh number $\rho$, representing the temperature difference driving the convection.
        pub rho: f64,
        /// The geometric factor $\beta$, related to the aspect ratio of the convection rolls.
        pub beta: f64,
        /// The current state of the system.
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

        /// Advances the system by time `dt` using the Runge-Kutta 4 (RK4) method.
        ///
        /// This now delegates to the generic `RungeKutta4` solver, ensuring DRY and type safety.
        pub fn step(&mut self, dt: f64) {
             self.state.vec = RungeKutta4::step(self, 0.0, &self.state.vec, dt);
        }

        /// Advances the system by time `dt` using a provided solver strategy.
        ///
        /// This allows the user to switch integrators (e.g., Euler, RK4) dynamically.
        pub fn step_with<S: Solver<Vector3<f64>>>(&mut self, solver: &S, dt: f64) {
            self.state.vec = solver.solve(self, 0.0, &self.state.vec, dt);
        }
    }

    impl OdeSystem<Vector3<f64>> for LorenzSystem {
        /// Calculates the derivative at a given state.
        fn derivative(&self, _t: f64, state: &Vector3<f64>) -> Vector3<f64> {
            let x = state.x;
            let y = state.y;
            let z = state.z;

            let dx = self.sigma * (y - x);
            let dy = x * (self.rho - z) - y;
            let dz = x * y - self.beta * z;

            Vector3::new(dx, dy, dz)
        }
    }
}

/// Quantifying Chaos (Lyapunov Exponents)
pub mod metrics {
    use super::logistic::LogisticMap;
    use nalgebra::Vector3;
    use crate::pure_math::analysis::ode::{OdeSystem, Solver};

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
            sum_logs += derivative.abs().ln();
            map.next();
        }

        sum_logs / n as f64
    }

    /// Calculates the largest Lyapunov Exponent for a 3D continuous dynamical system.
    ///
    /// This uses Wolf's algorithm, tracking two trajectories separated by a tiny distance
    /// and periodically renormalizing the shadow trajectory.
    ///
    /// # Arguments
    /// * `system` - The dynamical system implementing `OdeSystem<Vector3<f64>>`.
    /// * `initial_state` - The starting state for the main trajectory.
    /// * `time_step` - The simulation time step `dt`.
    /// * `iterations` - Number of renormalization steps to perform.
    /// * `evolution_time` - Time to evolve before measuring and renormalizing.
    ///
    /// # Returns
    /// Estimated largest Lyapunov exponent.
    pub fn lorenz_lyapunov<S, Sol>(
        system: &S,
        solver: &Sol,
        initial_state: Vector3<f64>,
        time_step: f64,
        iterations: usize,
        evolution_time: f64,
    ) -> Result<f64, String>
    where
        S: OdeSystem<Vector3<f64>> + ?Sized,
        Sol: Solver<Vector3<f64>>,
    {
        let d0 = 1e-8;
        let steps_per_iter = (evolution_time / time_step).round() as usize;

        if steps_per_iter == 0 {
            return Err("evolution_time must be greater than time_step".to_string());
        }

        let mut current_state = initial_state;
        let mut shadow_state = current_state + Vector3::new(d0, 0.0, 0.0);

        // Normalize initial separation
        let initial_diff = shadow_state - current_state;
        shadow_state = current_state + initial_diff.normalize() * d0;

        let mut sum_log_divergence = 0.0;
        let mut total_time = 0.0;

        for _ in 0..iterations {
            // Evolve both systems
            for _ in 0..steps_per_iter {
                current_state = solver.solve(system, 0.0, &current_state, time_step);
                shadow_state = solver.solve(system, 0.0, &shadow_state, time_step);
            }

            // Measure distance
            let dist_vec = shadow_state - current_state;
            let d_t = dist_vec.norm();

            if d_t == 0.0 {
                return Err("Trajectories converged completely (distance 0), cannot compute log.".to_string());
            }

            sum_log_divergence += (d_t / d0).ln();
            total_time += evolution_time;

            // Rescale: Reset shadow system to be distance d0 away from system
            // along the direction of the current separation.
            shadow_state = current_state + dist_vec.normalize() * d0;
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

        // Profiler Optimization: Sort by X-coordinate to prune search space.
        // This reduces complexity from O(N^2) to O(N * k) where k is the neighborhood size.
        // For small epsilon (the relevant case for fractal dimension), this yields massive speedups.

        let mut sorted_traj = trajectory.to_vec();
        // Use unstable_sort_by for speed; floating point sort handles NaN by treating as equal (safe assumption here).
        sorted_traj.sort_unstable_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));

        for i in 0..n {
            let p1 = sorted_traj[i];

            // Because data is sorted by X, we only look ahead.
            // We can stop as soon as the X-distance exceeds epsilon.
            for p2 in sorted_traj.iter().skip(i + 1) {
                let dx = p2.x - p1.x; // p2.x >= p1.x due to sort

                if dx > epsilon {
                    break;
                }

                let dy = p1.y - p2.y;
                let dz = p1.z - p2.z;

                // Manual squared distance check to avoid Vector3 overhead in the hot loop
                if dx * dx + dy * dy + dz * dz < epsilon_sq {
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
    fn test_lorenz_lyapunov_strategy() {
        use crate::pure_math::analysis::ode::{RungeKutta4, Euler};
        let state = lorenz::LorenzState::new(1.0, 1.0, 1.0);
        let system = lorenz::LorenzSystem::default_chaotic(state);

        // Test with RK4
        let lambda_rk4 = metrics::lorenz_lyapunov(
            &system,
            &RungeKutta4,
            na::Vector3::new(10.0, 10.0, 10.0),
            0.01,
            100,
            1.0
        ).unwrap();

        assert!(lambda_rk4 > 0.0, "Lorenz with RK4 should be chaotic, got {}", lambda_rk4);

        // Test with Euler (less accurate, but should run)
        let lambda_euler = metrics::lorenz_lyapunov(
            &system,
            &Euler,
            na::Vector3::new(10.0, 10.0, 10.0),
            0.0001, // Euler needs smaller step for stability
            100,
            1.0
        ).unwrap();

        assert!(lambda_euler > 0.0, "Lorenz with Euler should be chaotic, got {}", lambda_euler);
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
