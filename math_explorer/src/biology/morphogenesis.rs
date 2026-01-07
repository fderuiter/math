//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

/// Defines the reaction kinetics for the system.
/// This allows different chemical reaction models (e.g., Schnakenberg, Gray-Scott) to be plugged in.
pub trait ReactionKinetics {
    /// Calculates the reaction terms for activator u and inhibitor v.
    /// Returns (du/dt, dv/dt) contribution from reaction.
    fn reaction(&self, u: f64, v: f64) -> (f64, f64);
}

/// Default Schnakenberg kinetics:
/// u_t = a - u + u^2 v
/// v_t = b - u^2 v
#[derive(Clone, Copy, Debug)]
pub struct SchnakenbergKinetics {
    pub a: f64,
    pub b: f64,
}

impl Default for SchnakenbergKinetics {
    fn default() -> Self {
        Self { a: 0.01, b: 0.05 }
    }
}

impl ReactionKinetics for SchnakenbergKinetics {
    fn reaction(&self, u: f64, v: f64) -> (f64, f64) {
        let reaction_u = self.a - u + u.powi(2) * v;
        let reaction_v = self.b - u.powi(2) * v;
        (reaction_u, reaction_v)
    }
}

/// Represents a 1D Reaction-Diffusion system.
pub struct TuringSystem<K: ReactionKinetics = SchnakenbergKinetics> {
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
}

impl TuringSystem<SchnakenbergKinetics> {
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            d_u,
            d_v,
            dx,
            kinetics: SchnakenbergKinetics::default(),
        }
    }
}

impl<K: ReactionKinetics> TuringSystem<K> {
    pub fn new_with_kinetics(size: usize, d_u: f64, d_v: f64, dx: f64, kinetics: K) -> Self {
        Self {
            u: vec![0.0; size],
            v: vec![0.0; size],
            d_u,
            d_v,
            dx,
            kinetics,
        }
    }

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    pub fn step(&mut self, dt: f64) {
        let n = self.u.len();
        let mut new_u = self.u.clone();
        let mut new_v = self.v.clone();

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

            // Reaction terms calculated via strategy
            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            new_u[i] = u_curr + dt * (self.d_u * lap_u + reaction_u);
            new_v[i] = v_curr + dt * (self.d_v * lap_v + reaction_v);
        }

        self.u = new_u;
        self.v = new_v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turing_system_initialization() {
        let system = TuringSystem::new(100, 1.0, 0.5, 0.1);
        assert_eq!(system.u.len(), 100);
        assert_eq!(system.v.len(), 100);
        assert_eq!(system.d_u, 1.0);
        assert_eq!(system.d_v, 0.5);
    }

    #[test]
    fn test_turing_system_step() {
        let mut system = TuringSystem::new(10, 1.0, 1.0, 1.0);
        // Initialize with some values to ensure reaction terms do something
        for i in 0..10 {
            system.u[i] = 1.0;
            system.v[i] = 1.0;
        }

        system.step(0.1);

        // With u=1, v=1, a=0.01, b=0.05
        // reaction_u = 0.01 - 1 + 1*1 = 0.01
        // reaction_v = 0.05 - 1*1 = -0.95
        // laplacian is 0 because constant field
        // new_u = 1 + 0.1 * (0 + 0.01) = 1.001
        // new_v = 1 + 0.1 * (0 - 0.95) = 0.905

        assert!((system.u[5] - 1.001).abs() < 1e-6, "Expected u approx 1.001, got {}", system.u[5]);
        assert!((system.v[5] - 0.905).abs() < 1e-6, "Expected v approx 0.905, got {}", system.v[5]);
    }
}
