//! Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! The general equation is:
//! $$ \frac{\partial \mathbf{u}}{\partial t} = D \nabla^2 \mathbf{u} + \mathbf{f}(\mathbf{u}) $$

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
        let inv_dx_sq = 1.0 / dx_sq;

        // Optimization: Lift boundary checks out of the loop and use slices
        let u = &self.u;
        let v = &self.v;
        let buffer_u = &mut self.buffer_u;
        let buffer_v = &mut self.buffer_v;

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

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

            unsafe {
                *buffer_u.get_unchecked_mut(i) = u_curr + dt * (self.d_u * lap_u + reaction_u);
                *buffer_v.get_unchecked_mut(i) = v_curr + dt * (self.d_v * lap_v + reaction_v);
            }
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            for i in 1..n - 1 {
                // Safety: loop bounds ensure i, i-1, i+1 are valid
                unsafe {
                    let u_curr = *u.get_unchecked(i);
                    let v_curr = *v.get_unchecked(i);
                    let u_prev = *u.get_unchecked(i - 1);
                    let u_next = *u.get_unchecked(i + 1);
                    let v_prev = *v.get_unchecked(i - 1);
                    let v_next = *v.get_unchecked(i + 1);

                    let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                    let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                    let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

                    *buffer_u.get_unchecked_mut(i) = u_curr + dt * (self.d_u * lap_u + reaction_u);
                    *buffer_v.get_unchecked_mut(i) = v_curr + dt * (self.d_v * lap_v + reaction_v);
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

                let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                let (reaction_u, reaction_v) = self.kinetics.reaction(u_curr, v_curr);

                *buffer_u.get_unchecked_mut(i) = u_curr + dt * (self.d_u * lap_u + reaction_u);
                *buffer_v.get_unchecked_mut(i) = v_curr + dt * (self.d_v * lap_v + reaction_v);
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.buffer_u);
        std::mem::swap(&mut self.v, &mut self.buffer_v);
    }
}
