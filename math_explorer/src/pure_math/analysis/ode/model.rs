use super::stepper::TimeStepper;
use super::traits::{OdeSystem, Solver, VectorOperations};

/// A generic container for an ODE model, combining State, Dynamics, and a Solver.
///
/// This struct implements the Facade pattern, providing a unified interface
/// to simulate a system defined by `Dynamics` using a `Solver`.
///
/// # Generics
/// * `State` - The state vector type (must implement `VectorOperations`).
/// * `Dynamics` - The system definition (must implement `OdeSystem<State>`).
/// * `S` - The numerical solver strategy (must implement `Solver<State>`).
#[derive(Debug, Clone)]
pub struct OdeModel<State, Dynamics, S> {
    /// The current state of the system.
    pub state: State,
    /// The physics/dynamics definition.
    pub dynamics: Dynamics,
    /// The numerical solver.
    pub solver: S,
}

impl<State, Dynamics, S> OdeModel<State, Dynamics, S> {
    /// Constructs a new OdeModel.
    pub fn new(state: State, dynamics: Dynamics, solver: S) -> Self {
        Self {
            state,
            dynamics,
            solver,
        }
    }
}

impl<State, Dynamics, S> OdeSystem<State> for OdeModel<State, Dynamics, S>
where
    State: VectorOperations,
    Dynamics: OdeSystem<State>,
    S: Solver<State>,
{
    fn derivative(&self, t: f64, state: &State) -> State {
        self.dynamics.derivative(t, state)
    }

    fn derivative_in_place(&self, t: f64, state: &State, out: &mut State) {
        self.dynamics.derivative_in_place(t, state, out);
    }
}

impl<State, Dynamics, S> TimeStepper<State> for OdeModel<State, Dynamics, S>
where
    State: VectorOperations,
    Dynamics: OdeSystem<State>,
    S: Solver<State>,
{
    fn get_state(&self) -> &State {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    fn step(&mut self, dt: f64) {
        // We pass &self.dynamics to the solver.
        // The solver needs a reference to something implementing OdeSystem<State>.
        // Dynamics implements OdeSystem<State>.
        self.solver
            .step(&self.dynamics, 0.0, &mut self.state, dt);
    }
}
