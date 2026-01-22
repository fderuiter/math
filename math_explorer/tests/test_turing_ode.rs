use math_explorer::biology::morphogenesis::TuringSystem;
use math_explorer::pure_math::analysis::ode::{Euler, Solver};

#[test]
fn test_turing_ode_equivalence() {
    let size = 100;
    let dt = 0.01;
    let iterations = 10;

    // Create two identical systems
    let mut system_manual = TuringSystem::new(size, 0.1, 0.05, 1.0);
    let mut system_ode = TuringSystem::new(size, 0.1, 0.05, 1.0);

    // Seed them identically
    for i in 0..size {
        if i % 10 == 0 {
            system_manual.u_mut()[i] = 1.0;
            system_ode.u_mut()[i] = 1.0;
        }
    }

    let solver = Euler;

    for _ in 0..iterations {
        // Step manual
        system_manual.step(dt);

        // Step ODE (Euler)
        // We act on system_ode.state directly.
        // Since TuringSystem now implements OdeSystem<TuringState>, we can pass &system_ode as the system.
        let mut current_state = system_ode.state.clone();
        solver.step(&system_ode, 0.0, &mut current_state, dt);
        system_ode.state = current_state;
    }

    // Compare results
    let u_manual = system_manual.u();
    let u_ode = system_ode.u();
    let v_manual = system_manual.v();
    let v_ode = system_ode.v();

    for i in 0..size {
        // Use a small epsilon because order of operations might slightly differ
        // (Manual: u + dt*(...) vs Ode: clone + derivative*dt)
        let tolerance = 1e-12;
        assert!((u_manual[i] - u_ode[i]).abs() < tolerance, "Mismatch at U[{}] (Diff: {:e})", i, (u_manual[i] - u_ode[i]).abs());
        assert!((v_manual[i] - v_ode[i]).abs() < tolerance, "Mismatch at V[{}] (Diff: {:e})", i, (v_manual[i] - v_ode[i]).abs());
    }
}
