use super::stepper::TimeStepper;
use super::traits::{OdeSystem, Solver, VectorOperations};

/// A generic struct representing an ODE-based model.
///
/// This struct composes the system state, the system dynamics (OdeSystem),
/// and the numerical solver strategy. It implements `TimeStepper` to allow
/// easy simulation.
///
/// # Generics
/// * `State`: The state vector type (must implement `VectorOperations`).
/// * `Dyn`: The dynamics defining the derivative (must implement `OdeSystem<State>`).
/// * `S`: The numerical solver strategy (must implement `Solver<State>`).
#[derive(Debug, Clone)]
pub struct OdeModel<State, Dyn, S> {
    /// The current state of the system.
    pub state: State,
    /// The underlying dynamics model (parameters + equations).
    pub dynamics: Dyn,
    /// The numerical solver strategy.
    pub solver: S,
}

impl<State, Dyn, S> OdeModel<State, Dyn, S>
where
    State: VectorOperations,
    Dyn: OdeSystem<State>,
    S: Solver<State>,
{
    /// Constructs a new `OdeModel` from its components.
    pub fn from_parts(state: State, dynamics: Dyn, solver: S) -> Self {
        Self {
            state,
            dynamics,
            solver,
        }
    }

    /// Replaces the current solver with a new one.
    pub fn with_solver<NewS: Solver<State>>(self, new_solver: NewS) -> OdeModel<State, Dyn, NewS> {
        OdeModel {
            state: self.state,
            dynamics: self.dynamics,
            solver: new_solver,
        }
    }
}

impl<State, Dyn, S> TimeStepper<State> for OdeModel<State, Dyn, S>
where
    State: VectorOperations,
    Dyn: OdeSystem<State>,
    S: Solver<State>,
{
    fn get_state(&self) -> &State {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    fn step(&mut self, dt: f64) {
        // Efficiently step without cloning, passing components separately
        self.solver.step(&self.dynamics, 0.0, &mut self.state, dt);
    }

    fn step_with<OtherS: Solver<State>>(&mut self, solver: &mut OtherS, dt: f64) {
        // Efficiently step with external solver without cloning
        solver.step(&self.dynamics, 0.0, &mut self.state, dt);
    }
}

impl<State, Dyn, S> OdeSystem<State> for OdeModel<State, Dyn, S>
where
    State: VectorOperations,
    Dyn: OdeSystem<State>,
{
    fn derivative(&self, t: f64, state: &State) -> State {
        self.dynamics.derivative(t, state)
    }

    fn derivative_in_place(&self, t: f64, state: &State, out: &mut State) {
        self.dynamics.derivative_in_place(t, state, out);
    }
}
