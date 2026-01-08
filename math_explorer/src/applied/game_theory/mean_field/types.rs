use nalgebra::DMatrix;
use super::solver::{MFGSolver, FixedPointSolver};

/// Represents a 1D Mean Field Game (MFG) configuration.
///
/// Mean Field Games model systems with a large number of indistinguishable rational agents.
/// Instead of tracking $N$ players, we model the limit as $N \to \infty$.
///
/// The solution is characterized by a coupled system of Partial Differential Equations (PDEs):
///
/// 1. **Hamilton-Jacobi-Bellman (HJB) Equation**: Describes the optimal control problem of a single agent.
///    It evolves **backward in time** (from terminal cost to present).
///    $$ -\partial_t u + H(x, \nabla u) - \nu \Delta u = F(x, m) $$
///
/// 2. **Fokker-Planck (Kolmogorov Forward) Equation**: Describes the evolution of the population distribution.
///    It evolves **forward in time** (from initial distribution).
///    $$ \partial_t m + \nabla \cdot (m v) - \nu \Delta m = 0 $$
///    where the drift $v = -\nabla H_p = -\nabla u$ (assuming quadratic Hamiltonian).
///
/// # Field Descriptions
/// - `viscosity` ($\nu$): Diffusion parameter representing noise/randomness in agent motion.
/// - `time_horizon` ($T$): Duration of the game.
/// - `dt`, `dx`: Discretization steps.
#[derive(Debug, Clone, Copy)]
pub struct MeanFieldGame1D {
    pub viscosity: f64,          // nu
    pub time_horizon: f64,       // T
    pub time_steps: usize,       // Nt
    pub grid_points: usize,      // Nx
    pub dt: f64,
    pub dx: f64,
    pub space_min: f64,
    pub space_max: f64,
}

impl MeanFieldGame1D {
    pub fn new(
        viscosity: f64,
        time_horizon: f64,
        grid_points: usize,
        time_steps: usize,
        space_min: f64,
        space_max: f64,
    ) -> Self {
        let dt = time_horizon / (time_steps as f64);
        let dx = (space_max - space_min) / ((grid_points - 1) as f64);
        Self {
            viscosity,
            time_horizon,
            time_steps,
            grid_points,
            dt,
            dx,
            space_min,
            space_max,
        }
    }

    /// Legacy wrapper for backward compatibility.
    /// This implementation allows the existing `MeanFieldGame1D::solve` method to work
    /// by delegating to the new `FixedPointSolver`.
    ///
    /// # Parameters
    /// - `cost_function` $F(x, m)$: The running cost. Often penalizes congestion (large $m$).
    /// - `terminal_cost` $G(x, m)$: Cost at final time $T$.
    /// - `initial_distribution` $m_0(x)$: Starting population density.
    /// - `iterations`: Number of forward-backward sweeps.
    ///
    /// # Returns
    /// Tuple `(u, m)` containing the value function and distribution matrices.
    /// Rows correspond to space, columns to time.
    pub fn solve(
        &self,
        cost_function: impl Fn(f64, f64) -> f64,
        terminal_cost: impl Fn(f64, f64) -> f64,
        initial_distribution: impl Fn(f64) -> f64,
        iterations: usize,
    ) -> (DMatrix<f64>, DMatrix<f64>) {
        let solver = FixedPointSolver::new(iterations);
        solver.solve(
            self,
            &cost_function,
            &terminal_cost,
            &initial_distribution
        )
    }

    /// Solves the coupled system using a specified solver strategy.
    ///
    /// This method allows dependency injection of the solver algorithm.
    pub fn solve_with<S: MFGSolver>(
        &self,
        solver: &S,
        cost_function: impl Fn(f64, f64) -> f64,
        terminal_cost: impl Fn(f64, f64) -> f64,
        initial_distribution: impl Fn(f64) -> f64,
    ) -> (DMatrix<f64>, DMatrix<f64>) {
        solver.solve(
            self,
            &cost_function,
            &terminal_cost,
            &initial_distribution
        )
    }
}
