use nalgebra::DMatrix;

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

    /// Solves the coupled system using a **Fixed-Point Iteration** scheme.
    ///
    /// The algorithm iterates between:
    /// 1. Solving HJB **backward** given the current guess for distribution $m$.
    /// 2. Solving Fokker-Planck **forward** given the optimal control from $u$.
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
        let nx = self.grid_points;
        let nt = self.time_steps;

        // Initialize m (density) and u (value)
        // m[i, n] is density at x_i, t_n
        // u[i, n] is value at x_i, t_n
        // DMatrix stores data in column-major, so we treat rows as space x, cols as time t for convenience?
        // Actually DMatrix is (nrows, ncols). Let's say rows = space points, cols = time steps.
        let mut m = DMatrix::zeros(nx, nt + 1);
        let mut u = DMatrix::zeros(nx, nt + 1);

        // Initialize m at t=0
        for i in 0..nx {
            let x = self.space_min + (i as f64) * self.dx;
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

        for _iter in 0..iterations {
            // 1. Solve HJB Backward
            // Terminal condition
            for i in 0..nx {
                let x = self.space_min + (i as f64) * self.dx;
                u[(i, nt)] = terminal_cost(x, m[(i, nt)]);
            }

            // Backward in time
            for n in (0..nt).rev() {
                for i in 1..nx - 1 {
                    let x = self.space_min + (i as f64) * self.dx;

                    // Finite differences

                    // Central difference for derivatives at t+1 (implicit/explicit mix)
                    // For simplicity, using explicit Euler backward (using values at n+1 to find n)
                    // (u(i, n+1) - u(i, n)) / dt = - ( H + nu * lapl + F )
                    // => u(i, n) = u(i, n+1) + dt * ( H + nu * lapl + F )

                    let du_dx = (u[(i + 1, n + 1)] - u[(i - 1, n + 1)]) / (2.0 * self.dx);
                    let d2u_dx2 = (u[(i + 1, n + 1)] - 2.0 * u[(i, n + 1)] + u[(i - 1, n + 1)]) / (self.dx * self.dx);

                    let hamiltonian = 0.5 * du_dx * du_dx; // H(p) = p^2 / 2
                    let running_cost = cost_function(x, m[(i, n + 1)]);

                    // u(n) = u(n+1) - dt * u_t = u(n+1) - dt * (H - nu * d2u - F)

                    u[(i, n)] = u[(i, n + 1)] - self.dt * (hamiltonian - self.viscosity * d2u_dx2 - running_cost);
                }
                // Boundary conditions (Neumann 0)
                u[(0, n)] = u[(1, n)];
                u[(nx - 1, n)] = u[(nx - 2, n)];
            }

            // 2. Solve Fokker-Planck Forward
            // m(0) is fixed.
            for n in 0..nt {
                for i in 1..nx - 1 {
                    // dm/dt + div(m * v) - nu * Delta m = 0
                    // v = -dH/dp = -p = -grad u. (If minimizing cost)
                    // Actually if H(p) = p^2/2, optimal control alpha = -p = -grad u.
                    // Drift is v = -grad u.

                    // Explicit Euler forward
                    // (m(i, n+1) - m(i, n)) / dt = - div(m * v) + nu * Delta m

                    // Calculate v at current step n
                    let du_dx = (u[(i + 1, n)] - u[(i - 1, n)]) / (2.0 * self.dx);
                    let v = -du_dx;

                    // Upwind scheme for div(m * v) = d/dx (m * v)
                    // If v > 0, use backward diff for m. If v < 0, use forward diff.
                    // simpler: m * dv/dx + v * dm/dx

                    let d2m_dx2 = (m[(i + 1, n)] - 2.0 * m[(i, n)] + m[(i - 1, n)]) / (self.dx * self.dx);

                    // Upwind for drift term
                    let drift_flux = if v > 0.0 {
                         (m[(i, n)] * v - m[(i - 1, n)] * v) / self.dx
                    } else {
                         (m[(i + 1, n)] * v - m[(i, n)] * v) / self.dx
                    };

                    let rhs = -drift_flux + self.viscosity * d2m_dx2;
                    m[(i, n + 1)] = m[(i, n)] + self.dt * rhs;
                }
                // Boundary conditions (Neumann 0 for m to conserve mass roughly, or simpler 0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mfg_run() {
        let mfg = MeanFieldGame1D::new(
            0.1,  // viscosity
            1.0,  // T
            50,   // Grid points
            100,  // Time steps
            -2.0, // min
            2.0   // max
        );

        // F(x, m) = cost of being in crowd + potential
        // penalize high density
        let cost_fn = |x: f64, m: f64| -> f64 {
             m + x * x // simple quadratic potential + crowding cost
        };

        // G(x, m) = terminal cost
        let term_fn = |x: f64, _m: f64| -> f64 {
            x * x
        };

        // Initial bump at 0
        let init_dist = |x: f64| -> f64 {
            (-x * x * 5.0).exp()
        };

        let (u, m) = mfg.solve(cost_fn, term_fn, init_dist, 5);

        // Basic checks
        assert_eq!(u.nrows(), 50);
        assert_eq!(u.ncols(), 101);
        assert_eq!(m.nrows(), 50);

        // Check conservation of mass roughly?
        // sum(m) * dx should be constant-ish
        let sum_initial: f64 = m.column(0).sum();
        let sum_final: f64 = m.column(100).sum();

        // It won't be perfect due to simple boundaries and explicit scheme, but shouldn't explode
        assert!((sum_initial - sum_final).abs() < 5.0); // very loose bound just to check stability
    }
}
