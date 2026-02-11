use math_explorer::biology::diffusion::{FiniteDifference1D, SpatialDiffusion};

#[test]
fn test_diffusion_apply_step_safe() {
    let n = 10;
    let u = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let v = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let mut out_u = vec![0.0; n];
    let mut out_v = vec![0.0; n];

    let diff = FiniteDifference1D::new(1.0); // dx = 1.0 for simplicity
    let d_u = 0.1;
    let d_v = 0.2;
    let dt = 0.01;

    // Reaction: u grows, v decays
    let reaction = |u: f64, v: f64| (0.1 * u, -0.1 * v);

    diff.apply_step(&u, &v, &mut out_u, &mut out_v, d_u, d_v, dt, reaction);

    // For i=1 (interior):
    // u[0]=1, u[1]=2, u[2]=3. Lap(u) = (3 - 2*2 + 1)/1^2 = 0.
    // v[0]=10, v[1]=9, v[2]=8. Lap(v) = (8 - 2*9 + 10)/1^2 = 0.
    // Reaction u: 0.1 * 2 = 0.2.
    // Reaction v: -0.1 * 9 = -0.9.
    // out_u[1] = 2 + 0.01 * (0.1*0 + 0.2) = 2.002.
    // out_v[1] = 9 + 0.01 * (0.2*0 - 0.9) = 8.991.

    assert!(
        (out_u[1] - 2.002).abs() < 1e-6,
        "out_u[1] mismatch: {}",
        out_u[1]
    );
    assert!(
        (out_v[1] - 8.991).abs() < 1e-6,
        "out_v[1] mismatch: {}",
        out_v[1]
    );

    // Check boundary i=0
    // u[-1]=u[0]=1. u[1]=2. Lap = (2 - 2*1 + 1) = 1.
    // v[-1]=v[0]=10. v[1]=9. Lap = (9 - 2*10 + 10) = -1.
    // Reac u: 0.1. Reac v: -1.0.
    // out_u[0] = 1 + 0.01 * (0.1*1 + 0.1) = 1 + 0.01*0.2 = 1.002.
    // out_v[0] = 10 + 0.01 * (0.2*(-1) - 1.0) = 10 + 0.01*(-1.2) = 9.988.

    assert!(
        (out_u[0] - 1.002).abs() < 1e-6,
        "out_u[0] mismatch: {}",
        out_u[0]
    );
    assert!(
        (out_v[0] - 9.988).abs() < 1e-6,
        "out_v[0] mismatch: {}",
        out_v[0]
    );
}
