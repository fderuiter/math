//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$
//!
//! # Architecture
//!
//! This module uses the **Strategy Pattern** to decouple the reaction kinetics from the diffusion solver.
//! It also employs **Double Buffering** to avoid heap allocations during simulation steps.

/// Defines the reaction kinetics for a Reaction-Diffusion system.
///
/// Implementers should define the reaction rates for $u$ and $v$ given their current local concentrations.
pub trait ReactionKinetics {
    /// Calculates the reaction term for the activator $u$.
    ///
    /// $$ f(u, v) $$
    fn reaction_u(&self, u: f64, v: f64) -> f64;

    /// Calculates the reaction term for the inhibitor $v$.
    ///
    /// $$ g(u, v) $$
    fn reaction_v(&self, u: f64, v: f64) -> f64;
}

/// Schnakenberg Kinetics.
///
/// A classic model for Turing patterns.
/// $$ u_t = a - u + u^2 v $$
/// $$ v_t = b - u^2 v $$
#[derive(Debug, Clone, Copy)]
pub struct SchnakenbergKinetics {
    pub a: f64,
    pub b: f64,
}

impl SchnakenbergKinetics {
    /// Creates a new Schnakenberg kinetics model with standard parameters.
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
    fn reaction_u(&self, u: f64, v: f64) -> f64 {
        self.a - u + u.powi(2) * v
    }

    fn reaction_v(&self, u: f64, v: f64) -> f64 {
        self.b - u.powi(2) * v
    }
}

/// Gray-Scott Kinetics.
///
/// Another popular model for pattern formation.
/// $$ u_t = -uv^2 + F(1-u) $$
/// $$ v_t = uv^2 - (F+k)v $$
#[derive(Debug, Clone, Copy)]
pub struct GrayScottKinetics {
    pub f: f64,
    pub k: f64,
}

impl GrayScottKinetics {
    pub fn new(f: f64, k: f64) -> Self {
        Self { f, k }
    }
}

impl ReactionKinetics for GrayScottKinetics {
    fn reaction_u(&self, u: f64, v: f64) -> f64 {
        -u * v.powi(2) + self.f * (1.0 - u)
    }

    fn reaction_v(&self, u: f64, v: f64) -> f64 {
        u * v.powi(2) - (self.f + self.k) * v
    }
}

/// Represents a 1D Reaction-Diffusion system.
///
/// It is generic over the reaction kinetics `K`.
pub struct TuringSystem<K: ReactionKinetics = SchnakenbergKinetics> {
    /// Activator concentrations (Current State)
    pub u: Vec<f64>,
    /// Inhibitor concentrations (Current State)
    pub v: Vec<f64>,

    // Double Buffering: Next State
    next_u: Vec<f64>,
    next_v: Vec<f64>,

    /// Diffusion coefficient for u
    pub d_u: f64,
    /// Diffusion coefficient for v
    pub d_v: f64,
    /// Grid spacing
    pub dx: f64,

    /// The reaction kinetics strategy
    pub kinetics: K,
}

impl TuringSystem<SchnakenbergKinetics> {
    /// Creates a new TuringSystem with default Schnakenberg kinetics.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self::with_kinetics(size, d_u, d_v, dx, SchnakenbergKinetics::default())
    }
}

impl<K: ReactionKinetics> TuringSystem<K> {
    /// Creates a new TuringSystem with a specific kinetics strategy.
    pub fn with_kinetics(size: usize, d_u: f64, d_v: f64, dx: f64, kinetics: K) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            next_u: vec![0.0; size],
            next_v: vec![0.0; size],
            d_u,
            d_v,
            dx,
            kinetics,
        }
    }

    /// Updates the grid using a finite-difference Laplacian and the injected reaction kinetics.
    ///
    /// This method uses double buffering to avoid heap allocations.
    pub fn step(&mut self, dt: f64) {
        let n = self.u.len();
        let inv_dx2 = 1.0 / (self.dx * self.dx);

        for i in 0..n {
            // Boundary Conditions: Zero Flux (Neumann)
            // ghost points: u[-1] = u[0], u[n] = u[n-1]
            let idx_prev = if i == 0 { 0 } else { i - 1 };
            let idx_next = if i == n - 1 { n - 1 } else { i + 1 };

            let u_curr = self.u[i];
            let v_curr = self.v[i];

            // Laplacian: (u_{i+1} - 2u_i + u_{i-1}) / dx^2
            let lap_u = (self.u[idx_next] - 2.0 * u_curr + self.u[idx_prev]) * inv_dx2;
            let lap_v = (self.v[idx_next] - 2.0 * v_curr + self.v[idx_prev]) * inv_dx2;

            // Reaction terms from Strategy
            let reaction_u = self.kinetics.reaction_u(u_curr, v_curr);
            let reaction_v = self.kinetics.reaction_v(u_curr, v_curr);

            // Time Integration (Euler)
            self.next_u[i] = u_curr + dt * (self.d_u * lap_u + reaction_u);
            self.next_v[i] = v_curr + dt * (self.d_v * lap_v + reaction_v);
        }

        // Swap buffers
        // Rust's `std::mem::swap` is efficient, but we want `next` to become `current`
        // and reuse `current` memory for next `next`.
        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }
}
