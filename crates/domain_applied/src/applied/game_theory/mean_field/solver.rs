use super::physics::{Hamiltonian, QuadraticHamiltonian};
use super::types::{Density, MFGConfig, Position};
use nalgebra::DMatrix;

/// Strategy trait for solving Mean Field Games.
///
/// Implementors of this trait provide a specific algorithm (e.g., Fixed Point, Newton-Raphson, Deep Learning)
/// to solve the coupled HJB-FP system defined by the `MFGConfig`.
pub trait MFGSolver {
    /// Solves the Mean Field Game coupled system.
    ///
    /// # Parameters
    /// - `config`: The physical and grid configuration.
    /// - `cost_function`: $F(x, m)$ - Running cost.
    /// - `terminal_cost`: $G(x, m)$ - Terminal cost.
    /// - `initial_distribution`: $m_0(x)$ - Initial population distribution.
    ///
    /// # Returns
    /// Tuple `(u, m)` containing the value function and distribution matrices.
    #[verified_engine::verified]
    fn solve(
        &self,
        config: &MFGConfig,
        cost_function: &impl Fn(Position, Density) -> f64,
        terminal_cost: &impl Fn(Position, Density) -> f64,
        initial_distribution: &impl Fn(Position) -> f64,
    ) -> (DMatrix<f64>, DMatrix<f64>);
}

/// A Fixed-Point Iteration solver for Mean Field Games.
///
/// Iterates back and forth between the HJB (backward) and Fokker-Planck (forward) equations
/// until convergence (or for a fixed number of iterations).
pub struct FixedPointSolver<H: Hamiltonian> {
    pub iterations: usize,
    pub hamiltonian: H,
}

impl FixedPointSolver<QuadraticHamiltonian> {
    /// Creates a new solver with a default Quadratic Hamiltonian ($H = p^2/2$).
    #[verified_engine::verified]
    pub fn new(iterations: usize) -> Self {
        Self {
            iterations,
            hamiltonian: QuadraticHamiltonian::default(),
        }
    }
}

impl<H: Hamiltonian> FixedPointSolver<H> {
    /// Creates a new solver with a custom Hamiltonian strategy.
    #[verified_engine::verified]
    pub fn new_with_hamiltonian(iterations: usize, hamiltonian: H) -> Self {
        Self {
            iterations,
            hamiltonian,
        }
    }
}

impl<H: Hamiltonian> MFGSolver for FixedPointSolver<H> {
    #[verified_engine::verified]
    fn solve(
        &self,
        config: &MFGConfig,
        cost_function: &impl Fn(Position, Density) -> f64,
        terminal_cost: &impl Fn(Position, Density) -> f64,
        initial_distribution: &impl Fn(Position) -> f64,
    ) -> (DMatrix<f64>, DMatrix<f64>) {
        let nx = config.grid_points.get();
        let nt = config.time_steps.get();

        // Initialize m (density) and u (value)
        let mut m = DMatrix::zeros(nx, nt + 1);
        let mut u = DMatrix::zeros(nx, nt + 1);

        // Precompute x values
        let xs: Vec<f64> = (0..nx)
            .map(|i| config.space_min + (i as f64) * config.dx)
            .collect();

        // Initialize m at t=0
        for i in 0..nx {
            m[(i, 0)] = initial_distribution(Position(xs[i]));
        }

        // Normalize initial distribution
        let sum_0: f64 = m.column(0).sum();
        if sum_0 > 1e-9 {
            m.column_mut(0).scale_mut(1.0 / sum_0);
        }

        // Make an initial guess for m for all t (copy m0)
        let m0 = m.column(0).clone_owned();
        for n in 1..=nt {
            m.column_mut(n).copy_from(&m0);
        }

        for _iter in 0..self.iterations {
            // 1. Solve HJB Backward
            // Terminal condition
            for i in 0..nx {
                u[(i, nt)] = terminal_cost(Position(xs[i]), Density(m[(i, nt)]));
            }

            // Backward in time
            for n in (0..nt).rev() {
                for i in 1..nx - 1 {
                    let x = xs[i];

                    // Finite differences
                    // u(i, n) = u(i, n+1) + dt * ( H + nu * lapl + F )

                    let du_dx = (u[(i + 1, n + 1)] - u[(i - 1, n + 1)]) / (2.0 * config.dx);
                    let d2u_dx2 = (u[(i + 1, n + 1)] - 2.0 * u[(i, n + 1)] + u[(i - 1, n + 1)])
                        / (config.dx * config.dx);

                    let hamiltonian = self.hamiltonian.evaluate(du_dx);
                    let running_cost = cost_function(Position(x), Density(m[(i, n + 1)]));

                    u[(i, n)] = u[(i, n + 1)]
                        - config.dt * (hamiltonian - config.viscosity * d2u_dx2 - running_cost);
                }
                // Boundary conditions (Neumann 0)
                u[(0, n)] = u[(1, n)];
                u[(nx - 1, n)] = u[(nx - 2, n)];
            }

            // 2. Solve Fokker-Planck Forward
            for n in 0..nt {
                for i in 1..nx - 1 {
                    // Calculate v at current step n
                    let du_dx = (u[(i + 1, n)] - u[(i - 1, n)]) / (2.0 * config.dx);

                    // The drift velocity v = - H_p(p)
                    let v = -self.hamiltonian.derivative(du_dx);

                    let d2m_dx2 =
                        (m[(i + 1, n)] - 2.0 * m[(i, n)] + m[(i - 1, n)]) / (config.dx * config.dx);

                    // Upwind for drift term
                    let drift_flux = if v > 0.0 {
                        (m[(i, n)] * v - m[(i - 1, n)] * v) / config.dx
                    } else {
                        (m[(i + 1, n)] * v - m[(i, n)] * v) / config.dx
                    };

                    let rhs = -drift_flux + config.viscosity * d2m_dx2;
                    m[(i, n + 1)] = m[(i, n)] + config.dt * rhs;
                }
                // Boundary conditions
                m[(0, n + 1)] = m[(1, n + 1)];
                m[(nx - 1, n + 1)] = m[(nx - 2, n + 1)];

                // Normalize mass to prevent explosion/vanishing
                let sum: f64 = m.column(n + 1).sum();
                if sum > 1e-9 {
                    m.column_mut(n + 1).scale_mut(1.0 / sum);
                }
            }
        }

        (u, m)
    }
}
