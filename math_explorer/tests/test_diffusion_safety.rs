use math_explorer::biology::diffusion::{FiniteDifference1D, SpatialDiffusion};

#[test]
fn test_finite_difference_apply_step_correctness() {
    let n = 10;
    let dx = 0.1;
    let diff = FiniteDifference1D::new(dx);

    // Initial conditions: quadratic pulse
    // u(x) = x * (1 - x) on [0, 1]
    let u: Vec<f64> = (0..n).map(|i| {
        let x = i as f64 * dx;
        x * (1.0 - x)
    }).collect();
    let v: Vec<f64> = vec![0.0; n];

    let mut out_u = vec![0.0; n];
    let mut out_v = vec![0.0; n];

    let d_u = 0.01;
    let d_v = 0.0;
    let dt = 0.01;

    // Simple reaction: u -> u/2, v -> 0
    let reaction = |u_val: f64, _v_val: f64| (-u_val * 0.5, 0.0);

    // Run one step
    diff.apply_step(&u, &v, &mut out_u, &mut out_v, d_u, d_v, dt, reaction);

    // Verify interior points
    // Expected Laplacian for x(1-x) = -x^2 + x is -2
    // d2u/dx2 = -2
    // But discrete laplacian is slightly different near boundaries due to Neumann BC

    // Check index 5 (middle)
    let idx = 5;
    let x = idx as f64 * dx;
    let expected_u = x * (1.0 - x); // current u
    let expected_lap = (u[idx+1] - 2.0 * u[idx] + u[idx-1]) / (dx * dx);
    let expected_reac = -expected_u * 0.5;

    let expected_next_u = expected_u + dt * (d_u * expected_lap + expected_reac);

    assert!((out_u[idx] - expected_next_u).abs() < 1e-10, "Interior point calculation mismatch");

    // Verify boundary (idx 0)
    // Neumann BC: u[-1] = u[0] -> Laplacian = (u[1] - 2u[0] + u[0]) / dx^2 = (u[1] - u[0]) / dx^2
    let idx = 0;
    let expected_u = u[0];
    let expected_lap = (u[1] - u[0]) / (dx * dx); // simplified due to u[-1]=u[0]
    let expected_reac = -expected_u * 0.5;
    let expected_next_u = expected_u + dt * (d_u * expected_lap + expected_reac);

    assert!((out_u[idx] - expected_next_u).abs() < 1e-10, "Left boundary calculation mismatch");

    // Verify boundary (idx n-1)
    // Neumann BC: u[n] = u[n-1] -> Laplacian = (u[n-1] - 2u[n-1] + u[n-2]) / dx^2 = (u[n-2] - u[n-1]) / dx^2
    let idx = n - 1;
    let expected_u = u[idx];
    let expected_lap = (u[idx-1] - u[idx]) / (dx * dx);
    let expected_reac = -expected_u * 0.5;
    let expected_next_u = expected_u + dt * (d_u * expected_lap + expected_reac);

    assert!((out_u[idx] - expected_next_u).abs() < 1e-10, "Right boundary calculation mismatch");
}

#[test]
fn test_finite_difference_small_array() {
    let dx = 0.1;
    let diff = FiniteDifference1D::new(dx);
    let d_u = 0.1;
    let d_v = 0.1;
    let dt = 0.01;
    let reaction = |_u: f64, _v: f64| (0.0, 0.0);

    // Case n=1
    let u = vec![1.0];
    let v = vec![1.0];
    let mut out_u = vec![0.0];
    let mut out_v = vec![0.0];

    diff.apply_step(&u, &v, &mut out_u, &mut out_v, d_u, d_v, dt, reaction);
    // Laplacian is 0 for n=1 with Neumann BC?
    // Code says: if n > 1 { (u[1], v[1]) } else { (u_curr, v_curr) }
    // So u_next = u_curr. u_prev = u_curr.
    // lap = (u_curr - 2u_curr + u_curr) = 0. Correct.
    assert_eq!(out_u[0], 1.0);

    // Case n=2
    let u = vec![1.0, 2.0];
    let v = vec![1.0, 2.0];
    let mut out_u = vec![0.0; 2];
    let mut out_v = vec![0.0; 2];

    diff.apply_step(&u, &v, &mut out_u, &mut out_v, d_u, d_v, dt, reaction);

    // Index 0: u_next=2, u_curr=1, u_prev=1. lap = (2 - 2 + 1)/dx^2 = 1/dx^2 = 100.
    // out_u[0] = 1 + 0.01 * (0.1 * 100 + 0) = 1 + 0.1 = 1.1
    let expected_0 = 1.0 + dt * (d_u * (1.0 / (dx*dx)));
    assert!((out_u[0] - expected_0).abs() < 1e-10);

    // Index 1: u_next=2 (BC), u_curr=2, u_prev=1. lap = (2 - 4 + 1)/dx^2 = -1/dx^2 = -100.
    // out_u[1] = 2 + 0.01 * (0.1 * -100) = 2 - 0.1 = 1.9
    let expected_1 = 2.0 + dt * (d_u * (-1.0 / (dx*dx)));
    assert!((out_u[1] - expected_1).abs() < 1e-10);
}
