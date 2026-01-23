use math_explorer::biology::morphogenesis::TuringSystem;
use math_explorer::pure_math::analysis::ode::{Solver, RungeKutta4, Euler};

#[test]
fn test_turing_ode_integration() {
    let size = 20;
    // Small dt to compare Euler vs RK4
    let dt = 0.001;
    let d_u = 0.1;
    let d_v = 0.05;
    let dx = 1.0;

    let mut system_ode = TuringSystem::new(size, d_u, d_v, dx);
    let mut system_legacy = TuringSystem::new(size, d_u, d_v, dx);

    // Initialize with same seed
    for i in 0..size {
        if i % 5 == 0 {
            system_ode.state.u_mut()[i] = 1.0;
            system_legacy.state.u_mut()[i] = 1.0;
        }
    }

    let iterations = 10;
    let rk4 = RungeKutta4;

    for _ in 0..iterations {
        // Step with RK4
        let new_state = rk4.solve(&system_ode, 0.0, &system_ode.state, dt);
        system_ode.state = new_state;

        // Step with Legacy (optimized Euler)
        system_legacy.step(dt);
    }

    // Compare
    // They won't be identical because RK4 is more accurate.
    // But they should be "close enough" for this test, or at least finite and non-zero.

    let u_ode = system_ode.u();
    let u_leg = system_legacy.u();

    for i in 0..size {
        // Just ensure we didn't crash and numbers are changing
        assert!(u_ode[i].is_finite());
        assert!(u_leg[i].is_finite());

        // Difference check (generous tolerance due to method difference)
        let diff = (u_ode[i] - u_leg[i]).abs();
        assert!(diff < 0.01, "Divergence at index {}: ODE={}, Legacy={}, Diff={}", i, u_ode[i], u_leg[i], diff);
    }
}

#[test]
fn test_turing_euler_equivalence() {
    // Verify that the OdeSystem implementation produces the same derivative as the manual step implies.
    // If we use Euler solver with OdeSystem, it should match the manual step exactly (floating point allowing).

    let size = 20;
    let dt = 0.01;
    let d_u = 0.1;
    let d_v = 0.05;
    let dx = 1.0;

    let mut system_ode = TuringSystem::new(size, d_u, d_v, dx);
    let mut system_legacy = TuringSystem::new(size, d_u, d_v, dx);

    // Initialize with same seed
    for i in 0..size {
        if i % 5 == 0 {
            system_ode.state.u_mut()[i] = 1.0;
            system_legacy.state.u_mut()[i] = 1.0;
        }
    }

    let euler = Euler;

    // One step
    // Legacy
    system_legacy.step(dt);

    // ODE Euler
    let new_state = euler.solve(&system_ode, 0.0, &system_ode.state, dt);

    // Compare
    let u_ode = new_state.u();
    let u_leg = system_legacy.u();

    for i in 0..size {
        // Should be extremely close, possibly identical bit-wise if order of operations matches
        // But `scale_add` logic `a += b * scale` vs `u + dt * (...)` might differ slightly in associativity.
        let diff = (u_ode[i] - u_leg[i]).abs();
        assert!(diff < 1e-12, "Euler Equivalence failed at {}: ODE={}, Legacy={}, Diff={}", i, u_ode[i], u_leg[i], diff);
    }
}
