use super::error::OdeError;
use super::solvers::RungeKutta4;
use super::traits::{OdeSystem, Solver, VectorOperations};

/// A trait for systems that can advance in time.
///
/// This trait abstracts over the `step` and `step_with` methods,
/// reducing boilerplate in model implementations.
pub trait TimeStepper<State: VectorOperations>: OdeSystem<State> {
    /// Returns a reference to the current state.
    fn get_state(&self) -> &State;

    /// Returns a mutable reference to the current state.
    fn get_state_mut(&mut self) -> &mut State;

    /// Advances the system by `dt` using the default Runge-Kutta 4 solver.
    fn step(&mut self, dt: f64) -> Result<(), OdeError> {
        // We assume the system is autonomous (time-invariant derivative) or
        // tracks time internally if needed, hence passing 0.0 as time.
        // For strictly time-dependent systems, `step` might need to track `t`.
        // However, existing models pass 0.0, so we preserve that behavior.
        let current_state = self.get_state().clone();
        // Use generic parameter State to call the static method on the generic struct
        let new_state = RungeKutta4::<State>::step(self, 0.0, &current_state, dt)?;
        *self.get_state_mut() = new_state;
        Ok(())
    }

    /// Advances the system by `dt` using a provided solver.
    fn step_with<S: Solver<State>>(&mut self, solver: &mut S, dt: f64) -> Result<(), OdeError> {
        let current_state = self.get_state().clone();
        let new_state = solver.solve(self, 0.0, &current_state, dt)?;
        *self.get_state_mut() = new_state;
        Ok(())
    }
}
