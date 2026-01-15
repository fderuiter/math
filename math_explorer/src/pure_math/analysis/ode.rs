//! Ordinary Differential Equation (ODE) solvers.
//!
//! This module provides a generic `OdeSystem` trait for defining ODEs and solvers
//! like `RungeKutta4` to integrate them numerically.
//!
//! The design relies on the `VectorOperations` trait to allow the solvers to work
//! with any vector-like type (e.g., `Vec<f64>`, `nalgebra::Vector3<f64>`, etc.),
//! avoiding heap allocations when fixed-size arrays are sufficient.

use std::ops::{Add, AddAssign, Mul, MulAssign};

/// A trait defining the vector operations required by numerical integrators.
///
/// This trait allows the solver to be agnostic of the underlying storage (Heap vs Stack).
/// It requires the type to support addition and scalar multiplication.
pub trait VectorOperations:
    Sized + Clone + Add<Output = Self> + Mul<f64, Output = Self> + AddAssign + MulAssign<f64>
{
    /// Adds `other` scaled by `scale` to `self`.
    /// `self += other * scale`
    fn scale_add(&mut self, other: &Self, scale: f64);

    /// Copies the content of `other` into `self`.
    /// This allows reusing allocated buffers.
    fn copy_from(&mut self, other: &Self);
}

/// A wrapper around `Vec<f64>` that implements `VectorOperations`.
/// Use this when you need a heap-allocated state vector.
#[derive(Debug, Clone, PartialEq)]
pub struct VecState(pub Vec<f64>);

impl Add for VecState {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        let len = std::cmp::min(self.0.len(), rhs.0.len());
        self.0.truncate(len);
        for (a, b) in self.0.iter_mut().zip(rhs.0.iter()) {
            *a += b;
        }
        self
    }
}

impl AddAssign for VecState {
    fn add_assign(&mut self, rhs: Self) {
        let len = std::cmp::min(self.0.len(), rhs.0.len());
        // Use zip to avoid bounds checks and handle length mismatch gracefully
        for (a, b) in self.0.iter_mut().zip(rhs.0.iter()).take(len) {
            *a += b;
        }
    }
}

impl Mul<f64> for VecState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for val in self.0.iter_mut() {
            *val *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for VecState {
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.0.iter_mut() {
            *val *= scalar;
        }
    }
}

impl VectorOperations for VecState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        let len = std::cmp::min(self.0.len(), other.0.len());
        for (a, b) in self.0.iter_mut().zip(other.0.iter()).take(len) {
            *a += b * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        // Reuse buffer if possible
        if self.0.len() != other.0.len() {
            self.0.resize(other.0.len(), 0.0);
        }
        self.0.copy_from_slice(&other.0);
    }
}

// Implementations for nalgebra types
use nalgebra::{DVector, Vector2, Vector3};

impl VectorOperations for Vector2<f64> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        *self += other * scale;
    }

    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl VectorOperations for Vector3<f64> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        *self += other * scale;
    }

    fn copy_from(&mut self, other: &Self) {
        *self = *other;
    }
}

impl VectorOperations for DVector<f64> {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        // Use slice iteration to avoid temporary allocations from 'other * scale'
        // This assumes DVector storage is contiguous which it is for standard DVector.
        for (a, b) in self.as_mut_slice().iter_mut().zip(other.as_slice().iter()) {
            *a += b * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        self.copy_from(other);
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
    ///
    /// # Performance
    /// Default implementation allocates a new State. Implement `derivative_in_place` for better performance.
    fn derivative(&self, t: f64, state: &State) -> State;

    /// Computes the time derivative of the system state in-place.
    ///
    /// # Arguments
    /// * `t` - The current time.
    /// * `state` - The current state vector $\vec{y}$.
    /// * `out` - The output vector to store $\frac{d\vec{y}}{dt}$.
    fn derivative_in_place(&self, t: f64, state: &State, out: &mut State) {
        *out = self.derivative(t, state);
    }
}

/// A trait defining a numerical ODE solver strategy.
pub trait Solver<State: VectorOperations> {
    /// Advances the system state by one time step `dt`.
    fn solve<S>(&self, system: &S, t: f64, state: &State, dt: f64) -> State
    where
        S: OdeSystem<State> + ?Sized;

    /// Advances the system state by one time step `dt` in-place.
    fn step<S>(&self, system: &S, t: f64, state: &mut State, dt: f64)
    where
        S: OdeSystem<State> + ?Sized;
}

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
