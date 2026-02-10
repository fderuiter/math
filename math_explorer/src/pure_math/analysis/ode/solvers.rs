use super::error::OdeError;
use super::traits::{OdeSystem, Solver, VectorOperations};

/// Euler's Method Solver.
///
/// A simple first-order numerical integrator.
/// Maintains an internal buffer to avoid allocations during steps.
#[derive(Debug, Clone)]
pub struct Euler<State> {
    buffer: Option<State>,
}

impl<State> Default for Euler<State> {
    fn default() -> Self {
        Self { buffer: None }
    }
}

impl<State> Euler<State> {
    /// Creates a new Euler solver.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<State: VectorOperations> Solver<State> for Euler<State> {
    fn solve<S>(&mut self, system: &S, t: f64, state: &State, dt: f64) -> Result<State, OdeError>
    where
        S: OdeSystem<State> + ?Sized,
    {
        // For solve (returning new state), we can clone the input and step in-place.
        let mut new_state = state.clone();
        self.step(system, t, &mut new_state, dt)?;
        Ok(new_state)
    }

    fn step<S>(&mut self, system: &S, t: f64, state: &mut State, dt: f64) -> Result<(), OdeError>
    where
        S: OdeSystem<State> + ?Sized,
    {
        // Initialize buffer if needed or if size changed
        let derivative = self.buffer.get_or_insert_with(|| state.clone());

        // Ensure buffer is ready (in case of re-use with different size, though specific implementations handle copy/resize)
        // Ideally we'd check dimensions here if VectorOperations exposed them.
        // For now, we rely on the implementation of copy_from or implicit behavior.

        // derivative = f(t, y)
        system.derivative_in_place(t, state, derivative);

        // y += derivative * dt
        state.scale_add(derivative, dt);

        Ok(())
    }
}

/// Runge-Kutta 4th Order Solver.
///
/// A classic fixed-step integrator for ODEs.
/// Maintains internal buffers to avoid allocations.
#[derive(Debug, Clone)]
pub struct RungeKutta4<State> {
    k: Option<State>,
    tmp: Option<State>,
    initial_state: Option<State>,
}

impl<State> Default for RungeKutta4<State> {
    fn default() -> Self {
        Self {
            k: None,
            tmp: None,
            initial_state: None,
        }
    }
}

impl<State> RungeKutta4<State> {
    /// Creates a new Runge-Kutta 4 solver.
    pub fn new() -> Self {
        Self::default()
    }

    /// Performs a single integration step using a temporary solver.
    ///
    /// This method allocates a new solver (and thus buffers) on every call.
    /// For performance-critical code, instantiate a `RungeKutta4` struct and reuse it.
    pub fn step<S>(system: &S, t: f64, state: &State, dt: f64) -> Result<State, OdeError>
    where
        State: VectorOperations,
        S: OdeSystem<State> + ?Sized,
    {
        let mut solver = Self::default();
        solver.solve(system, t, state, dt)
    }
}

impl<State: VectorOperations> Solver<State> for RungeKutta4<State> {
    fn solve<S>(&mut self, system: &S, t: f64, state: &State, dt: f64) -> Result<State, OdeError>
    where
        S: OdeSystem<State> + ?Sized,
    {
        let mut new_state = state.clone();
        self.step(system, t, &mut new_state, dt)?;
        Ok(new_state)
    }

    fn step<S>(&mut self, system: &S, t: f64, state: &mut State, dt: f64) -> Result<(), OdeError>
    where
        S: OdeSystem<State> + ?Sized,
    {
        // Lazy initialization of buffers
        // We use get_or_insert_with to avoid unwrap()
        let k = self.k.get_or_insert_with(|| state.clone());
        let tmp = self.tmp.get_or_insert_with(|| state.clone());
        let initial_state = self.initial_state.get_or_insert_with(|| state.clone());

        // Copy current state to initial_state buffer to preserve it
        initial_state.copy_from(state);

        // k1 = f(t, y)
        system.derivative_in_place(t, initial_state, k);
        // y += k1 * dt/6
        state.scale_add(k, dt / 6.0);

        // k2 = f(t + dt/2, y + k1 * dt/2)
        // tmp = y + k1 * dt/2
        tmp.copy_from(initial_state);
        tmp.scale_add(k, dt / 2.0);
        system.derivative_in_place(t + dt / 2.0, tmp, k);
        // y += k2 * dt/3
        state.scale_add(k, dt / 3.0);

        // k3 = f(t + dt/2, y + k2 * dt/2)
        // tmp = y + k2 * dt/2
        tmp.copy_from(initial_state);
        tmp.scale_add(k, dt / 2.0);
        system.derivative_in_place(t + dt / 2.0, tmp, k);
        // y += k3 * dt/3
        state.scale_add(k, dt / 3.0);

        // k4 = f(t + dt, y + k3 * dt)
        // tmp = y + k3 * dt
        tmp.copy_from(initial_state);
        tmp.scale_add(k, dt);
        system.derivative_in_place(t + dt, tmp, k);
        // y += k4 * dt/6
        state.scale_add(k, dt / 6.0);

        Ok(())
    }
}
