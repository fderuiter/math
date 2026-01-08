pub mod types;
pub mod solver;

pub use types::MeanFieldGame1D;
pub use solver::{MFGSolver, FixedPointSolver};

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

    #[test]
    fn test_mfg_solve_with_strategy() {
        let mfg = MeanFieldGame1D::new(
            0.1, 1.0, 50, 100, -2.0, 2.0
        );
        let solver = FixedPointSolver::new(2);

        let cost_fn = |x: f64, m: f64| m + x * x;
        let term_fn = |x: f64, _| x * x;
        let init_dist = |x: f64| (-x * x * 5.0).exp();

        let (u, m) = mfg.solve_with(&solver, cost_fn, term_fn, init_dist);

        assert_eq!(u.nrows(), 50);
        assert_eq!(m.ncols(), 101);
    }
}
