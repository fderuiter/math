use math_explorer::biology::diffusion::{SpatialDiffusion, FiniteDifference1D};

#[test]
fn test_diffusion_reproduction() {
    // Setup a small system
    let n = 10;
    let d_u = 0.1;
    let d_v = 0.5;
    let dx = 1.0;

    // Using FiniteDifference1D directly to test the specific diffusion logic
    let diff_strategy = FiniteDifference1D::new(dx);

    let mut u = vec![0.0; n];
    let mut v = vec![0.0; n];

    // Initialize with a simple pattern: impulse in middle
    u[4] = 1.0;
    v[5] = 1.0;

    let mut out_u = vec![0.0; n];
    let mut out_v = vec![0.0; n];

    // Apply diffusion - UPDATED: Now called separately for each component
    diff_strategy.apply(&u, &mut out_u, d_u);
    diff_strategy.apply(&v, &mut out_v, d_v);

    // Expected values calculated by hand/mental model for 3-point stencil:
    // Lap[i] = (u[i+1] - 2u[i] + u[i-1]) / dx^2
    // dx = 1, so / 1

    // For u:
    // i=3: (1 - 0 + 0) = 1.0 * 0.1 = 0.1
    // i=4: (0 - 2 + 0) = -2.0 * 0.1 = -0.2
    // i=5: (0 - 0 + 1) = 1.0 * 0.1 = 0.1

    // For v:
    // i=4: (1 - 0 + 0) = 1.0 * 0.5 = 0.5
    // i=5: (0 - 2 + 0) = -2.0 * 0.5 = -1.0
    // i=6: (0 - 0 + 1) = 1.0 * 0.5 = 0.5

    let tolerance = 1e-10;

    assert!((out_u[3] - 0.1).abs() < tolerance, "u[3] mismatch: {}", out_u[3]);
    assert!((out_u[4] + 0.2).abs() < tolerance, "u[4] mismatch: {}", out_u[4]);
    assert!((out_u[5] - 0.1).abs() < tolerance, "u[5] mismatch: {}", out_u[5]);

    assert!((out_v[4] - 0.5).abs() < tolerance, "v[4] mismatch: {}", out_v[4]);
    assert!((out_v[5] + 1.0).abs() < tolerance, "v[5] mismatch: {}", out_v[5]);
    assert!((out_v[6] - 0.5).abs() < tolerance, "v[6] mismatch: {}", out_v[6]);

    // Boundary conditions (Neumann: derivative is 0 => u[-1] = u[0])
    // If u[0] = 1.0, then u[-1]=1.0, so Lap[0] = (u[1] - 2u[0] + u[0]) = u[1] - u[0]

    // Reset and test boundary
    let mut u_bound = vec![0.0; n];
    u_bound[0] = 1.0;

    diff_strategy.apply(&u_bound, &mut out_u, d_u);

    // i=0: (0 - 2*1 + 1) = -1 * 0.1 = -0.1
    // i=1: (0 - 0 + 1) = 1 * 0.1 = 0.1
    assert!((out_u[0] + 0.1).abs() < tolerance, "u[0] boundary mismatch: {}", out_u[0]);
    assert!((out_u[1] - 0.1).abs() < tolerance, "u[1] boundary mismatch: {}", out_u[1]);
}
