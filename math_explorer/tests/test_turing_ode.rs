use math_explorer::biology::morphogenesis::{TuringSystem, TuringState};
use math_explorer::pure_math::analysis::ode::{Euler, TimeStepper};

#[test]
fn test_turing_state_arithmetic() {
    let mut s1 = TuringState::new(2);
    s1.u_mut()[0] = 1.0;
    s1.u_mut()[1] = 2.0;

    let mut s2 = TuringState::new(2);
    s2.u_mut()[0] = 0.5;
    s2.u_mut()[1] = 0.5;

    // Test Add
    let s3 = s1.clone() + s2.clone();
    assert_eq!(s3.u()[0], 1.5);

    // Test Mul
    let s4 = s1.clone() * 2.0;
    assert_eq!(s4.u()[0], 2.0);
}

#[test]
fn test_turing_euler_compatibility() {
    let size = 10;
    let d_u = 1.0;
    let d_v = 0.5;
    let dx = 1.0;
    let dt = 0.01;

    let mut model_legacy = TuringSystem::new(size, d_u, d_v, dx);
    let mut model_generic = TuringSystem::new(size, d_u, d_v, dx);

    // Initialize with some noise/pattern
    for i in 0..size {
        model_legacy.state.u_mut()[i] = (i as f64) * 0.1;
        model_generic.state.u_mut()[i] = (i as f64) * 0.1;
    }

    // Step Legacy (Optimized Euler)
    model_legacy.step(dt);

    // Step Generic (OdeSystem Euler)
    model_generic.step_with(&Euler, dt);

    // Compare
    // Since floating point ops might be slightly different order (u + dt*(...)) vs (u + ... * dt)
    // we use a small epsilon.
    let epsilon = 1e-10;

    for i in 0..size {
        let u_leg = model_legacy.state.u()[i];
        let u_gen = model_generic.state.u()[i];
        assert!((u_leg - u_gen).abs() < epsilon, "Index {}: Legacy {} != Generic {}", i, u_leg, u_gen);

        let v_leg = model_legacy.state.v()[i];
        let v_gen = model_generic.state.v()[i];
        assert!((v_leg - v_gen).abs() < epsilon, "Index {}: Legacy {} != Generic {}", i, v_leg, v_gen);
    }
}

#[test]
fn test_turing_rk4_runnable() {
    let size = 10;
    let d_u = 1.0;
    let d_v = 0.5;
    let dx = 1.0;
    let dt = 0.01;

    let mut model = TuringSystem::new(size, d_u, d_v, dx);
    // Initialize
    model.state.u_mut()[5] = 1.0;

    // Use RK4 (via TimeStepper trait default)
    // Note: We use the trait method explicitly to verify it works
    <TuringSystem as TimeStepper<TuringState>>::step(&mut model, dt);

    // Check it changed (simple smoke test)
    // u[5] should diffuse and react.
    assert!(model.state.u()[5] != 1.0 || model.state.u()[4] != 0.0);
}
