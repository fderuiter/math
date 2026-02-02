//! Mean Field Games (MFG)
//!
//! MFGs model the limit of $N \to \infty$ players, using coupled PDE systems:
//! - **HJB Equation**: Optimal control for a single agent (Backward).
//! - **Fokker-Planck Equation**: Evolution of the population distribution (Forward).
//!
//! This module provides the configuration types and solver strategies for 1D MFGs.

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
