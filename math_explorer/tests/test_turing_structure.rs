#[cfg(test)]
mod tests {
    use math_explorer::biology::diffusion::FiniteDifference1D;
    use math_explorer::biology::morphogenesis::{SchnakenbergKinetics, TuringSystem};
    use math_explorer::biology::reaction_diffusion::ChemicalState;
    use math_explorer::pure_math::analysis::ode::{RungeKutta4, Solver};

    #[test]
    fn test_turing_system_generic_step() {
        let size = 100;
        let d_u = 1.0;
        let d_v = 10.0;
        let dx = 1.0;

        let kinetics = SchnakenbergKinetics::default();
        let diffusion = FiniteDifference1D::new(dx);

        // Construct a solver with a dummy state
        let dummy_state = ChemicalState::new(2, size);
        let solver = RungeKutta4::new(&dummy_state);

        // Use new_with_solver to inject RK4
        let mut system = TuringSystem::new_with_solver(size, d_u, d_v, kinetics, diffusion, solver);

        system.u_mut()[50] = 1.0;

        // Verify we can call step (uses injected RK4 solver)
        system.step(0.1);

        // Verify state changed
        let val_after = system.u()[50];
        assert!(
            val_after < 1.0,
            "Value should have diffused/reacted away from 1.0"
        );
    }
}
