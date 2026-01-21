use math_explorer::biology::morphogenesis::TuringSystem;
use math_explorer::pure_math::analysis::ode::{Euler, RungeKutta4, Solver};

#[test]
fn test_turing_system_euler_equivalence() {
    let size = 100;
    let dt = 0.01;
    let iterations = 10;

    // System 1: Manual Step
    let mut system1 = TuringSystem::new(size, 0.1, 0.05, 1.0);
    // Seed
    system1.u_mut()[50] = 1.0;

    // System 2: Euler Solver
    let mut system2 = TuringSystem::new(size, 0.1, 0.05, 1.0);
    // Seed
    system2.u_mut()[50] = 1.0;

    let solver = Euler;

    for _ in 0..iterations {
        system1.step(dt);

        let mut current_state = system2.state.clone();
        // Note: The OdeSystem trait is implemented for TuringSystem, so we pass &system2
        solver.step(&system2, 0.0, &mut current_state, dt);
        system2.state = current_state;
    }

    // Compare
    for i in 0..size {
        assert!((system1.u()[i] - system2.u()[i]).abs() < 1e-12, "Mismatch at u[{}]: {} vs {}", i, system1.u()[i], system2.u()[i]);
        assert!((system1.v()[i] - system2.v()[i]).abs() < 1e-12, "Mismatch at v[{}]: {} vs {}", i, system1.v()[i], system2.v()[i]);
    }
}

#[test]
fn test_turing_system_rk4() {
    let size = 50;
    let dt = 0.01;
    let iterations = 10;

    let mut system = TuringSystem::new(size, 0.1, 0.05, 1.0);
    system.u_mut()[25] = 1.0;

    let solver = RungeKutta4;
    let mut current_state = system.state.clone();

    for _ in 0..iterations {
        solver.step(&system, 0.0, &mut current_state, dt);
    }

    // Check for finiteness
    for val in current_state.u() {
        assert!(val.is_finite());
    }
    for val in current_state.v() {
        assert!(val.is_finite());
    }

    // Ensure something happened (diffusion started)
    assert!(current_state.u()[25] < 1.0); // Should diffuse away
}
