//! The `chaos` module explores Deterministic Chaos, covering continuous flows, discrete maps,
//! and methods to quantify chaos (Lyapunov exponents and Fractal Dimensions).
//!
//! Chaos theory studies the behavior of dynamical systems that are highly sensitive to initial conditions.
//! This sensitivity, often referred to as the butterfly effect, implies that small differences in initial
//! states yield widely diverging outcomes for such dynamical systems, rendering long-term prediction impossible
//! in general.

pub mod logistic;
pub mod lorenz;
pub mod metrics;
pub mod fractals;

// Re-export specific items to maintain backward compatibility if needed,
// but the plan says "Extract `logistic` module to `math_explorer/src/physics/chaos/logistic.rs`".
// The original `chaos.rs` had `pub mod logistic`, `pub mod lorenz`, `pub mod metrics`, `pub mod fractals`.
// So we just need to declare them as pub mod.

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra as na;
    use approx::assert_relative_eq;

    #[test]
    fn test_logistic_chaos_lyapunov() {
        // For r=3.9, the map is chaotic, so Lyapunov exponent should be positive.
        // Theoretical value is approx 0.496.
        // We use x0=0.1 to avoid starting at the critical point x=0.5.
        let lambda = metrics::logistic_lyapunov(3.9, 0.1, 1000);
        assert!(lambda > 0.0, "Lyapunov exponent for r=3.9 should be positive, got {}", lambda);
    }

    #[test]
    fn test_logistic_stability_lyapunov() {
        // For r=2.5, the map is stable (fixed point at 0.6), so Lyapunov exponent should be negative.
        // f'(x*) = 2.5(1 - 1.2) = 2.5(-0.2) = -0.5. ln(0.5) approx -0.693.
        // We use x0=0.1.
        let lambda = metrics::logistic_lyapunov(2.5, 0.1, 1000);
        assert!(lambda < 0.0, "Lyapunov exponent for r=2.5 should be negative, got {}", lambda);
        // We can be more precise
        assert!((lambda - -std::f64::consts::LN_2).abs() < 0.1, "Expected approx -0.693, got {}", lambda);
    }

    #[test]
    fn test_lorenz_boundedness() {
        // Run Lorenz system and ensure it stays within reasonable bounds.
        // The Lorenz attractor is contained within a specific region of space.
        let state = lorenz::LorenzState::new(1.0, 1.0, 1.0);
        let mut system = lorenz::LorenzSystem::default_chaotic(state);

        let dt = 0.01;
        for _ in 0..1000 {
            system.step(dt);
            let s = system.state.vec;
            assert!(s.x.abs() < 100.0, "x diverged: {}", s.x);
            assert!(s.y.abs() < 100.0, "y diverged: {}", s.y);
            assert!(s.z.abs() < 100.0, "z diverged: {}", s.z);
        }
    }

    #[test]
    fn test_bifurcation_diagram_generation() {
        let points = logistic::generate_bifurcation_diagram(3.0, 4.0, 10);
        // steps=10 means 11 values of r. Each has 50 points. Total 550 points.
        assert_eq!(points.len(), 11 * 50);
        // Check bounds
        for (r, x) in points {
            assert!(r >= 3.0 && r <= 4.0);
            assert!(x >= 0.0 && x <= 1.0);
        }
    }

    #[test]
    fn test_lorenz_lyapunov_strategy() {
        use crate::pure_math::analysis::ode::{RungeKutta4, Euler};
        let state = lorenz::LorenzState::new(1.0, 1.0, 1.0);
        let system = lorenz::LorenzSystem::default_chaotic(state);

        // Test with RK4
        let lambda_rk4 = metrics::lorenz_lyapunov(
            &system,
            &RungeKutta4,
            na::Vector3::new(10.0, 10.0, 10.0),
            0.01,
            100,
            1.0
        ).unwrap();

        assert!(lambda_rk4 > 0.0, "Lorenz with RK4 should be chaotic, got {}", lambda_rk4);

        // Test with Euler (less accurate, but should run)
        let lambda_euler = metrics::lorenz_lyapunov(
            &system,
            &Euler,
            na::Vector3::new(10.0, 10.0, 10.0),
            0.0001, // Euler needs smaller step for stability
            100,
            1.0
        ).unwrap();

        assert!(lambda_euler > 0.0, "Lorenz with Euler should be chaotic, got {}", lambda_euler);
    }

    #[test]
    fn test_correlation_dimension_simple() {
        // Create a line of points: (0,0,0), (1,0,0), (2,0,0) ...
        // Dimension should be 1. But we just test the C(epsilon) calculation here.
        let mut traj = Vec::new();
        for i in 0..10 {
            traj.push(na::Vector3::new(i as f64, 0.0, 0.0));
        }

        // Epsilon = 1.1. Pairs with dist < 1.1 are adjacent points.
        // Pairs (0,1), (1,2), ..., (8,9). There are 9 such pairs.
        // Total pairs N(N-1) = 10*9 = 90.
        // Count = 9 * 2 = 18. (symmetric)
        // C = 18 / 90 = 0.2.

        let c = fractals::correlation_dimension(&traj, 1.1);
        assert_relative_eq!(c, 0.2);
    }

    #[test]
    fn test_lorenz_builder_custom_parameters() {
        let state = lorenz::LorenzState::new(1.0, 1.0, 1.0);

        // Build a system with non-standard parameters
        // Example: sigma = 11.0, rho = 29.0 (slightly different from standard)
        let system = lorenz::LorenzBuilder::new()
            .sigma(11.0)
            .rho(29.0)
            .beta(3.0)
            .build(state);

        assert_eq!(system.sigma, 11.0);
        assert_eq!(system.rho, 29.0);
        assert_eq!(system.beta, 3.0);

        // Ensure it runs
        let mut running_system = system;
        running_system.step(0.01);
        assert!(running_system.state.vec.x.is_finite());
    }
}
