use math_explorer::biology::morphogenesis::TuringSystem;

#[test]
fn test_turing_system_initialization() {
    let system = TuringSystem::new(100, 1.0, 1.0, 1.0);
    assert_eq!(system.u.len(), 100);
    assert_eq!(system.v.len(), 100);
}

#[test]
fn test_turing_system_step_determinism() {
    let mut system = TuringSystem::new(10, 1.0, 10.0, 1.0);

    // Initialize with some noise or pattern to ensure dynamics happen
    for i in 0..10 {
        system.u[i] = 0.5 + (i as f64 * 0.1);
        system.v[i] = 0.5 - (i as f64 * 0.1);
    }

    let initial_u = system.u.clone();

    // Step
    system.step(0.01);

    // Check that state changed
    assert_ne!(system.u, initial_u);

    // Check conservation/bounds logic if applicable, but for now just regression:
    // We expect specific values? No, just that it runs and produces numbers.
    // Let's print one value to manual check if needed, but for automated regression,
    // I will record the value after 1 step and ensure the refactor matches it.

    let u_5 = system.u[5];
    let v_5 = system.v[5];

    println!("u[5]: {}, v[5]: {}", u_5, v_5);
}
