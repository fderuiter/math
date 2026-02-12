//! # Morphogenesis (Turing Patterns)
//!
//! This module implements a Reaction-Diffusion system capable of generating Turing patterns.
//! It uses a 1D grid to simulate the interaction between an activator ($u$) and an inhibitor ($v$).
//!
//! Turing patterns arise when a stable uniform state becomes unstable due to diffusion (Diffusion-driven instability),
//! typically when the inhibitor diffuses much faster than the activator ($D_v \gg D_u$).
//!
//! ## 🔄 The Mechanism
//!
//! ```mermaid
//! graph TD
//!     subgraph "Local Reaction"
//!     A[Activator U] -->|Self-Catalysis| A
//!     A -->|Activates| B[Inhibitor V]
//!     B -->|Inhibits| A
//!     end
//!
//!     subgraph "Spatial Diffusion"
//!     DiffA[Diffusion of U]
//!     DiffB[Diffusion of V]
//!     end
//!
//!     A --- DiffA
//!     B --- DiffB
//!
//!     DiffA -->|Short Range| Patterns
//!     DiffB -->|Long Range| Patterns
//!
//!     style A fill:#a5d6a7,stroke:#2e7d32
//!     style B fill:#ef9a9a,stroke:#c62828
//! ```
//!
//! ## 🚀 Quick Start
//!
//! Simulate the emergence of a pattern from random noise.
//!
//! ```rust
//! use math_explorer::biology::morphogenesis::{TuringSystem, SchnakenbergKinetics};
//!
//! // 1. System Configuration
//! // Activator diffuses slowly (1.0), Inhibitor diffuses fast (40.0)
//! let n = 100;
//! let mut system = TuringSystem::new(n, 1.0, 40.0, 1.0);
//!
//! // 2. Initialize with Random Noise
//! // A uniform state would be stable without noise
//! for i in 0..n {
//!     system.u_mut()[i] = 1.0 + (i as f64 * 0.01).sin(); // Add some perturbation
//!     system.v_mut()[i] = 0.5 + (i as f64 * 0.02).cos();
//! }
//!
//! // 3. Run Simulation
//! let dt = 0.01;
//! for _ in 0..100 {
//!     system.step(dt);
//! }
//!
//! // 4. Analyze Results
//! let u_center = system.u()[50];
//! println!("Concentration of Activator at center: {:.4}", u_center);
//! ```

use crate::biology::diffusion::{FiniteDifference1D, SpatialDiffusion};
use crate::biology::reaction_diffusion::ReactionModel;
use crate::pure_math::analysis::ode::{OdeSystem, TimeStepper, VectorOperations};
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
/// This model is famous for generating spot-like patterns (like leopard spots).
///
/// ## Equations
///
/// $$ \frac{du}{dt} = a - u + u^2 v $$
/// $$ \frac{dv}{dt} = b - u^2 v $$
///
/// Where:
/// - $a$: Production rate of the activator.
/// - $b$: Production rate of the inhibitor.
/// - $u^2 v$: Non-linear autocatalysis term (Activator requires Inhibitor to grow, but consumes it).
#[derive(Debug, Clone, Copy)]
pub struct SchnakenbergKinetics {
    /// Production rate of activator ($a$).
    pub a: f64,
    /// Production rate of inhibitor ($b$).
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

impl ReactionModel for SchnakenbergKinetics {
    fn reaction(&self, concentrations: &[f64], rates: &mut [f64]) {
        if concentrations.len() < 2 || rates.len() < 2 {
            return;
        }
        let u = concentrations[0];
        let v = concentrations[1];
        let (du, dv) = <Self as ReactionKinetics>::reaction(self, u, v);
        rates[0] = du;
        rates[1] = dv;
    }
}

/// Represents the state of a Turing system at a point in time.
///
/// This struct encapsulates the concentration vectors for the activator and inhibitor,
/// protecting them from invalid resizing while providing safe access.
#[derive(Debug, Clone, PartialEq)]
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
        for (u, r) in self.u.iter_mut().zip(rhs.u.iter()) {
            *u += r;
        }
        for (v, r) in self.v.iter_mut().zip(rhs.v.iter()) {
            *v += r;
        }
        self
    }
}

impl AddAssign for TuringState {
    fn add_assign(&mut self, rhs: Self) {
        for (u, r) in self.u.iter_mut().zip(rhs.u.iter()) {
            *u += r;
        }
        for (v, r) in self.v.iter_mut().zip(rhs.v.iter()) {
            *v += r;
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
        for (u, r) in self.u.iter_mut().zip(other.u.iter()) {
            *u += r * scale;
        }
        for (v, r) in self.v.iter_mut().zip(other.v.iter()) {
            *v += r * scale;
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

/// Represents a Reaction-Diffusion system.
pub struct TuringSystem<
    K: ReactionKinetics = SchnakenbergKinetics,
    D: SpatialDiffusion = FiniteDifference1D,
> {
    /// The current state of the system.
    pub state: TuringState,

    // Double buffer for the next state.
    next_state: TuringState,

    /// Diffusion coefficient for u
    pub d_u: f64,
    /// Diffusion coefficient for v
    pub d_v: f64,
    /// Reaction kinetics strategy
    pub kinetics: K,
    /// Spatial diffusion strategy
    pub diffusion: D,
}

impl TuringSystem<SchnakenbergKinetics, FiniteDifference1D> {
    /// Creates a new Turing System with default Schnakenberg kinetics and 1D Finite Difference.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self {
            state: TuringState::new(size),
            next_state: TuringState::new(size),
            d_u,
            d_v,
            kinetics: SchnakenbergKinetics::default(),
            diffusion: FiniteDifference1D::new(dx),
        }
    }
}

impl<K: ReactionKinetics, D: SpatialDiffusion> TuringSystem<K, D> {
    /// Creates a new Turing System with custom kinetics and diffusion strategy.
    pub fn new_with_kinetics(size: usize, d_u: f64, d_v: f64, kinetics: K, diffusion: D) -> Self {
        Self {
            state: TuringState::new(size),
            next_state: TuringState::new(size),
            d_u,
            d_v,
            kinetics,
            diffusion,
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

    /// Updates the grid using the diffusion strategy and reaction kinetics.
    pub fn step(&mut self, dt: f64) {
        let n = self.state.len();
        if n == 0 {
            return;
        }

        // Ensure buffers are the right size
        if self.next_state.len() != n {
            self.next_state = TuringState::new(n);
        }

        let u = &self.state.u;
        let v = &self.state.v;
        let next_u = &mut self.next_state.u;
        let next_v = &mut self.next_state.v;

        // Fused Diffusion-Reaction-Integration Step
        // This is significantly faster than separate passes because it keeps data in registers/L1 cache.
        self.diffusion
            .apply_step(u, v, next_u, next_v, self.d_u, self.d_v, dt, |u, v| {
                self.kinetics.reaction(u, v)
            });

        // Swap buffers (states)
        std::mem::swap(&mut self.state, &mut self.next_state);
    }
}

impl<K: ReactionKinetics, D: SpatialDiffusion> OdeSystem<TuringState> for TuringSystem<K, D> {
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

        // Ensure output buffer is the right size
        if out.len() != n {
            *out = TuringState::new(n);
        }

        let u = &state.u;
        let v = &state.v;
        let out_u = &mut out.u;
        let out_v = &mut out.v;

        // 1. Compute Diffusion
        self.diffusion.apply(u, v, out_u, out_v, self.d_u, self.d_v);

        // 2. Compute Reaction and Accumulate
        unsafe {
            for i in 0..n {
                let u_curr = *u.get_unchecked(i);
                let v_curr = *v.get_unchecked(i);

                let (reac_u, reac_v) = self.kinetics.reaction(u_curr, v_curr);

                *out_u.get_unchecked_mut(i) += reac_u;
                *out_v.get_unchecked_mut(i) += reac_v;
            }
        }
    }
}

impl<K: ReactionKinetics, D: SpatialDiffusion> TimeStepper<TuringState> for TuringSystem<K, D> {
    fn get_state(&self) -> &TuringState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut TuringState {
        &mut self.state
    }

    fn step(&mut self, dt: f64) {
        // Delegate to the optimized inherent method
        self.step(dt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turing_system_logic_preservation() {
        // Setup a small system
        let n = 10;
        let d_u = 1.0;
        let d_v = 0.5;
        let dx = 1.0;
        let mut system = TuringSystem::new(n, d_u, d_v, dx);

        // Initialize with some pattern
        for i in 0..n {
            system.state.u_mut()[i] = 1.0 + 0.1 * (i as f64);
            system.state.v_mut()[i] = 0.5 - 0.05 * (i as f64);
        }

        // Run for a few steps
        let dt = 0.01;
        for _ in 0..5 {
            system.step(dt);
        }

        // Capture output
        let u_out = system.u().to_vec();
        let v_out = system.v().to_vec();

        // Expected values captured from baseline run
        let expected_u = vec![
            0.9798926377401955,
            1.0722504645444493,
            1.1685990805783317,
            1.2642647090938448,
            1.359028327357602,
            1.4527705800845148,
            1.5453811790730032,
            1.6367576303434268,
            1.7267186541725483,
            1.8109737170223916,
        ];
        let expected_v = vec![
            0.47709091921002866,
            0.4263770084483741,
            0.3750152156844884,
            0.32443296992262166,
            0.2747722006954079,
            0.22615405798594523,
            0.1786914141249832,
            0.1324883911255509,
            0.08765106523936222,
            0.04531981611374585,
        ];

        // Assert with tolerance
        let tolerance = 1e-10;
        for i in 0..n {
            assert!(
                (u_out[i] - expected_u[i]).abs() < tolerance,
                "U mismatch at {}: {} vs {}",
                i,
                u_out[i],
                expected_u[i]
            );
            assert!(
                (v_out[i] - expected_v[i]).abs() < tolerance,
                "V mismatch at {}: {} vs {}",
                i,
                v_out[i],
                expected_v[i]
            );
        }
    }
}
