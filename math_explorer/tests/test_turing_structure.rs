#[cfg(test)]
mod tests {
    use math_explorer::biology::morphogenesis::{
        SchnakenbergKinetics, StandardSolverAdapter, TuringState, TuringSystem,
    };
    use math_explorer::pure_math::analysis::ode::RungeKutta4;

    #[test]
    fn test_turing_system_generic_step_with_rk4() {
        let size = 100;
        let d_u = 1.0;
        let d_v = 10.0;
        let dx = 1.0;

        // Use generic constructor to inject RK4 solver via Adapter
        // This validates that the system can interoperate with standard solvers
        let dummy_state = TuringState::new(size);
        let rk4 = RungeKutta4::new(&dummy_state);
        let adapter = StandardSolverAdapter::new(rk4);

        let mut system = TuringSystem::new_with_solver(math_explorer::math_kernel::types::Dimension(
            size),
            [math_explorer::math_kernel::types::DiffusionCoeff(d_u), math_explorer::math_kernel::types::DiffusionCoeff(d_v)],
            SchnakenbergKinetics::default(),
            math_explorer::biology::diffusion::FiniteDifference1D::new(math_explorer::math_kernel::types::StepSize(dx)),
            adapter,
        );

        system.state.u_mut()[50] = 1.0;

        // Verify step works with external solver logic wrapped in adapter
        system.step(0.1);

        // Verify state changed
        let val_after = system.state.u()[50];
        assert!(
            val_after < 1.0,
            "Value should have diffused/reacted away from 1.0"
        );
    }
}
