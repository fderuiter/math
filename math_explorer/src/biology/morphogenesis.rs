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
}

impl TuringSystem {
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            d_u,
            d_v,
            dx,
        }
    }

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    /// Using Gierer-Meinhardt-like kinetics as suggested:
    /// f(u,v) = a - u + u^2 v
    pub fn step(&mut self, dt: f64) {
        let n = self.u.len();
        let mut new_u = self.u.clone();
        let mut new_v = self.v.clone();

        let a = 0.01; // Feed rate / constant
        let b = 0.05; // Another constant
        // Using Schnakenberg-like kinetics for demonstration if not strictly specified beyond "f(u,v) = ..."

        for i in 0..n {
            // Laplacian with periodic boundary or zero-flux?
            // Zero flux (Neumann) is safer for 1D patterns usually, or periodic.
            // Using simple indices with clamping for Neumann-ish or just simple handling.
            let u_curr = self.u[i];
            let v_curr = self.v[i];

            let idx_prev = if i == 0 { 0 } else { i - 1 }; // Zero flux approx (u_-1 = u_0) -> deriv is 0
            let idx_next = if i == n - 1 { n - 1 } else { i + 1 };

            // Laplacian: (u_{i+1} - 2u_i + u_{i-1}) / dx^2
            // If i=0, u_{i-1} is u_0 -> (u_1 - 2u_0 + u_0) = u_1 - u_0.
            // This corresponds to forward difference at boundary, effectively zero flux if we consider ghost points.
            // Standard 3-point stencil.
            let lap_u = (self.u[idx_next] - 2.0 * u_curr + self.u[idx_prev]) / (self.dx * self.dx);
            let lap_v = (self.v[idx_next] - 2.0 * v_curr + self.v[idx_prev]) / (self.dx * self.dx);

            // Reaction terms
            // u_t = ... + a - u + u^2 v
            // v_t = ... + b - u^2 v (Schnakenberg)
            let reaction_u = a - u_curr + u_curr.powi(2) * v_curr;
            let reaction_v = b - u_curr.powi(2) * v_curr;

            new_u[i] = u_curr + dt * (self.d_u * lap_u + reaction_u);
            new_v[i] = v_curr + dt * (self.d_v * lap_v + reaction_v);
        }

        self.u = new_u;
        self.v = new_v;
    }
}
