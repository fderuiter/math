use nalgebra::DMatrix;
use super::types::MeanFieldGame1D;

/// Defines a strategy for solving Mean Field Games.
pub trait MFGSolver {
    /// Solves the MFG coupled system.
    ///
    /// # Parameters
    /// - `game`: The game configuration (grid, physics).
    /// - `cost_function`: $F(x, m)$ - Running cost.
    /// - `terminal_cost`: $G(x, m)$ - Terminal cost.
    /// - `initial_distribution`: $m_0(x)$ - Initial density.
    ///
    /// # Returns
    /// Tuple `(u, m)` containing the value function and distribution matrices.
    fn solve(
        &self,
        game: &MeanFieldGame1D,
        cost_function: &dyn Fn(f64, f64) -> f64,
        terminal_cost: &dyn Fn(f64, f64) -> f64,
        initial_distribution: &dyn Fn(f64) -> f64,
    ) -> (DMatrix<f64>, DMatrix<f64>);
}

/// Solves the coupled system using a **Fixed-Point Iteration** scheme.
///
/// The algorithm iterates between:
/// 1. Solving HJB **backward** given the current guess for distribution $m$.
/// 2. Solving Fokker-Planck **forward** given the optimal control from $u$.
pub struct FixedPointSolver {
    /// Number of forward-backward sweeps.
    pub iterations: usize,
}

impl FixedPointSolver {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }
}

impl Default for FixedPointSolver {
    fn default() -> Self {
        Self { iterations: 100 }
    }
}

impl MFGSolver for FixedPointSolver {
    fn solve(
        &self,
        game: &MeanFieldGame1D,
        cost_function: &dyn Fn(f64, f64) -> f64,
        terminal_cost: &dyn Fn(f64, f64) -> f64,
        initial_distribution: &dyn Fn(f64) -> f64,
    ) -> (DMatrix<f64>, DMatrix<f64>) {
        let nx = game.grid_points;
        let nt = game.time_steps;

        // Initialize m (density) and u (value)
        // m[i, n] is density at x_i, t_n
        // u[i, n] is value at x_i, t_n
        let mut m = DMatrix::zeros(nx, nt + 1);
        let mut u = DMatrix::zeros(nx, nt + 1);

        // Initialize m at t=0
        for i in 0..nx {
            let x = game.space_min + (i as f64) * game.dx;
            m[(i, 0)] = initial_distribution(x);
        }

        // Normalize initial distribution
        let sum_0: f64 = m.column(0).sum();
        if sum_0 > 1e-9 {
            for i in 0..nx {
                m[(i, 0)] /= sum_0;
            }
        }

        // Make an initial guess for m for all t (copy m0)
        for n in 1..=nt {
            for i in 0..nx {
                m[(i, n)] = m[(i, 0)];
            }
        }

        for _iter in 0..self.iterations {
            // 1. Solve HJB Backward
            // Terminal condition
            for i in 0..nx {
                let x = game.space_min + (i as f64) * game.dx;
                u[(i, nt)] = terminal_cost(x, m[(i, nt)]);
            }

            // Backward in time
            for n in (0..nt).rev() {
                for i in 1..nx - 1 {
                    let x = game.space_min + (i as f64) * game.dx;

                    // Finite differences
                    // Explicit Euler backward
                    let du_dx = (u[(i + 1, n + 1)] - u[(i - 1, n + 1)]) / (2.0 * game.dx);
                    let d2u_dx2 = (u[(i + 1, n + 1)] - 2.0 * u[(i, n + 1)] + u[(i - 1, n + 1)]) / (game.dx * game.dx);

                    let hamiltonian = 0.5 * du_dx * du_dx; // H(p) = p^2 / 2
                    let running_cost = cost_function(x, m[(i, n + 1)]);

                    // u(n) = u(n+1) - dt * (H - nu * d2u - F)
                    u[(i, n)] = u[(i, n + 1)] - game.dt * (hamiltonian - game.viscosity * d2u_dx2 - running_cost);
                }
                // Boundary conditions (Neumann 0)
                u[(0, n)] = u[(1, n)];
                u[(nx - 1, n)] = u[(nx - 2, n)];
            }

            // 2. Solve Fokker-Planck Forward
            // m(0) is fixed.
            for n in 0..nt {
                for i in 1..nx - 1 {
                    // Explicit Euler forward
                    // v = -dH/dp = -p = -grad u.
                    let du_dx = (u[(i + 1, n)] - u[(i - 1, n)]) / (2.0 * game.dx);
                    let v = -du_dx;

                    let d2m_dx2 = (m[(i + 1, n)] - 2.0 * m[(i, n)] + m[(i - 1, n)]) / (game.dx * game.dx);

                    // Upwind for drift term
                    let drift_flux = if v > 0.0 {
                         (m[(i, n)] * v - m[(i - 1, n)] * v) / game.dx
                    } else {
                         (m[(i + 1, n)] * v - m[(i, n)] * v) / game.dx
                    };

                    let rhs = -drift_flux + game.viscosity * d2m_dx2;
                    m[(i, n + 1)] = m[(i, n)] + game.dt * rhs;
                }
                // Reflective boundaries
                m[(0, n + 1)] = m[(1, n + 1)];
                m[(nx - 1, n + 1)] = m[(nx - 2, n + 1)];

                // Normalize mass to prevent explosion/vanishing
                let sum: f64 = m.column(n + 1).sum();
                if sum > 1e-9 {
                    for i in 0..nx {
                        m[(i, n + 1)] /= sum;
                    }
                }
            }
        }

        (u, m)
    }
}
