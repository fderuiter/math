//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

/// Represents a 1D Reaction-Diffusion system.
pub struct TuringSystem {
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

    // Double buffering scratchpads
    next_u: Vec<f64>,
    next_v: Vec<f64>,
}

impl TuringSystem {
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            d_u,
            d_v,
            dx,
            next_u: vec![0.0; size],
            next_v: vec![0.0; size],
        }
    }

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    /// Using Gierer-Meinhardt-like kinetics as suggested:
    /// f(u,v) = a - u + u^2 v
    pub fn step(&mut self, dt: f64) {
        let n = self.u.len();
        if n == 0 { return; }

        // Ensure buffers are correct size
        if self.next_u.len() != n { self.next_u.resize(n, 0.0); }
        if self.next_v.len() != n { self.next_v.resize(n, 0.0); }

        // Constants
        let a = 0.01;
        let b = 0.05;
        let inv_dx2 = 1.0 / (self.dx * self.dx);
        let d_u = self.d_u;
        let d_v = self.d_v;

        // Reaction-Diffusion update function
        // Inline ensures no function call overhead
        let update_cell = |i: usize, u_curr: f64, v_curr: f64, u_prev: f64, v_prev: f64, u_next: f64, v_next: f64, next_u: &mut [f64], next_v: &mut [f64]| {
            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx2;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx2;

            let uv2 = u_curr * u_curr * v_curr;
            let reaction_u = a - u_curr + uv2;
            let reaction_v = b - uv2;

            next_u[i] = u_curr + dt * (d_u * lap_u + reaction_u);
            next_v[i] = v_curr + dt * (d_v * lap_v + reaction_v);
        };

        if n == 1 {
            // Special case for n=1: u_prev=u[0], u_next=u[0] (Neumann)
            let u0 = self.u[0];
            let v0 = self.v[0];
            update_cell(0, u0, v0, u0, v0, u0, v0, &mut self.next_u, &mut self.next_v);
        } else {
            // General case n >= 2

            // Boundary i = 0
            {
                let u0 = self.u[0];
                let v0 = self.v[0];
                let u1 = self.u[1];
                let v1 = self.v[1];
                // u_prev = u0, u_next = u1
                update_cell(0, u0, v0, u0, v0, u1, v1, &mut self.next_u, &mut self.next_v);
            }

            // Inner loop 1..n-1
            for i in 1..n-1 {
                let u_curr = self.u[i];
                let v_curr = self.v[i];
                let u_prev = self.u[i - 1];
                let v_prev = self.v[i - 1];
                let u_next = self.u[i + 1];
                let v_next = self.v[i + 1];
                update_cell(i, u_curr, v_curr, u_prev, v_prev, u_next, v_next, &mut self.next_u, &mut self.next_v);
            }

            // Boundary i = n-1
            {
                let i = n - 1;
                let u_curr = self.u[i];
                let v_curr = self.v[i];
                let u_prev = self.u[i - 1];
                let v_prev = self.v[i - 1];
                // u_next = u_curr
                update_cell(i, u_curr, v_curr, u_prev, v_prev, u_curr, v_curr, &mut self.next_u, &mut self.next_v);
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }
}
