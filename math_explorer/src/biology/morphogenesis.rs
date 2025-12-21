//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

/// Defines the reaction kinetics for a Reaction-Diffusion system.
///
/// This trait allows users to plug in different reaction models (e.g., Schnakenberg,
/// Gierer-Meinhardt, Gray-Scott) without modifying the core solver.
pub trait ReactionKinetics {
    /// Computes the reaction rates for activator (u) and inhibitor (v).
    ///
    /// # Arguments
    ///
    /// * `u` - Current activator concentration.
    /// * `v` - Current inhibitor concentration.
    ///
    /// # Returns
    ///
    /// A tuple `(du/dt, dv/dt)` representing the reaction components of the time derivative.
    fn reaction(&self, u: f64, v: f64) -> (f64, f64);
}

/// Schnakenberg Kinetics.
///
/// Equations:
/// $$ \frac{\partial u}{\partial t} = a - u + u^2 v $$
/// $$ \frac{\partial v}{\partial t} = b - u^2 v $$
#[derive(Debug, Clone, Copy)]
pub struct SchnakenbergKinetics {
    /// Feed rate / constant a
    pub a: f64,
    /// Constant b
    pub b: f64,
}

impl SchnakenbergKinetics {
    /// Creates a new instance with specified parameters.
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
        let u2v = u.powi(2) * v;
        let du = self.a - u + u2v;
        let dv = self.b - u2v;
        (du, dv)
    }
}

/// Represents a 1D Reaction-Diffusion system.
pub struct TuringSystem<K: ReactionKinetics> {
    /// Activator concentrations
    pub u: Vec<f64>,
    /// Inhibitor concentrations
    pub v: Vec<f64>,
    /// Double buffer for activator
    u_next: Vec<f64>,
    /// Double buffer for inhibitor
    v_next: Vec<f64>,
    /// Diffusion coefficient for u
    pub d_u: f64,
    /// Diffusion coefficient for v
    pub d_v: f64,
    /// Grid spacing
    pub dx: f64,
    /// Reaction kinetics strategy
    pub kinetics: K,
}

impl<K: ReactionKinetics> TuringSystem<K> {
    /// Creates a new Turing System with the given kinetics.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64, kinetics: K) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            u_next: vec![0.0; size],
            v_next: vec![0.0; size],
            d_u,
            d_v,
            dx,
            kinetics,
        }
    }

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    pub fn step(&mut self, dt: f64) {
        let n = self.u.len();
        let dx_sq = self.dx * self.dx;

        for i in 0..n {
            // Zero flux (Neumann) boundary conditions
            let idx_prev = if i == 0 { 0 } else { i - 1 };
            let idx_next = if i == n - 1 { n - 1 } else { i + 1 };

            let u_curr = self.u[i];
            let v_curr = self.v[i];

            // Laplacian: (u_{i+1} - 2u_i + u_{i-1}) / dx^2
            let lap_u = (self.u[idx_next] - 2.0 * u_curr + self.u[idx_prev]) / dx_sq;
            let lap_v = (self.v[idx_next] - 2.0 * v_curr + self.v[idx_prev]) / dx_sq;

            // Calculate reaction rates using the strategy
            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            // Update next state
            self.u_next[i] = u_curr + dt * (self.d_u * lap_u + reaction_u);
            self.v_next[i] = v_curr + dt * (self.d_v * lap_v + reaction_v);
        }

        // Swap buffers to avoid allocation
        std::mem::swap(&mut self.u, &mut self.u_next);
        std::mem::swap(&mut self.v, &mut self.v_next);
    }
}
