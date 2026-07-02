use super::traits::{OdeSystem, Solver, SolverExt, VectorOperations};

/// A trait for systems that can advance in time.
///
/// This trait abstracts over the `step` and `step_with` methods,
/// enforcing a consistent interface for time-stepping simulations.
pub trait TimeStepper<State: VectorOperations>: OdeSystem<State> {
    /// Returns a reference to the current state.
    #[verified_engine::verified]
    fn get_state(&self) -> &State;

    /// Returns a mutable reference to the current state.
    #[verified_engine::verified]
    fn get_state_mut(&mut self) -> &mut State;

    /// Advances the system by `dt` using a default strategy.
    ///
    /// The default implementation uses Runge-Kutta 4 (RK4).
    #[verified_engine::verified]
    fn step(&mut self, dt: f64) {
        use super::solvers::RungeKutta4;
        use super::traits::SolverExt;
        let mut solver = RungeKutta4::new(self.get_state());
        let new_state = solver.solve(self, 0.0, self.get_state(), dt);
        *self.get_state_mut() = new_state;
    }

    /// Advances the system by `dt` using a provided solver.
    fn step_with<S: Solver<State>>(&mut self, solver: &mut S, dt: f64) {
        let current_state = self.get_state().clone();
        let new_state = solver.solve(self, 0.0, &current_state, dt);
        *self.get_state_mut() = new_state;
    }
}
