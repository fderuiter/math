use super::traits::{OdeSystem, Solver, VectorOperations};

/// Euler's Method Solver.
pub struct Euler;

impl<State: VectorOperations> Solver<State> for Euler {
    fn solve<S>(&self, system: &S, t: f64, state: &State, dt: f64) -> State
    where
        S: OdeSystem<State> + ?Sized,
    {
        let derivative = system.derivative(t, state);
        state.clone() + derivative * dt
    }

    fn step<S>(&self, system: &S, t: f64, state: &mut State, dt: f64)
    where
        S: OdeSystem<State> + ?Sized,
    {
        // Simple Euler step: y += f(t, y) * dt
        let mut derivative = state.clone(); // Template allocation
        system.derivative_in_place(t, state, &mut derivative);
        state.scale_add(&derivative, dt);
    }
}

/// Runge-Kutta 4th Order Solver.
///
/// A classic fixed-step integrator for ODEs.
/// It is generic over the `State` type, allowing for zero-cost abstractions.
pub struct RungeKutta4;

impl<State: VectorOperations> Solver<State> for RungeKutta4 {
    fn solve<S>(&self, system: &S, t: f64, state: &State, dt: f64) -> State
    where
        S: OdeSystem<State> + ?Sized,
    {
        // Fallback to in-place implementation
        let mut new_state = state.clone();
        self.step(system, t, &mut new_state, dt);
        new_state
    }

    fn step<S>(&self, system: &S, t: f64, state: &mut State, dt: f64)
    where
        S: OdeSystem<State> + ?Sized,
    {
        // Allocation: 3 vectors (k, tmp, initial_state).
        let initial_state = state.clone();
        let mut k = initial_state.clone();
        let mut tmp = initial_state.clone();

        // k1 = f(t, y)
        system.derivative_in_place(t, &initial_state, &mut k);
        // y += k1 * dt/6
        state.scale_add(&k, dt / 6.0);

        // k2 = f(t + dt/2, y + k1 * dt/2)
        // tmp = y + k1 * dt/2
        tmp.copy_from(&initial_state);
        tmp.scale_add(&k, dt / 2.0);
        system.derivative_in_place(t + dt / 2.0, &tmp, &mut k);
        // y += k2 * dt/3
        state.scale_add(&k, dt / 3.0);

        // k3 = f(t + dt/2, y + k2 * dt/2)
        // tmp = y + k2 * dt/2
        tmp.copy_from(&initial_state);
        tmp.scale_add(&k, dt / 2.0);
        system.derivative_in_place(t + dt / 2.0, &tmp, &mut k);
        // y += k3 * dt/3
        state.scale_add(&k, dt / 3.0);

        // k4 = f(t + dt, y + k3 * dt)
        // tmp = y + k3 * dt
        tmp.copy_from(&initial_state);
        tmp.scale_add(&k, dt);
        system.derivative_in_place(t + dt, &tmp, &mut k);
        // y += k4 * dt/6
        state.scale_add(&k, dt / 6.0);
    }
}

impl RungeKutta4 {
    /// Performs a single integration step.
    pub fn step<State, S>(system: &S, t: f64, state: &State, dt: f64) -> State
    where
        State: VectorOperations,
        S: OdeSystem<State> + ?Sized,
    {
        // Delegate to the trait implementation via a temporary instance
        let solver = RungeKutta4;
        solver.solve(system, t, state, dt)
    }
}
