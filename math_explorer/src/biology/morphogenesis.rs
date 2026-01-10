//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

use super::kinetics::{ReactionKinetics, GiererMeinhardtKinetics};

/// Represents a 1D Reaction-Diffusion system.
pub struct TuringSystem<K: ReactionKinetics = GiererMeinhardtKinetics> {
    /// Activator concentrations
    pub u: Vec<f64>,
    /// Inhibitor concentrations
    pub v: Vec<f64>,
    /// Diffusion coefficient for u
    pub d_u: f64,
    /// Diffusion coefficient for v
    pub d_v: f64,
    /// Grid spacing
    pub dx: f64,
    /// Reaction kinetics strategy
    pub kinetics: K,

    // Private buffers for double buffering to avoid allocation in hot loops.
    // Marked hidden to discourage manual usage if fields were public (which they are).
    #[doc(hidden)]
    buffer_u: Vec<f64>,
    #[doc(hidden)]
    buffer_v: Vec<f64>,
}

impl TuringSystem<GiererMeinhardtKinetics> {
    /// Creates a new TuringSystem with default Gierer-Meinhardt kinetics.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self::new_with_kinetics(size, d_u, d_v, dx, GiererMeinhardtKinetics::default())
    }
}

impl<K: ReactionKinetics> TuringSystem<K> {
    /// Creates a new TuringSystem with custom reaction kinetics.
    pub fn new_with_kinetics(size: usize, d_u: f64, d_v: f64, dx: f64, kinetics: K) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            d_u,
            d_v,
            dx,
            kinetics,
            buffer_u: vec![0.0; size],
            buffer_v: vec![0.0; size],
        }
    }

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    pub fn step(&mut self, dt: f64) {
        let n = self.u.len();
        if n == 0 { return; }

        // Ensure buffers are the right size
        if self.buffer_u.len() != n {
            self.buffer_u = vec![0.0; n];
        }
        if self.buffer_v.len() != n {
            self.buffer_v = vec![0.0; n];
        }

        let dx_sq = self.dx * self.dx;

        // Optimization: Lift boundary checks out of the loop

        // Helper closure to calculate update for a single index
        let calculate_update = |u_curr: f64, v_curr: f64, u_prev: f64, u_next: f64, v_prev: f64, v_next: f64| -> (f64, f64) {
            let lap_u = (u_next - 2.0 * u_curr + u_prev) / dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) / dx_sq;

            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            let next_u = u_curr + dt * (self.d_u * lap_u + reaction_u);
            let next_v = v_curr + dt * (self.d_v * lap_v + reaction_v);
            (next_u, next_v)
        };

        // 1. Handle i = 0
        {
            let i = 0;
            let u_curr = self.u[i];
            let v_curr = self.v[i];
            // idx_prev = 0, idx_next = 1 (or 0 if n=1)
            let u_prev = u_curr;
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                (self.u[1], self.v[1])
            } else {
                (u_curr, v_curr)
            };

            let (res_u, res_v) = calculate_update(u_curr, v_curr, u_prev, u_next, v_prev, v_next);
            self.buffer_u[i] = res_u;
            self.buffer_v[i] = res_v;
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            for i in 1..n-1 {
                // Safety: i-1 and i+1 are valid
                let u_curr = self.u[i];
                let v_curr = self.v[i];
                let u_prev = self.u[i-1];
                let u_next = self.u[i+1];
                let v_prev = self.v[i-1];
                let v_next = self.v[i+1];

                let (res_u, res_v) = calculate_update(u_curr, v_curr, u_prev, u_next, v_prev, v_next);
                self.buffer_u[i] = res_u;
                self.buffer_v[i] = res_v;
            }
        }

        // 3. Handle i = n-1
        if n > 1 {
            let i = n - 1;
            let u_curr = self.u[i];
            let v_curr = self.v[i];
            let u_prev = self.u[i-1];
            let v_prev = self.v[i-1];
            let u_next = u_curr; // idx_next = n-1
            let v_next = v_curr;

            let (res_u, res_v) = calculate_update(u_curr, v_curr, u_prev, u_next, v_prev, v_next);
            self.buffer_u[i] = res_u;
            self.buffer_v[i] = res_v;
        }

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.buffer_u);
        std::mem::swap(&mut self.v, &mut self.buffer_v);
    }
}
