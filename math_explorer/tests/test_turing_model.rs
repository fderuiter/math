use math_explorer::biology::morphogenesis::{TuringModel, TuringState, SchnakenbergKinetics};
use math_explorer::biology::diffusion::FiniteDifference1D;
use math_explorer::pure_math::analysis::ode::solvers::Euler;
use math_explorer::pure_math::analysis::ode::traits::Solver;

#[test]
fn test_turing_model_independent_simulation() {
    // 1. Setup the Physics Model (no simulation state/buffers yet)
    let n = 20;
    let dx = 1.0;
    let kinetics = SchnakenbergKinetics::default();
    let diffusion = FiniteDifference1D::new(dx);

    let model = TuringModel {
        d_u: 1.0,
        d_v: 0.5,
        kinetics,
        diffusion,
    };

    // 2. Setup the State independently
    let mut state = TuringState::new(n);
    // Initialize with noise
    for i in 0..n {
        state.u_mut()[i] = 1.0 + (i as f64 * 0.1).sin();
        state.v_mut()[i] = 0.5 + (i as f64 * 0.1).cos();
    }

    // 3. Use a Generic Solver (Euler) to drive the simulation
    // This proves that TuringModel implements OdeSystem correctly and can be decoupled from TuringSystem/FusedTuringSolver.
    let mut solver = Euler::new(&state);
    let dt = 0.01;

    // Run 10 steps
    for _ in 0..10 {
        state = solver.solve(&model, 0.0, &state, dt);
    }

    // 4. Verify output is not NaN and has changed
    assert!(!state.u()[0].is_nan());
    assert!(!state.v()[0].is_nan());

    // Check that reaction/diffusion happened (values changed)
    let initial_u_0 = 1.0 + (0.0_f64).sin(); // 1.0
    assert!((state.u()[0] - initial_u_0).abs() > 1e-6, "State should have evolved");
}
