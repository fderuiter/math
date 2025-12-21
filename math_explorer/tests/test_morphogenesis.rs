#[cfg(test)]
mod tests {
    use math_explorer::biology::morphogenesis::{TuringSystem, SchnakenbergKinetics, ReactionKinetics};

    #[test]
    fn test_turing_system_runs() {
        let kinetics = SchnakenbergKinetics::default();
        let mut system = TuringSystem::new(100, 1.0, 0.5, 1.0, kinetics);
        // Step a few times
        for _ in 0..10 {
            system.step(0.1);
        }

        // Just check it doesn't explode immediately (NaN check)
        assert!(!system.u[0].is_nan());
        assert!(!system.v[0].is_nan());
    }

    #[test]
    fn test_schnakenberg_kinetics() {
        let kinetics = SchnakenbergKinetics { a: 0.1, b: 0.2 };
        let u = 0.5;
        let v = 0.5;
        // du/dt = a - u + u^2 v = 0.1 - 0.5 + 0.25*0.5 = 0.1 - 0.5 + 0.125 = -0.275
        // dv/dt = b - u^2 v = 0.2 - 0.125 = 0.075
        let (du, dv) = kinetics.reaction(u, v);
        assert!((du - -0.275).abs() < 1e-6);
        assert!((dv - 0.075).abs() < 1e-6);
    }
}
