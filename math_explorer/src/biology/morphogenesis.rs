//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

use crate::pure_math::analysis::ode::{OdeSystem, Solver, TimeStepper, VectorOperations};
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
#[derive(Debug, Clone)]
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
        for (u, u_rhs) in self.u.iter_mut().zip(rhs.u.iter()) {
            *u += u_rhs;
        }
        for (v, v_rhs) in self.v.iter_mut().zip(rhs.v.iter()) {
            *v += v_rhs;
        }
        self
    }
}

impl AddAssign for TuringState {
    fn add_assign(&mut self, rhs: Self) {
        for (u, u_rhs) in self.u.iter_mut().zip(rhs.u.iter()) {
            *u += u_rhs;
        }
        for (v, v_rhs) in self.v.iter_mut().zip(rhs.v.iter()) {
            *v += v_rhs;
        }
    }
}

impl Mul<f64> for TuringState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for u in self.u.iter_mut() {
            *u *= scalar;
        }
        for v in self.v.iter_mut() {
            *v *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for TuringState {
    fn mul_assign(&mut self, scalar: f64) {
        for u in self.u.iter_mut() {
            *u *= scalar;
        }
        for v in self.v.iter_mut() {
            *v *= scalar;
        }
    }
}

impl VectorOperations for TuringState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (u, u_other) in self.u.iter_mut().zip(other.u.iter()) {
            *u += u_other * scale;
        }
        for (v, v_other) in self.v.iter_mut().zip(other.v.iter()) {
            *v += v_other * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        if self.u.len() != other.u.len() {
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

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    pub fn step(&mut self, dt: f64) {
        <Self as TimeStepper<TuringState>>::step(self, dt);
    }

    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    fn compute_rates(
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
        (d_u * lap_u + reaction_u, d_v * lap_v + reaction_v)
    }
}

impl<K: ReactionKinetics> TimeStepper<TuringState> for TuringSystem<K> {
    fn get_state(&self) -> &TuringState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut TuringState {
        &mut self.state
    }

    /// Optimized Euler step implementation reusing the internal double buffer.
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

        let u = &self.state.u;
        let v = &self.state.v;
        let next_u = &mut self.next_state.u;
        let next_v = &mut self.next_state.v;

        // 1. Handle i = 0
        {
            let i = 0;
            let u_curr = unsafe { *u.get_unchecked(i) };
            let v_curr = unsafe { *v.get_unchecked(i) };

            let u_prev = u_curr;
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                unsafe { (*u.get_unchecked(1), *v.get_unchecked(1)) }
            } else {
                (u_curr, v_curr)
            };

            let (rate_u, rate_v) = Self::compute_rates(
                self.d_u, self.d_v, &self.kinetics, u_prev, u_curr, u_next, v_prev, v_curr, v_next,
                inv_dx_sq,
            );

            unsafe {
                *next_u.get_unchecked_mut(i) = u_curr + dt * rate_u;
                *next_v.get_unchecked_mut(i) = v_curr + dt * rate_v;
            }
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            unsafe {
                let mut u_prev = *u.get_unchecked(0);
                let mut u_curr = *u.get_unchecked(1);
                let mut v_prev = *v.get_unchecked(0);
                let mut v_curr = *v.get_unchecked(1);

                for i in 1..n - 1 {
                    let u_next = *u.get_unchecked(i + 1);
                    let v_next = *v.get_unchecked(i + 1);

                    let (rate_u, rate_v) = Self::compute_rates(
                        self.d_u, self.d_v, &self.kinetics, u_prev, u_curr, u_next, v_prev,
                        v_curr, v_next, inv_dx_sq,
                    );

                    *next_u.get_unchecked_mut(i) = u_curr + dt * rate_u;
                    *next_v.get_unchecked_mut(i) = v_curr + dt * rate_v;

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

                let (rate_u, rate_v) = Self::compute_rates(
                    self.d_u, self.d_v, &self.kinetics, u_prev, u_curr, u_next, v_prev, v_curr,
                    v_next, inv_dx_sq,
                );

                *next_u.get_unchecked_mut(i) = u_curr + dt * rate_u;
                *next_v.get_unchecked_mut(i) = v_curr + dt * rate_v;
            }
        }

        std::mem::swap(&mut self.state, &mut self.next_state);
    }
}

impl<K: ReactionKinetics> OdeSystem<TuringState> for TuringSystem<K> {
    fn derivative(&self, t: f64, state: &TuringState) -> TuringState {
        let mut out = TuringState::new(state.len());
        self.derivative_in_place(t, state, &mut out);
        out
    }

    fn derivative_in_place(&self, _t: f64, state: &TuringState, out: &mut TuringState) {
        let n = state.len();
        if n == 0 {
            return;
        }

        if out.len() != n {
            out.u.resize(n, 0.0);
            out.v.resize(n, 0.0);
        }

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;

        let u = &state.u;
        let v = &state.v;
        let out_u = &mut out.u;
        let out_v = &mut out.v;

        // 1. Handle i = 0
        {
            let i = 0;
            let u_curr = unsafe { *u.get_unchecked(i) };
            let v_curr = unsafe { *v.get_unchecked(i) };

            let u_prev = u_curr;
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                unsafe { (*u.get_unchecked(1), *v.get_unchecked(1)) }
            } else {
                (u_curr, v_curr)
            };

            let (rate_u, rate_v) = Self::compute_rates(
                self.d_u, self.d_v, &self.kinetics, u_prev, u_curr, u_next, v_prev, v_curr, v_next,
                inv_dx_sq,
            );

            unsafe {
                *out_u.get_unchecked_mut(i) = rate_u;
                *out_v.get_unchecked_mut(i) = rate_v;
            }
        }

        // 2. Handle i = 1..n-1
        if n > 2 {
            unsafe {
                let mut u_prev = *u.get_unchecked(0);
                let mut u_curr = *u.get_unchecked(1);
                let mut v_prev = *v.get_unchecked(0);
                let mut v_curr = *v.get_unchecked(1);

                for i in 1..n - 1 {
                    let u_next = *u.get_unchecked(i + 1);
                    let v_next = *v.get_unchecked(i + 1);

                    let (rate_u, rate_v) = Self::compute_rates(
                        self.d_u, self.d_v, &self.kinetics, u_prev, u_curr, u_next, v_prev,
                        v_curr, v_next, inv_dx_sq,
                    );

                    *out_u.get_unchecked_mut(i) = rate_u;
                    *out_v.get_unchecked_mut(i) = rate_v;

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

                let (rate_u, rate_v) = Self::compute_rates(
                    self.d_u, self.d_v, &self.kinetics, u_prev, u_curr, u_next, v_prev, v_curr,
                    v_next, inv_dx_sq,
                );

                *out_u.get_unchecked_mut(i) = rate_u;
                *out_v.get_unchecked_mut(i) = rate_v;
            }
        }
    }
}
