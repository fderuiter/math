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
//! use applied::game_theory::mean_field::{MFGConfig, FixedPointSolver, MFGSolver};
//!
//! // 1. Configuration
//! // Viscosity (noise) = 0.1, Time Horizon = 1.0
//! // Grid: 50 spatial points, 100 time steps. Range: [-2.0, 2.0]
//! use std::num::NonZeroUsize;
//! use applied::game_theory::mean_field::types::MFGConfigBuilder;
//! let config = MFGConfigBuilder::new()
//!     .viscosity(0.1)
//!     .time_horizon(1.0)
//!     .grid_points(NonZeroUsize::new(50).unwrap())
//!     .time_steps(NonZeroUsize::new(100).unwrap())
//!     .space_bounds(-2.0, 2.0)
//!     .unwrap()
//!     .build()
//!     .unwrap();
//!
//! // 2. Define Costs and Initial Distribution
//! // Running Cost: F(x, m) = m + x^2 (Agents dislike crowds + prefer origin)
//! use applied::game_theory::mean_field::types::{Density, Position};
//! let cost_fn = |p: Position, d: Density| -> f64 { d.0 + p.0 * p.0 };
//!
//! // Terminal Cost: G(x) = x^2 (Agents want to be at origin at T)
//! let term_fn = |p: Position, _d: Density| -> f64 { p.0 * p.0 };
//!
//! // Initial Distribution: Gaussian centered at origin
//! let init_dist = |p: Position| -> f64 { (-p.0 * p.0 * 5.0).exp() };
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
pub use types::{Density, MFGConfig, MFGConfigBuilder, Position};

// Re-export for backward compatibility, though the API has changed slightly (requires solver struct).
// We can provide a type alias if MeanFieldGame1D was just a struct.
// But it had methods. So we can re-create the struct as a wrapper.

/// Legacy wrapper for backward compatibility.
///
/// Wraps `MFGConfig` and uses `FixedPointSolver` by default.
///
/// ```rust
/// use std::num::NonZeroUsize;
/// use applied::game_theory::mean_field::{MeanFieldGame1D, MFGConfigBuilder};
/// let mfg = MeanFieldGame1D::new(0.1, 1.0, 50, 100, -2.0, 2.0);
/// ```
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
        use std::num::NonZeroUsize;
        Self {
            config: MFGConfigBuilder::new()
                .viscosity(viscosity)
                .time_horizon(time_horizon)
                .grid_points(NonZeroUsize::new(grid_points).expect("Grid points must be non-zero"))
                .time_steps(NonZeroUsize::new(time_steps).expect("Time steps must be non-zero"))
                .space_bounds(space_min, space_max)
                .expect("Failed to configure space bounds")
                .build()
                .expect("Failed to build MFGConfig with given parameters"),
        }
    }

    pub fn solve(
        &self,
        cost_function: impl Fn(Position, Density) -> f64,
        terminal_cost: impl Fn(Position, Density) -> f64,
        initial_distribution: impl Fn(Position) -> f64,
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

        let cost_fn = |p: Position, d: Density| -> f64 { d.0 + p.0 * p.0 };
        let term_fn = |p: Position, _d: Density| -> f64 { p.0 * p.0 };
        let init_dist = |p: Position| -> f64 { (-p.0 * p.0 * 5.0).exp() };

        let (u, _m) = mfg.solve(cost_fn, term_fn, init_dist, 5);

        assert_eq!(u.nrows(), 50);
        assert_eq!(u.ncols(), 101);
    }

    #[test]
    fn test_mfg_with_custom_hamiltonian() {
        use std::num::NonZeroUsize;
        let config = MFGConfigBuilder::new()
            .viscosity(0.1)
            .time_horizon(1.0)
            .grid_points(NonZeroUsize::new(50).unwrap())
            .time_steps(NonZeroUsize::new(100).unwrap())
            .space_bounds(-2.0, 2.0)
            .unwrap()
            .build()
            .unwrap();
        // Heavy particles (mass = 2.0)
        let hamiltonian = QuadraticHamiltonian::new(2.0);
        let solver = FixedPointSolver::new_with_hamiltonian(5, hamiltonian);

        let cost_fn = |p: Position, d: Density| -> f64 { d.0 + p.0 * p.0 };
        let term_fn = |p: Position, _d: Density| -> f64 { p.0 * p.0 };
        let init_dist = |p: Position| -> f64 { (-p.0 * p.0 * 5.0).exp() };

        let (u, _m) = solver.solve(&config, &cost_fn, &term_fn, &init_dist);

        assert_eq!(u.nrows(), 50);
        assert_eq!(u.ncols(), 101);
    }
}

// [cite:graph_parameters_rust]
