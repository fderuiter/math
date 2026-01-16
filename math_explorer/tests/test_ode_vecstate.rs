use math_explorer::pure_math::analysis::ode::{
    BufferedRungeKutta4, OdeSystem, RungeKutta4, Solver, VecState,
};

/// A simple decay model: dy/dt = -y
struct DecayModel;

impl OdeSystem<VecState> for DecayModel {
    fn derivative(&self, _t: f64, state: &VecState) -> VecState {
        // dy/dt = -y
        let derivatives: Vec<f64> = state.0.iter().map(|&y| -y).collect();
        VecState(derivatives)
    }

    fn derivative_in_place(&self, _t: f64, state: &VecState, out: &mut VecState) {
        if out.0.len() != state.0.len() {
            out.0.resize(state.0.len(), 0.0);
        }
        for (i, &y) in state.0.iter().enumerate() {
            out.0[i] = -y;
        }
    }
}

#[test]
fn test_vec_state_ode() {
    let system = DecayModel;
    let initial_state = VecState(vec![10.0, 5.0]); // Two independent decay processes
    let dt = 0.01;
    let total_time = 1.0;
    let steps = (total_time / dt) as usize;

    let mut state = initial_state.clone();
    let mut t = 0.0;

    let mut solver = RungeKutta4;

    for _ in 0..steps {
        state = solver.solve(&system, t, &state, dt);
        t += dt;
    }

    // Analytical solution: y(t) = y(0) * e^(-t)
    let expected_factor = (-1.0f64).exp(); // e^-1 at t=1

    let y1 = state.0[0];
    let y2 = state.0[1];

    let expected_y1 = 10.0 * expected_factor;
    let expected_y2 = 5.0 * expected_factor;

    // Check with tolerance
    assert!(
        (y1 - expected_y1).abs() < 1e-4,
        "y1: expected {}, got {}",
        expected_y1,
        y1
    );
    assert!(
        (y2 - expected_y2).abs() < 1e-4,
        "y2: expected {}, got {}",
        expected_y2,
        y2
    );
}

#[test]
fn test_buffered_rk4() {
    let system = DecayModel;
    let initial_state = VecState(vec![10.0, 5.0]);
    let dt = 0.01;
    let total_time = 1.0;
    let steps = (total_time / dt) as usize;

    let mut state = initial_state.clone();
    let mut t = 0.0;

    let mut solver = BufferedRungeKutta4::new(&initial_state);

    for _ in 0..steps {
        state = solver.solve(&system, t, &state, dt);
        t += dt;
    }

    let expected_factor = (-1.0f64).exp();
    let y1 = state.0[0];
    let expected_y1 = 10.0 * expected_factor;

    assert!(
        (y1 - expected_y1).abs() < 1e-4,
        "Buffered RK4 y1: expected {}, got {}",
        expected_y1,
        y1
    );
}
