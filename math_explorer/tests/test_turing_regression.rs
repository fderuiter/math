use math_explorer::biology::morphogenesis::TuringSystem;

#[test]
fn test_turing_regression() {
    let size = 100;
    let iterations = 100;
    let mut system = TuringSystem::new(size, 0.1, 0.05, 1.0);

    // Seed it deterministically
    for i in 0..size {
        if i % 10 == 0 {
            system.u[i] = 1.0;
        }
    }

    for _ in 0..iterations {
        system.step(0.01);
    }

    // Verify exactness
    assert!((system.u[50] - 0.3116871030).abs() < 1e-9);
    assert!((system.v[50] - 0.0447023450).abs() < 1e-9);
}
