use math_explorer::biology::diffusion::{FiniteDifference1D, SpatialDiffusion};
use math_explorer::biology::morphogenesis::ReactionKinetics;
use math_explorer::biology::morphogenesis::{
    SchnakenbergKinetics, StandardSolverAdapter, TuringSolverStrategy, TuringState, TuringSystem,
};
use math_explorer::pure_math::analysis::ode::{Euler, RungeKutta4};

#[test]
fn test_turing_adapter() {
    let n = 20;
    let d_u = 1.0;
    let d_v = 40.0;
    let dx = 1.0;
    let dt = 0.01;
    let steps = 10;

    // 1. Baseline: FusedEulerSolver
    let mut fused_system = TuringSystem::new(n, d_u, d_v, dx);
    initialize_state(&mut fused_system);

    for _ in 0..steps {
        fused_system.step(dt);
    }
    let fused_u = fused_system.u()[n / 2];

    // 2. Adapter with Euler
    // Euler should be mathematically identical to FusedEulerSolver
    let example_state = TuringState::new(n);
    let euler = Euler::new(&example_state);
    let adapter = StandardSolverAdapter(euler);

    let kinetics = SchnakenbergKinetics::default();
    let diffusion = FiniteDifference1D::new(dx);

    let mut euler_system = TuringSystem::new_with_solver(n, d_u, d_v, kinetics, diffusion, adapter);
    initialize_state(&mut euler_system);

    for _ in 0..steps {
        euler_system.step(dt);
    }
    let euler_u = euler_system.u()[n / 2];

    assert!(
        (fused_u - euler_u).abs() < 1e-10,
        "Euler result differs from Fused result!"
    );

    // 3. Adapter with RK4
    let rk4 = RungeKutta4::new(&example_state);
    let adapter_rk4 = StandardSolverAdapter(rk4);

    let kinetics = SchnakenbergKinetics::default();
    let diffusion = FiniteDifference1D::new(dx);

    let mut rk4_system =
        TuringSystem::new_with_solver(n, d_u, d_v, kinetics, diffusion, adapter_rk4);
    initialize_state(&mut rk4_system);

    for _ in 0..steps {
        rk4_system.step(dt);
    }
    let rk4_u = rk4_system.u()[n / 2];

    // RK4 is more accurate, so it might differ slightly, but should be close for small dt
    assert!(
        (rk4_u - fused_u).abs() < 1e-1,
        "RK4 result diverges wildly!"
    );
}

fn initialize_state<K, D, S>(system: &mut TuringSystem<K, D, S>)
where
    K: ReactionKinetics,
    D: SpatialDiffusion<2>,
    S: TuringSolverStrategy,
{
    let n = system.state.len();
    for i in 0..n {
        system.u_mut()[i] = 1.0 + (i as f64 * 0.1).sin();
        system.v_mut()[i] = 0.5 + (i as f64 * 0.1).cos();
    }
}
