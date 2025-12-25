use nalgebra::{DMatrix, DVector};
use crate::pure_math::analysis::ode::{OdeSystem, Solver, RungeKutta4};

/// Represents an Evolutionary Game with Replicator Dynamics.
/// dx_i/dt = x_i * ( fitness_i(x) - average_fitness(x) )
/// For matrix games: fitness_i(x) = (Ax)_i
/// average_fitness(x) = x^T A x
pub struct ReplicatorDynamics {
    pub payoff_matrix: DMatrix<f64>,
}

impl OdeSystem<DVector<f64>> for ReplicatorDynamics {
    fn derivative(&self, _t: f64, x: &DVector<f64>) -> DVector<f64> {
        self.calculate_derivative(x)
    }
}

impl ReplicatorDynamics {
    pub fn new(payoff_matrix: DMatrix<f64>) -> Self {
        assert_eq!(payoff_matrix.nrows(), payoff_matrix.ncols(), "Payoff matrix must be square");
        Self { payoff_matrix }
    }

    /// Computes the time derivative dx/dt for the population state x.
    /// Renamed to calculate_derivative to avoid conflict with OdeSystem::derivative,
    /// though inherent methods usually take precedence. Kept public for backward compat if needed,
    /// but the trait is the preferred interface.
    pub fn calculate_derivative(&self, x: &DVector<f64>) -> DVector<f64> {
        let fitness_vector = &self.payoff_matrix * x;
        let average_fitness = x.dot(&fitness_vector);

        let mut dxdt = DVector::zeros(x.len());
        for i in 0..x.len() {
            dxdt[i] = x[i] * (fitness_vector[i] - average_fitness);
        }
        dxdt
    }

    /// Backward-compatible wrapper for calculate_derivative.
    #[deprecated(note = "Use OdeSystem::derivative instead")]
    pub fn derivative(&self, x: &DVector<f64>) -> DVector<f64> {
        self.calculate_derivative(x)
    }

    /// Simulates the dynamics over time using Runge-Kutta 4 method (default).
    pub fn simulate(
        &self,
        initial_population: DVector<f64>,
        time_horizon: f64,
        dt: f64,
    ) -> Vec<(f64, DVector<f64>)> {
        self.simulate_with(initial_population, time_horizon, dt, &RungeKutta4)
    }

    /// Simulates the dynamics with a provided solver strategy.
    /// This allows swapping integrators (e.g., Euler vs RK4) for performance or stability studies.
    pub fn simulate_with<S: Solver<DVector<f64>>>(
        &self,
        initial_population: DVector<f64>,
        time_horizon: f64,
        dt: f64,
        solver: &S,
    ) -> Vec<(f64, DVector<f64>)> {
        let steps = (time_horizon / dt).ceil() as usize;
        let mut trajectory = Vec::with_capacity(steps + 1);
        let mut current_x = initial_population;
        let mut current_t = 0.0;

        trajectory.push((current_t, current_x.clone()));

        for _ in 0..steps {
            current_x = solver.solve(self, current_t, &current_x, dt);
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
        // Rock Paper Scissors matrix
        //      R   P   S
        // R    0  -1   1
        // P    1   0  -1
        // S   -1   1   0
        // (Standard zero-sum version)
        // If we add a constant to make payoffs positive (standard biology practice), dynamics are same.

        let payoff = DMatrix::from_row_slice(3, 3, &[
             0.0, -1.0,  1.0,
             1.0,  0.0, -1.0,
            -1.0,  1.0,  0.0
        ]);

        let system = ReplicatorDynamics::new(payoff);

        // Start near equilibrium (1/3, 1/3, 1/3)
        // If exact equilibrium, derivative should be 0.
        let equilibrium = DVector::from_vec(vec![1.0/3.0, 1.0/3.0, 1.0/3.0]);
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
