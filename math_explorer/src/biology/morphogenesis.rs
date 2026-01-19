//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

use crate::pure_math::analysis::ode::{OdeSystem, RungeKutta4, Solver, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Defines the reaction kinetics for a 2-component reaction-diffusion system.
pub trait ReactionKinetics {
    /// Calculates the reaction rates for activator u and inhibitor v.
    fn reaction(&self, u: f64, v: f64) -> (f64, f64);
}

/// Schnakenberg kinetics (often used for Turing patterns).
#[derive(Debug, Clone, Copy)]
pub struct SchnakenbergKinetics {
    pub a: f64,
    pub b: f64,
}

impl SchnakenbergKinetics {
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

/// The state of the Turing System (u and v concentrations).
#[derive(Debug, Clone, PartialEq)]
pub struct TuringState {
    pub u: Vec<f64>,
    pub v: Vec<f64>,
}

impl TuringState {
    pub fn new(size: usize) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
        }
    }
}

impl Add for TuringState {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        for (a, b) in self.u.iter_mut().zip(rhs.u.iter()) {
            *a += b;
        }
        for (a, b) in self.v.iter_mut().zip(rhs.v.iter()) {
            *a += b;
        }
        self
    }
}

impl AddAssign for TuringState {
    fn add_assign(&mut self, rhs: Self) {
        for (a, b) in self.u.iter_mut().zip(rhs.u.iter()) {
            *a += b;
        }
        for (a, b) in self.v.iter_mut().zip(rhs.v.iter()) {
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
        for (a, b) in self.u.iter_mut().zip(other.u.iter()) {
            *a += b * scale;
        }
        for (a, b) in self.v.iter_mut().zip(other.v.iter()) {
            *a += b * scale;
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
    /// The current state (u and v).
    pub state: TuringState,
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
            d_u,
            d_v,
            dx,
            kinetics,
        }
    }

    /// Updates the grid using the default RK4 solver.
    pub fn step(&mut self, dt: f64) {
        self.state = RungeKutta4::step(self, 0.0, &self.state, dt);
    }

    /// Updates the grid using a provided solver.
    pub fn step_with<S: Solver<TuringState>>(&mut self, solver: &S, dt: f64) {
        self.state = solver.solve(self, 0.0, &self.state, dt);
    }
}

impl<K: ReactionKinetics> OdeSystem<TuringState> for TuringSystem<K> {
    fn derivative(&self, _t: f64, state: &TuringState) -> TuringState {
        let n = state.u.len();
        let mut deriv = TuringState::new(n);
        self.derivative_in_place(_t, state, &mut deriv);
        deriv
    }

    fn derivative_in_place(&self, _t: f64, state: &TuringState, out: &mut TuringState) {
        let n = state.u.len();
        if n == 0 {
            return;
        }

        let dx_sq = self.dx * self.dx;
        let inv_dx_sq = 1.0 / dx_sq;
        let u = &state.u;
        let v = &state.v;
        let du_dt = &mut out.u;
        let dv_dt = &mut out.v;

        // Ensure output is sized correctly
        if du_dt.len() != n {
            du_dt.resize(n, 0.0);
            dv_dt.resize(n, 0.0);
        }

        // Safe implementation using get (or get_unchecked if strict optimization needed, keeping safe for now)
        // Boundary i=0
        {
            let i = 0;
            let u_curr = u[i];
            let v_curr = v[i];
            let u_prev = u_curr; // Dirichlet or Neumann? Code implies simple neighbor check, let's stick to existing logic: index -1 -> index 0
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 { (u[i+1], v[i+1]) } else { (u_curr, v_curr) };

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;
            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            du_dt[i] = self.d_u * lap_u + reaction_u;
            dv_dt[i] = self.d_v * lap_v + reaction_v;
        }

        // Interior
        for i in 1..n-1 {
            let u_curr = u[i];
            let v_curr = v[i];
            let u_prev = u[i-1];
            let v_prev = v[i-1];
            let u_next = u[i+1];
            let v_next = v[i+1];

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;
            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            du_dt[i] = self.d_u * lap_u + reaction_u;
            dv_dt[i] = self.d_v * lap_v + reaction_v;
        }

        // Boundary i=n-1
        if n > 1 {
            let i = n - 1;
            let u_curr = u[i];
            let v_curr = v[i];
            let u_prev = u[i-1];
            let v_prev = v[i-1];
            let u_next = u_curr;
            let v_next = v_curr;

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;
            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            du_dt[i] = self.d_u * lap_u + reaction_u;
            dv_dt[i] = self.d_v * lap_v + reaction_v;
        }
    }
}
