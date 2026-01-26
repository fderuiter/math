#[cfg(test)]
mod tests {
    use math_explorer::biology::morphogenesis::{TuringSystem, TuringState};
    use math_explorer::pure_math::analysis::ode::{RungeKutta4, Solver, TimeStepper};

    #[test]
    fn test_turing_system_generic_step() {
        let size = 100;
        let d_u = 1.0;
        let d_v = 10.0;
        let dx = 1.0;
        let mut system = TuringSystem::new(size, d_u, d_v, dx);

        system.state.u_mut()[50] = 1.0;

        // Verify we can call step_with (requires TimeStepper trait)
        system.step_with(&RungeKutta4, 0.1);

        // Verify state changed
        let val_after = system.state.u()[50];
        assert!(val_after < 1.0, "Value should have diffused/reacted away from 1.0");
    }
}
