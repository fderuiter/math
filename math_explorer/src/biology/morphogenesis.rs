//! Morphogenesis (Turing Patterns)
//!
//! This module implements a **Reaction-Diffusion system** capable of generating Turing patterns.
//! It simulates the interaction between two chemical substances: an **Activator** ($u$) and an **Inhibitor** ($v$)
//! on a 1D grid.
//!
//! > **"How does the leopard get its spots?"** — Alan Turing
//!
//! ## The Math
//!
//! The system evolves according to:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D_u \nabla^2 \mathbf{u} + f(u, v) $$
//! $$ \frac{\partial \mathbf{v}}{\partial t} = D_v \nabla^2 \mathbf{v} + g(u, v) $$
//!
//! Where diffusion drives spreading ($D \nabla^2$) and reaction kinetics ($f, g$) drive local interaction.
//! We use **Schnakenberg Kinetics** by default:
//!
//! *   Activator production: $f(u, v) = a - u + u^2 v$
//! *   Inhibitor production: $g(u, v) = b - u^2 v$
//!
//! ## 🚀 Quick Start
//!
//! ```rust
//! use math_explorer::biology::morphogenesis::{TuringSystem, SchnakenbergKinetics};
//!
//! fn main() {
//!     // 1. Initialize System
//!     // Grid size=100, Du=1.0, Dv=20.0 (Inhibitor diffuses faster), dx=1.0
//!     let mut system = TuringSystem::new(100, 1.0, 20.0, 1.0);
//!
//!     // 2. Perturb Initial State
//!     // Turing instabilities require small random noise to break symmetry.
//!     for i in 0..100 {
//!         // Small random perturbation around the steady state
//!         // (In a real app, use `rand::Rng`)
//!         let noise = (i as f64).sin() * 0.1;
//!         system.u[i] = 1.0 + noise;
//!         system.v[i] = 0.9 + noise;
//!     }
//!
//!     // 3. Evolve System
//!     // Simulate for 1000 time steps
//!     let dt = 0.01;
//!     for _ in 0..1000 {
//!         system.step(dt);
//!     }
//!
//!     // 4. Observe Pattern
//!     // In a Turing pattern, concentration levels should vary across the grid.
//!     let u_variance: f64 = system.u.iter()
//!         .map(|val| (val - 1.0).powi(2))
//!         .sum();
//!
//!     println!("Pattern Variance: {:.4}", u_variance);
//!     assert!(u_variance > 0.1, "Pattern should have formed!");
//! }
//! ```

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
        let uv_sq = u.powi(2) * v;
        let reaction_u = self.a - u + uv_sq;
        let reaction_v = self.b - uv_sq;
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

    // Private buffers for double buffering to avoid allocation in hot loops.
    // Marked hidden to discourage manual usage if fields were public (which they are).
    #[doc(hidden)]
    buffer_u: Vec<f64>,
    #[doc(hidden)]
    buffer_v: Vec<f64>,
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
        if n == 0 {
            return;
        }

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
        let calculate_update = |u_curr: f64,
                                v_curr: f64,
                                u_prev: f64,
                                u_next: f64,
                                v_prev: f64,
                                v_next: f64|
         -> (f64, f64) {
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
            for i in 1..n - 1 {
                // Safety: i-1 and i+1 are valid
                let u_curr = self.u[i];
                let v_curr = self.v[i];
                let u_prev = self.u[i - 1];
                let u_next = self.u[i + 1];
                let v_prev = self.v[i - 1];
                let v_next = self.v[i + 1];

                let (res_u, res_v) =
                    calculate_update(u_curr, v_curr, u_prev, u_next, v_prev, v_next);
                self.buffer_u[i] = res_u;
                self.buffer_v[i] = res_v;
            }
        }

        // 3. Handle i = n-1
        if n > 1 {
            let i = n - 1;
            let u_curr = self.u[i];
            let v_curr = self.v[i];
            let u_prev = self.u[i - 1];
            let v_prev = self.v[i - 1];
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
