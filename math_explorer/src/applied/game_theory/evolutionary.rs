use nalgebra::{DMatrix, DVector};
use crate::pure_math::analysis::ode::{OdeSystem, Solver, VectorOperations};

/// Defines the rules of an Evolutionary Game.
///
/// This trait decouples the "physics" of the game (fitness calculation)
/// from the dynamics (how populations evolve).
///
/// # Implementors
/// - `MatrixGame`: A standard game where fitness is linear ($Ax$).
pub trait EvolutionaryGame {
    /// Calculates the fitness vector for the current population state.
    ///
    /// # Arguments
    /// * `population` - The current distribution of strategies (must sum to 1.0).
    ///
    /// # Returns
    /// A vector where the i-th component is the fitness of the i-th strategy.
    fn fitness(&self, population: &DVector<f64>) -> DVector<f64>;

    /// Returns the number of strategies in the game.
    fn strategy_count(&self) -> usize;
}

/// A standard Matrix Game where fitness is determined by a payoff matrix.
///
/// Fitness vector $f = A x$.
pub struct MatrixGame {
    pub payoff_matrix: DMatrix<f64>,
}

impl MatrixGame {
    /// Creates a new Matrix Game.
    ///
    /// # Panics
    /// Panics if the payoff matrix is not square.
    pub fn new(payoff_matrix: DMatrix<f64>) -> Self {
        assert_eq!(payoff_matrix.nrows(), payoff_matrix.ncols(), "Payoff matrix must be square");
        Self { payoff_matrix }
    }
}

impl EvolutionaryGame for MatrixGame {
    fn fitness(&self, population: &DVector<f64>) -> DVector<f64> {
        &self.payoff_matrix * population
    }

    fn strategy_count(&self) -> usize {
        self.payoff_matrix.nrows()
    }
}

/// Implements Replicator Dynamics for any Evolutionary Game.
///
/// The change in population share $x_i$ is given by:
/// $$ \dot{x}_i = x_i (f_i(x) - \bar{f}(x)) $$
/// where $\bar{f}(x)$ is the average fitness of the population.
pub struct ReplicatorDynamics<G: EvolutionaryGame> {
    pub game: G,
}

impl<G: EvolutionaryGame> ReplicatorDynamics<G> {
    pub fn new(game: G) -> Self {
        Self { game }
    }

    /// Simulates the dynamics over time using a provided Solver.
    ///
    /// This method enforces the simplex constraint (sum(x) = 1) after each step
    /// to prevent numerical drift.
    pub fn simulate<S>(
        &self,
        initial_population: DVector<f64>,
        time_horizon: f64,
        dt: f64,
        solver: &S,
    ) -> Vec<(f64, DVector<f64>)>
    where
        S: Solver<DVector<f64>> + ?Sized,
    {
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
                // Avoid division by zero
                if sum > 1e-12 {
                    current_x /= sum;
                }
            }

            // Clamp negative values to zero (extinction)
            // Replicator dynamics technically shouldn't go negative if started positive,
            // but numerical error can cause it.
            current_x.apply(|x| if *x < 0.0 { *x = 0.0; });
            // Re-normalize after clamping
             let sum = current_x.sum();
             if (sum - 1.0).abs() > 1e-9 && sum > 1e-12 {
                 current_x /= sum;
             }

            trajectory.push((current_t, current_x.clone()));
        }

        trajectory
    }
}

/// Implement `OdeSystem` so we can use standard Solvers.
/// The state is `DVector<f64>`.
impl<G: EvolutionaryGame> OdeSystem<DVector<f64>> for ReplicatorDynamics<G> {
    fn derivative(&self, _t: f64, x: &DVector<f64>) -> DVector<f64> {
        let fitness = self.game.fitness(x);
        let average_fitness = x.dot(&fitness);

        // dx_i/dt = x_i * (f_i - f_avg)
        // efficient component-wise operation
        let mut dxdt = x.clone();
        for i in 0..x.len() {
            dxdt[i] *= fitness[i] - average_fitness;
        }
        dxdt
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pure_math::analysis::ode::RungeKutta4;

    #[test]
    fn test_rock_paper_scissors() {
        // Rock Paper Scissors matrix
        //      R   P   S
        // R    0  -1   1
        // P    1   0  -1
        // S   -1   1   0

        let payoff = DMatrix::from_row_slice(3, 3, &[
             0.0, -1.0,  1.0,
             1.0,  0.0, -1.0,
            -1.0,  1.0,  0.0
        ]);

        let game = MatrixGame::new(payoff);
        let system = ReplicatorDynamics::new(game);

        // Start near equilibrium (1/3, 1/3, 1/3)
        // If exact equilibrium, derivative should be 0.
        let equilibrium = DVector::from_vec(vec![1.0/3.0, 1.0/3.0, 1.0/3.0]);
        let deriv = system.derivative(0.0, &equilibrium); // t doesn't matter
        assert!(deriv.norm() < 1e-9);

        // Start off-center. Should cycle (closed orbits for zero-sum RPS).
        let init = DVector::from_vec(vec![0.4, 0.3, 0.3]);
        let solver = RungeKutta4;
        let trajectory = system.simulate(init, 10.0, 0.01, &solver);

        // Check that we didn't leave the simplex significantly
        let last_state = &trajectory.last().unwrap().1;
        assert!((last_state.sum() - 1.0).abs() < 1e-6);
        assert!(last_state.min() >= -1e-9);
    }
}
