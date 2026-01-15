use math_explorer::biology::morphogenesis::{ReactionKinetics, TuringSystem};

struct GrayScottKinetics {
    f: f64,
    k: f64,
}

impl ReactionKinetics for GrayScottKinetics {
    fn reaction(&self, u: f64, v: f64) -> (f64, f64) {
        // Gray-Scott model:
        // u_t = -uv^2 + f(1-u)
        // v_t = uv^2 - (f+k)v
        let uv_sq = u * v.powi(2);
        let reaction_u = -uv_sq + self.f * (1.0 - u);
        let reaction_v = uv_sq - (self.f + self.k) * v;
        (reaction_u, reaction_v)
    }
}

#[test]
fn test_turing_regression() {
    let size = 100;
    let iterations = 100;
    // Default uses Schnakenberg kinetics (backward compatible)
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

#[test]
fn test_custom_kinetics_strategy() {
    let size = 50;
    let iterations = 10;

    // Use Gray-Scott kinetics via the Strategy Pattern
    let kinetics = GrayScottKinetics { f: 0.055, k: 0.062 };
    let mut system = TuringSystem::new_with_kinetics(size, 0.2, 0.1, 1.0, kinetics);

    // Seed
    system.u[25] = 0.5;
    system.v[25] = 0.25;

    // Run simulation
    for _ in 0..iterations {
        system.step(0.1);
    }

    // Check that values changed (diffusion + reaction happened)
    // We don't check exact values as this is a new model, but we ensure it didn't panic and produced finite numbers
    assert!(system.u[25] != 0.5);
    assert!(system.u[25].is_finite());
    assert!(system.v[25].is_finite());
}
