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

/// Runge-Kutta 4th Order Solver.
///
/// A classic fixed-step integrator for ODEs.
/// It is generic over the `State` type, allowing for zero-cost abstractions.
pub struct RungeKutta4;

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
        let k1 = system.derivative(t, state);
        let k2 = system.derivative(t + dt / 2.0, &(state.clone() + k1.clone() * (dt / 2.0)));
        let k3 = system.derivative(t + dt / 2.0, &(state.clone() + k2.clone() * (dt / 2.0)));
        let k4 = system.derivative(t + dt, &(state.clone() + k3.clone() * dt));

        // delta = (k1 + 2k2 + 2k3 + k4) * dt / 6
        let k2_2 = k2 * 2.0;
        let k3_2 = k3 * 2.0;
        let sum_k = k1 + k2_2 + k3_2 + k4;
        let delta = sum_k * (dt / 6.0);

        state.clone() + delta
    }
}

/// Euler's Method Solver.
///
/// A first-order fixed-step integrator: $y_{n+1} = y_n + f(t_n, y_n) \cdot \Delta t$.
/// Less accurate than RK4 but faster and simpler.
pub struct Euler;

impl Euler {
    /// Performs a single integration step using Euler's method.
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
        let derivative = system.derivative(t, state);
        state.clone() + derivative * dt
    }
}

// Implement VectorOperations for Vec<f64> requires a wrapper or manual impl?
// Vec<f64> does NOT implement Add<Output=Vec<f64>> directly. It implements Add<&Vec<f64>> etc.
// But we need value-based Add for the trait bound `Add<Output=Self>`.
//
// This is a common issue with Rust's orphan rules and standard types.
// `Vec<f64> + Vec<f64>` is not implemented in std.
//
// To support `Vec<f64>`, we need a wrapper or helper.
// However, the `epidemiology` module uses `Vec<f64>`.
//
// OPTION: We can define a wrapper in `epidemiology.rs` or here.
// But to avoid "Tuple Soup", let's encourage using a Newtype.
//
// For backward compatibility with the existing `Vec<f64>` usage in `epidemiology.rs` (which I must fix),
// I will provide a `VecState` wrapper here that implements the necessary ops.

/// A wrapper around `Vec<f64>` that implements `VectorOperations`.
/// Use this when you need a heap-allocated state vector.
#[derive(Debug, Clone)]
pub struct VecState(pub Vec<f64>);

impl Add for VecState {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let res: Vec<f64> = self.0.iter().zip(rhs.0.iter()).map(|(a, b)| a + b).collect();
        VecState(res)
    }
}

impl Mul<f64> for VecState {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        VecState(self.0.iter().map(|x| x * rhs).collect())
    }
}

// Convenience conversion
impl From<Vec<f64>> for VecState {
    fn from(v: Vec<f64>) -> Self {
        VecState(v)
    }
}

impl From<VecState> for Vec<f64> {
    fn from(v: VecState) -> Self {
        v.0
    }
}
