use nalgebra::{DMatrix, DVector};

/// Represents an Evolutionary Game solved via **Replicator Dynamics**.
///
/// In evolutionary game theory, we study how the composition of a population changes over time
/// based on the fitness of different strategies. The governing equation is the Replicator Equation:
///
/// $$ \dot{x}_i = x_i \left( f_i(x) - \bar{f}(x) \right) $$
///
/// Where:
/// - $x_i$ is the proportion of the population playing strategy $i$.
/// - $f_i(x) = (Ax)_i$ is the fitness of strategy $i$ against population state $x$.
/// - $\bar{f}(x) = x^T A x$ is the average fitness of the population.
///
/// Strategies with better-than-average fitness grow in prevalence; those with worse-than-average shrink.
pub struct ReplicatorDynamics {
    pub payoff_matrix: DMatrix<f64>,
}

impl ReplicatorDynamics {
    /// Creates a new system with the given payoff matrix $A$.
    pub fn new(payoff_matrix: DMatrix<f64>) -> Self {
        assert_eq!(
            payoff_matrix.nrows(),
            payoff_matrix.ncols(),
            "Payoff matrix must be square"
        );
        Self { payoff_matrix }
    }

    /// Computes the time derivative $\dot{x}$ for the population state $x$.
    pub fn derivative(&self, x: &DVector<f64>) -> DVector<f64> {
        let fitness_vector = &self.payoff_matrix * x;
        let average_fitness = x.dot(&fitness_vector);

        let mut dxdt = DVector::zeros(x.len());
        for i in 0..x.len() {
            dxdt[i] = x[i] * (fitness_vector[i] - average_fitness);
        }
        dxdt
    }

    /// Simulates the dynamics over time using the **Runge-Kutta 4** method.
    ///
    /// # Parameters
    /// - `initial_population`: Vector of proportions summing to 1.0.
    /// - `time_horizon`: Total time to simulate.
    /// - `dt`: Time step size.
    ///
    /// # Returns
    /// A time-series of population states: `Vec<(Time, State)>`.
    pub fn simulate(
        &self,
        initial_population: DVector<f64>,
        time_horizon: f64,
        dt: f64,
    ) -> Vec<(f64, DVector<f64>)> {
        let steps = (time_horizon / dt).ceil() as usize;
        let mut trajectory = Vec::with_capacity(steps + 1);
        let mut current_x = initial_population;
        let mut current_t = 0.0;

        trajectory.push((current_t, current_x.clone()));

        for _ in 0..steps {
            let k1 = self.derivative(&current_x);
            let k2 = self.derivative(&(&current_x + 0.5 * dt * &k1));
            let k3 = self.derivative(&(&current_x + 0.5 * dt * &k2));
            let k4 = self.derivative(&(&current_x + dt * &k3));

            current_x += (dt / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            current_t += dt;

            // Normalize to prevent numerical drift from simplex
            let sum = current_x.sum();
            if (sum - 1.0).abs() > 1e-9 {
                current_x /= sum;
            }

            trajectory.push((current_t, current_x.clone()));
        }

        trajectory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rock_paper_scissors() {
        // Rock Paper Scissors matrix (Zero-Sum)
        //      R   P   S
        // R    0  -1   1
        // P    1   0  -1
        // S   -1   1   0
        //
        // This is a "Cyclic" game. The interior equilibrium is (1/3, 1/3, 1/3).
        // Trajectories should cycle around it.

        let payoff =
            DMatrix::from_row_slice(3, 3, &[0.0, -1.0, 1.0, 1.0, 0.0, -1.0, -1.0, 1.0, 0.0]);

        let system = ReplicatorDynamics::new(payoff);

        // Start near equilibrium (1/3, 1/3, 1/3)
        // If exact equilibrium, derivative should be 0.
        let equilibrium = DVector::from_vec(vec![1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0]);
        let deriv = system.derivative(&equilibrium);
        assert!(deriv.norm() < 1e-9);

        // Start off-center. Should cycle (closed orbits for zero-sum RPS).
        let init = DVector::from_vec(vec![0.4, 0.3, 0.3]);
        let trajectory = system.simulate(init, 10.0, 0.01);

        // Check that we didn't leave the simplex significantly
        let last_state = &trajectory.last().unwrap().1;
        assert!((last_state.sum() - 1.0).abs() < 1e-6);
        assert!(last_state.min() >= -1e-9);
    }
}
