use math_explorer::biology::morphogenesis::TuringSystem;
use math_explorer::pure_math::analysis::ode::{RungeKutta4, TimeStepper};

#[test]
fn test_turing_step_with_rk4() {
    let size = 100;
    // Small dt for stability
    let dt = 0.01;
    let mut system_euler = TuringSystem::new(size, 0.1, 0.05, 1.0);
    let mut system_rk4 = TuringSystem::new(size, 0.1, 0.05, 1.0);

    // Seed both identically
    for i in 0..size {
        if i % 10 == 0 {
            system_euler.u_mut()[i] = 1.0;
            system_rk4.u_mut()[i] = 1.0;
        }
    }

    // Step Euler (optimized)
    system_euler.step(dt);

    // Step RK4
    system_rk4.step_with(&RungeKutta4, dt);

    // They should differ slightly because Euler is O(dt) and RK4 is O(dt^4),
    // but they should be close.
    let diff_u = system_euler
        .u()
        .iter()
        .zip(system_rk4.u().iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f64>();

    println!("Diff U: {}", diff_u);
    assert!(diff_u > 0.0, "RK4 should differ from Euler");
    assert!(diff_u < 1.0, "RK4 should stay reasonably close to Euler for small dt");
}
