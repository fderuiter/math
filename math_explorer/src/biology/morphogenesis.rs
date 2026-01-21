//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

use crate::pure_math::analysis::ode::{EvolvingSystem, OdeSystem, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Defines the reaction kinetics for a 2-component reaction-diffusion system.
pub trait ReactionKinetics {
    /// Calculates the reaction rates for activator u and inhibitor v.
    ///
    /// # Arguments
    /// * `u` - Concentration of activator.
    /// * `v` - Concentration of inhibitor.
    ///
    /// # Returns
    /// A tuple `(du/dt, dv/dt)` representing the reaction terms.
    fn reaction(&self, u: f64, v: f64) -> (f64, f64);
}

/// Schnakenberg kinetics (often used for Turing patterns).
///
/// Equations:
/// $$ f(u, v) = a - u + u^2 v $$
/// $$ g(u, v) = b - u^2 v $$
#[derive(Debug, Clone, Copy)]
pub struct SchnakenbergKinetics {
    /// Production rate of activator.
    pub a: f64,
    /// Production rate of inhibitor.
    pub b: f64,
}

impl SchnakenbergKinetics {
    /// Creates a new Schnakenberg kinetics model.
    pub fn new(a: f64, b: f64) -> Self {
        Self { a, b }
    }
}

impl Default for SchnakenbergKinetics {
    fn default() -> Self {
        Self { a: 0.01, b: 0.05 }
    }
}

impl ReactionKinetics for SchnakenbergKinetics {
    fn reaction(&self, u: f64, v: f64) -> (f64, f64) {
        let uv_sq = u * u * v;
        let reaction_u = self.a - u + uv_sq;
        let reaction_v = self.b - uv_sq;
        (reaction_u, reaction_v)
    }
}

/// Represents the state of a Turing system at a point in time.
///
/// This struct encapsulates the concentration vectors for the activator and inhibitor,
/// protecting them from invalid resizing while providing safe access.
#[derive(Debug, Clone, PartialEq)]
pub struct TuringState {
    u: Vec<f64>,
    v: Vec<f64>,
}

impl TuringState {
    /// Creates a new zero-initialized state of a given size.
    pub fn new(size: usize) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
        }
    }

    /// Returns a slice of the activator concentrations.
    pub fn u(&self) -> &[f64] {
        &self.u
    }

    /// Returns a slice of the inhibitor concentrations.
    pub fn v(&self) -> &[f64] {
        &self.v
    }

    /// Returns a mutable slice of the activator concentrations.
    pub fn u_mut(&mut self) -> &mut [f64] {
        &mut self.u
    }

    /// Returns a mutable slice of the inhibitor concentrations.
    pub fn v_mut(&mut self) -> &mut [f64] {
        &mut self.v
    }

    /// Returns the length of the grid.
    pub fn len(&self) -> usize {
        self.u.len()
    }

    /// Returns true if the grid is empty.
    pub fn is_empty(&self) -> bool {
        self.u.is_empty()
    }
}

impl Add for TuringState {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        let len = std::cmp::min(self.u.len(), rhs.u.len());
        for (a, b) in self.u.iter_mut().zip(rhs.u.iter()).take(len) {
            *a += b;
        }
        for (a, b) in self.v.iter_mut().zip(rhs.v.iter()).take(len) {
            *a += b;
        }
        self
    }
}

impl AddAssign for TuringState {
    fn add_assign(&mut self, rhs: Self) {
        let len = std::cmp::min(self.u.len(), rhs.u.len());
        for (a, b) in self.u.iter_mut().zip(rhs.u.iter()).take(len) {
            *a += b;
        }
        for (a, b) in self.v.iter_mut().zip(rhs.v.iter()).take(len) {
            *a += b;
        }
    }
}

impl Mul<f64> for TuringState {
    type Output = Self;
    fn mul(mut self, scalar: f64) -> Self {
        for a in self.u.iter_mut() {
            *a *= scalar;
        }
        for a in self.v.iter_mut() {
            *a *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for TuringState {
    fn mul_assign(&mut self, scalar: f64) {
        for a in self.u.iter_mut() {
            *a *= scalar;
        }
        for a in self.v.iter_mut() {
            *a *= scalar;
        }
    }
}

impl VectorOperations for TuringState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        let len = std::cmp::min(self.u.len(), other.u.len());
        for (a, b) in self.u.iter_mut().zip(other.u.iter()).take(len) {
            *a += b * scale;
        }
        for (a, b) in self.v.iter_mut().zip(other.v.iter()).take(len) {
            *a += b * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        if self.len() != other.len() {
            self.u.resize(other.u.len(), 0.0);
            self.v.resize(other.v.len(), 0.0);
        }
        self.u.copy_from_slice(&other.u);
        self.v.copy_from_slice(&other.v);
    }
}

/// Represents a 1D Reaction-Diffusion system.
pub struct TuringSystem<K: ReactionKinetics = SchnakenbergKinetics> {
    /// The current state of the system.
    pub state: TuringState,

    // Double buffer for the next state.
    // Kept for the optimized `step` implementation.
    next_state: TuringState,

    /// Diffusion coefficient for u
    pub d_u: f64,
    /// Diffusion coefficient for v
    pub d_v: f64,
    /// Grid spacing
    pub dx: f64,
    /// Reaction kinetics strategy
    pub kinetics: K,
}

impl TuringSystem<SchnakenbergKinetics> {
    /// Creates a new Turing System with default Schnakenberg kinetics.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self::new_with_kinetics(size, d_u, d_v, dx, SchnakenbergKinetics::default())
    }
}

impl<K: ReactionKinetics> TuringSystem<K> {
    /// Creates a new Turing System with custom kinetics.
    pub fn new_with_kinetics(size: usize, d_u: f64, d_v: f64, dx: f64, kinetics: K) -> Self {
        Self {
            state: TuringState::new(size),
            next_state: TuringState::new(size),
            d_u,
            d_v,
            dx,
            kinetics,
        }
    }

    /// Accessor for the activator concentrations (backward compatibility/convenience).
    pub fn u(&self) -> &[f64] {
        self.state.u()
    }

    /// Accessor for the inhibitor concentrations (backward compatibility/convenience).
    pub fn v(&self) -> &[f64] {
        self.state.v()
    }

    /// Mutable accessor for the activator concentrations.
    pub fn u_mut(&mut self) -> &mut [f64] {
        self.state.u_mut()
    }

    /// Mutable accessor for the inhibitor concentrations.
    pub fn v_mut(&mut self) -> &mut [f64] {
        self.state.v_mut()
    }

    /// Internal function to calculate Laplacian terms and reaction terms.
    /// Used by both `step` (optimized) and `OdeSystem::derivative` (generic).
    /// Returns (lap_u, lap_v, reaction_u, reaction_v) at index i.
    #[inline(always)]
    fn calc_rates(
        d_u: f64,
        d_v: f64,
        kinetics: &K,
        u_prev: f64,
        u_curr: f64,
        u_next: f64,
        v_prev: f64,
        v_curr: f64,
        v_next: f64,
        inv_dx_sq: f64,
    ) -> (f64, f64) {
        let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
        let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

        let (reaction_u, reaction_v) = kinetics.reaction(u_curr, v_curr);

        // Return total rate: D * lap + reaction
        (d_u * lap_u + reaction_u, d_v * lap_v + reaction_v)
    }
}

impl<K: ReactionKinetics> OdeSystem<TuringState> for TuringSystem<K> {
    fn derivative(&self, _t: f64, state: &TuringState) -> TuringState {
        let n = state.len();
        let mut d_state = TuringState::new(n);

        if n == 0 {
            return d_state;
        }

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;
        let u = &state.u;
        let v = &state.v;

        // Naive implementation for derivative calculation (generic fallback)
        // Optimization is preserved in `step` override.
        // If one wanted to optimize RK4, this logic should be optimized too.

        for i in 0..n {
            let u_curr = u[i];
            let v_curr = v[i];

            // Handle boundaries (Dirichlet/Neumann or Periodic? Code implies Dirichlet/Neumann mix or just clamped)
            // Original code:
            // i=0: prev=curr, next=1 (if exists)
            // i=n-1: prev=n-2, next=curr

            let (u_prev, v_prev) = if i > 0 {
                (u[i - 1], v[i - 1])
            } else {
                (u_curr, v_curr)
            };
            let (u_next, v_next) = if i < n - 1 {
                (u[i + 1], v[i + 1])
            } else {
                (u_curr, v_curr)
            };

            let (du, dv) = Self::calc_rates(
                self.d_u,
                self.d_v,
                &self.kinetics,
                u_prev,
                u_curr,
                u_next,
                v_prev,
                v_curr,
                v_next,
                inv_dx_sq,
            );

            d_state.u[i] = du;
            d_state.v[i] = dv;
        }

        d_state
    }
}

impl<K: ReactionKinetics> EvolvingSystem<TuringState> for TuringSystem<K> {
    fn state(&self) -> &TuringState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut TuringState {
        &mut self.state
    }

    /// Optimized step implementation that overrides the default RK4.
    ///
    /// This implementation uses a finite-difference Laplacian and explicit time-stepping (Euler),
    /// heavily optimized with loop unrolling and sliding windows for cache locality.
    fn step(&mut self, dt: f64) {
        let n = self.state.len();
        if n == 0 {
            return;
        }

        // Ensure buffers are the right size
        if self.next_state.len() != n {
            self.next_state = TuringState::new(n);
        }

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;
        let d_u = self.d_u;
        let d_v = self.d_v;
        let kinetics = &self.kinetics;

        // Optimization: Lift boundary checks out of the loop and use slices
        let u = &self.state.u;
        let v = &self.state.v;
        let next_u = &mut self.next_state.u;
        let next_v = &mut self.next_state.v;

        // 1. Handle i = 0
        {
            let i = 0;
            // Safety: n > 0 checked above
            let u_curr = unsafe { *u.get_unchecked(i) };
            let v_curr = unsafe { *v.get_unchecked(i) };

            let u_prev = u_curr;
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                unsafe { (*u.get_unchecked(1), *v.get_unchecked(1)) }
            } else {
                (u_curr, v_curr)
            };

            let (du, dv) = Self::calc_rates(
                d_u, d_v, kinetics, u_prev, u_curr, u_next, v_prev, v_curr, v_next, inv_dx_sq,
            );

            unsafe {
                *next_u.get_unchecked_mut(i) = u_curr + dt * du;
                *next_v.get_unchecked_mut(i) = v_curr + dt * dv;
            }
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            // Optimization: Sliding Window / Register Rotation
            unsafe {
                let mut u_prev = *u.get_unchecked(0);
                let mut u_curr = *u.get_unchecked(1);
                let mut v_prev = *v.get_unchecked(0);
                let mut v_curr = *v.get_unchecked(1);

                for i in 1..n - 1 {
                    let u_next = *u.get_unchecked(i + 1);
                    let v_next = *v.get_unchecked(i + 1);

                    let (du, dv) = Self::calc_rates(
                        d_u, d_v, kinetics, u_prev, u_curr, u_next, v_prev, v_curr, v_next,
                        inv_dx_sq,
                    );

                    *next_u.get_unchecked_mut(i) = u_curr + dt * du;
                    *next_v.get_unchecked_mut(i) = v_curr + dt * dv;

                    // Shift window
                    u_prev = u_curr;
                    u_curr = u_next;
                    v_prev = v_curr;
                    v_curr = v_next;
                }
            }
        }

        // 3. Handle i = n-1
        if n > 1 {
            let i = n - 1;
            unsafe {
                let u_curr = *u.get_unchecked(i);
                let v_curr = *v.get_unchecked(i);
                let u_prev = *u.get_unchecked(i - 1);
                let v_prev = *v.get_unchecked(i - 1);
                let u_next = u_curr;
                let v_next = v_curr;

                let (du, dv) = Self::calc_rates(
                    d_u, d_v, kinetics, u_prev, u_curr, u_next, v_prev, v_curr, v_next, inv_dx_sq,
                );

                *next_u.get_unchecked_mut(i) = u_curr + dt * du;
                *next_v.get_unchecked_mut(i) = v_curr + dt * dv;
            }
        }

        // Swap buffers (states)
        std::mem::swap(&mut self.state, &mut self.next_state);
    }
}
