//! Ordinary Differential Equation (ODE) solvers.
//!
//! This module provides a generic `OdeSystem` trait for defining ODEs and solvers
//! like `RungeKutta4` to integrate them numerically.
//!
//! The design relies on the `VectorOperations` trait to allow the solvers to work
//! with any vector-like type (e.g., `Vec<f64>`, `nalgebra::Vector3<f64>`, etc.),
//! avoiding heap allocations when fixed-size arrays are sufficient.

use std::ops::{Add, Mul};

/// A trait defining the vector operations required by numerical integrators.
///
/// This trait allows the solver to be agnostic of the underlying storage (Heap vs Stack).
/// It requires the type to support addition and scalar multiplication.
pub trait VectorOperations: Sized + Clone + Add<Output = Self> + Mul<f64, Output = Self> {
    // No extra methods needed, just the supertraits.
}

// Blanket implementation for any type that satisfies the bounds.
impl<T> VectorOperations for T where T: Sized + Clone + Add<Output = Self> + Mul<f64, Output = Self> {}

/// A wrapper around `Vec<f64>` that implements `VectorOperations`.
/// Use this when you need a heap-allocated state vector.
#[derive(Debug, Clone, PartialEq)]
pub struct VecState(pub Vec<f64>);

impl Add for VecState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        // In a production system, we might want to check for length mismatch.
        // For now, zip will truncate to the shorter length, but states should be consistent.
        let new_data: Vec<f64> = self.0.iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| a + b)
            .collect();
        VecState(new_data)
    }
}

impl Mul<f64> for VecState {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        let new_data: Vec<f64> = self.0.iter()
            .map(|val| val * scalar)
            .collect();
        VecState(new_data)
    }
}

/// A trait defining a system of Ordinary Differential Equations.
///
/// $$ \frac{d\vec{y}}{dt} = f(t, \vec{y}) $$
///
/// The state type `State` must implement `VectorOperations`.
pub trait OdeSystem<State: VectorOperations> {
    /// Computes the time derivative of the system state.
    ///
    /// # Arguments
    /// * `t` - The current time.
    /// * `state` - The current state vector $\vec{y}$.
    ///
    /// # Returns
    /// The derivative vector $\frac{d\vec{y}}{dt}$.
    fn derivative(&self, t: f64, state: &State) -> State;
}

/// A trait defining a numerical ODE solver strategy.
///
/// This allows different integration schemes (e.g., Euler, Runge-Kutta) to be swapped
/// interchangeably, adhering to the Strategy Pattern.
pub trait Solver<State: VectorOperations> {
    /// Advances the system state by one time step `dt`.
    ///
    /// # Arguments
    /// * `system` - The ODE system defining the derivatives.
    /// * `t` - The current time.
    /// * `state` - The current state vector.
    /// * `dt` - The time step size.
    fn solve<S>(&self, system: &S, t: f64, state: &State, dt: f64) -> State
    where
        S: OdeSystem<State> + ?Sized;
}

/// Euler's Method Solver.
///
/// A simple first-order integrator: $y_{n+1} = y_n + f(t_n, y_n) \cdot \Delta t$.
/// Fast but less accurate; useful for stiff systems or performance-critical approximations.
pub struct Euler;

impl<State: VectorOperations> Solver<State> for Euler {
    fn solve<S>(&self, system: &S, t: f64, state: &State, dt: f64) -> State
    where
        S: OdeSystem<State> + ?Sized,
    {
        let derivative = system.derivative(t, state);
        state.clone() + derivative * dt
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
        // Re-use the static logic (duplicated here to avoid self-borrowing quirks, though straightforward).
        // delta = (k1 + 2k2 + 2k3 + k4) * dt / 6

        let k1 = system.derivative(t, state);
        let k2 = system.derivative(t + dt / 2.0, &(state.clone() + k1.clone() * (dt / 2.0)));
        let k3 = system.derivative(t + dt / 2.0, &(state.clone() + k2.clone() * (dt / 2.0)));
        let k4 = system.derivative(t + dt, &(state.clone() + k3.clone() * dt));

        let k2_2 = k2 * 2.0;
        let k3_2 = k3 * 2.0;
        let sum_k = k1 + k2_2 + k3_2 + k4;
        let delta = sum_k * (dt / 6.0);

        state.clone() + delta
    }
}

impl RungeKutta4 {
    /// Performs a single integration step.
    ///
    /// # Arguments
    /// * `system` - The ODE system to solve.
    /// * `t` - The current time.
    /// * `state` - The current state vector.
    /// * `dt` - The time step size.
    ///
    /// # Returns
    /// The new state vector after time `dt`.
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
