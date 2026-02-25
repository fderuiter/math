//! # Mean Field Games (MFG)
//!
//! MFGs model the limit of $N \to \infty$ players, where individual interactions are replaced by
//! an interaction with the "mean field" (the aggregate population). This leads to a system of two coupled PDEs:
//!
//! 1.  **Hamilton-Jacobi-Bellman (HJB)**: Backward equation. Determines the optimal value function $u(x,t)$ for a representative agent.
//! 2.  **Fokker-Planck (FP)**: Forward equation. Describes the evolution of the population density $m(x,t)$ under the optimal control.
//!
//! ##  Quick Start
//!
//! Solve a standard 1D MFG where agents try to minimize travel cost while avoiding congestion.
//!
//! ```rust
//! use math_explorer::applied::game_theory::mean_field::{MFGConfig, FixedPointSolver, MFGSolver};
//!
//! // 1. Configuration
//! // Viscosity (noise) = 0.1, Time Horizon = 1.0
//! // Grid: 50 spatial points, 100 time steps. Range: [-2.0, 2.0]
//! let config = MFGConfig::new(0.1, 1.0, 50, 100, -2.0, 2.0);
//!
//! // 2. Define Costs and Initial Distribution
//! // Running Cost: F(x, m) = m + x^2 (Agents dislike crowds + prefer origin)
//! let cost_fn = |x: f64, m: f64| -> f64 { m + x * x };
//!
//! // Terminal Cost: G(x) = x^2 (Agents want to be at origin at T)
//! let term_fn = |x: f64, _m: f64| -> f64 { x * x };
//!
//! // Initial Distribution: Gaussian centered at origin
//! let init_dist = |x: f64| -> f64 { (-x * x * 5.0).exp() };
//!
//! // 3. Solve using Fixed Point Iteration
//! let solver = FixedPointSolver::new(5); // 5 Iterations
//! let (u, m) = solver.solve(&config, &cost_fn, &term_fn, &init_dist);
//!
//! println!("Value at origin at t=0: {:.4}", u[(25, 0)]);
//! ```

pub mod physics;
pub mod solver;
pub mod types;

pub use physics::{Hamiltonian, QuadraticHamiltonian};
pub use solver::{FixedPointSolver, MFGSolver};
pub use types::MFGConfig;

// Re-export for backward compatibility, though the API has changed slightly (requires solver struct).
// We can provide a type alias if MeanFieldGame1D was just a struct.
// But it had methods. So we can re-create the struct as a wrapper.

/// Legacy wrapper for backward compatibility.
///
/// Wraps `MFGConfig` and uses `FixedPointSolver` by default.
pub struct MeanFieldGame1D {
    config: MFGConfig,
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
        Self {
            config: MFGConfig::new(
                viscosity,
                time_horizon,
                grid_points,
                time_steps,
                space_min,
                space_max,
            ),
        }
    }

    pub fn solve(
        &self,
        cost_function: impl Fn(f64, f64) -> f64,
        terminal_cost: impl Fn(f64, f64) -> f64,
        initial_distribution: impl Fn(f64) -> f64,
        iterations: usize,
    ) -> (nalgebra::DMatrix<f64>, nalgebra::DMatrix<f64>) {
        let solver = FixedPointSolver::new(iterations);
        solver.solve(
            &self.config,
            &cost_function,
            &terminal_cost,
            &initial_distribution,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfg_run_legacy() {
        let mfg = MeanFieldGame1D::new(0.1, 1.0, 50, 100, -2.0, 2.0);

        let cost_fn = |x: f64, m: f64| -> f64 { m + x * x };
        let term_fn = |x: f64, _m: f64| -> f64 { x * x };
        let init_dist = |x: f64| -> f64 { (-x * x * 5.0).exp() };

        let (u, _m) = mfg.solve(cost_fn, term_fn, init_dist, 5);

        assert_eq!(u.nrows(), 50);
        assert_eq!(u.ncols(), 101);
    }

    #[test]
    fn test_mfg_with_custom_hamiltonian() {
        let config = MFGConfig::new(0.1, 1.0, 50, 100, -2.0, 2.0);
        // Heavy particles (mass = 2.0)
        let hamiltonian = QuadraticHamiltonian::new(2.0);
        let solver = FixedPointSolver::new_with_hamiltonian(5, hamiltonian);

        let cost_fn = |x: f64, m: f64| -> f64 { m + x * x };
        let term_fn = |x: f64, _m: f64| -> f64 { x * x };
        let init_dist = |x: f64| -> f64 { (-x * x * 5.0).exp() };

        let (u, _m) = solver.solve(&config, &cost_fn, &term_fn, &init_dist);

        assert_eq!(u.nrows(), 50);
        assert_eq!(u.ncols(), 101);
    }
}
