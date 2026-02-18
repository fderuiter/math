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

use crate::biology::diffusion::FiniteDifference1D;
use crate::biology::reaction_diffusion::{
    ChemicalState, DiffusionModel, ReactionDiffusionSystem, ReactionModel,
};
use crate::pure_math::analysis::ode::solvers::Euler;
use crate::pure_math::analysis::ode::traits::Solver;

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

/// Blanket implementation of `ReactionModel` for any type that implements `ReactionKinetics`.
/// This adapts the 2-variable `reaction` method to the N-variable `ReactionModel` trait.
impl<T: ReactionKinetics> ReactionModel for T {
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

    fn add_reaction_batch(&self, concentrations: &[Vec<f64>], rates: &mut [Vec<f64>]) {
        if concentrations.len() < 2 || rates.len() < 2 {
            return;
        }

        let u_vec = &concentrations[0];
        let v_vec = &concentrations[1];

        // Split mutable borrow to access both rate vectors simultaneously
        let (left, right) = rates.split_at_mut(1);
        let rates_u = &mut left[0];
        let rates_v = &mut right[0];

        let n = u_vec
            .len()
            .min(v_vec.len())
            .min(rates_u.len())
            .min(rates_v.len());

        // Vectorized loop: Access memory linearly, enabling prefetch and SIMD
        for i in 0..n {
            let (du, dv) = <Self as ReactionKinetics>::reaction(self, u_vec[i], v_vec[i]);
            rates_u[i] += du;
            rates_v[i] += dv;
        }
    }
}

/// Represents a Reaction-Diffusion system specialized for Turing patterns.
///
/// This is a wrapper around the generic `ReactionDiffusionSystem`, pre-configured for 2 species.
pub struct TuringSystem<
    K: ReactionModel = SchnakenbergKinetics,
    D: DiffusionModel = FiniteDifference1D,
    S: Solver<ChemicalState> = Euler<ChemicalState>,
> {
    /// The underlying generic reaction-diffusion system.
    pub inner: ReactionDiffusionSystem<K, D, S>,
}

impl TuringSystem<SchnakenbergKinetics, FiniteDifference1D, Euler<ChemicalState>> {
    /// Creates a new Turing System with default Schnakenberg kinetics and 1D Finite Difference.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        let kinetics = SchnakenbergKinetics::default();
        let diffusion = FiniteDifference1D::new(dx);
        let diffusion_coeffs = vec![d_u, d_v];

        let inner = ReactionDiffusionSystem::new(2, size, kinetics, diffusion, diffusion_coeffs);

        Self { inner }
    }
}

impl<K: ReactionModel, D: DiffusionModel, S: Solver<ChemicalState>> TuringSystem<K, D, S> {
    /// Creates a new Turing System with custom kinetics, diffusion strategy, and solver.
    pub fn new_with_solver(
        size: usize,
        d_u: f64,
        d_v: f64,
        kinetics: K,
        diffusion: D,
        solver: S,
    ) -> Self {
        let diffusion_coeffs = vec![d_u, d_v];
        let inner = ReactionDiffusionSystem::new_with_solver(
            2,
            size,
            kinetics,
            diffusion,
            diffusion_coeffs,
            solver,
        );
        Self { inner }
    }
}

impl<K: ReactionModel, D: DiffusionModel> TuringSystem<K, D, Euler<ChemicalState>> {
    /// Creates a new Turing System with custom kinetics and diffusion strategy.
    ///
    /// Uses the default Euler solver.
    pub fn new_with_kinetics(size: usize, d_u: f64, d_v: f64, kinetics: K, diffusion: D) -> Self {
        let diffusion_coeffs = vec![d_u, d_v];
        let inner = ReactionDiffusionSystem::new(2, size, kinetics, diffusion, diffusion_coeffs);
        Self { inner }
    }
}

impl<K: ReactionModel, D: DiffusionModel, S: Solver<ChemicalState>> TuringSystem<K, D, S> {
    /// Accessor for the activator concentrations (backward compatibility/convenience).
    pub fn u(&self) -> &[f64] {
        self.inner.state.species(0)
    }

    /// Accessor for the inhibitor concentrations (backward compatibility/convenience).
    pub fn v(&self) -> &[f64] {
        self.inner.state.species(1)
    }

    /// Mutable accessor for the activator concentrations.
    pub fn u_mut(&mut self) -> &mut [f64] {
        self.inner.state.species_mut(0)
    }

    /// Mutable accessor for the inhibitor concentrations.
    pub fn v_mut(&mut self) -> &mut [f64] {
        self.inner.state.species_mut(1)
    }

    /// Updates the grid using the configured solver.
    pub fn step(&mut self, dt: f64) {
        self.inner.step(dt);
    }

    /// Returns the length of the grid.
    pub fn len(&self) -> usize {
        self.inner.state.grid_size()
    }

    /// Returns true if the grid is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.state.grid_size() == 0
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
            system.u_mut()[i] = 1.0 + 0.1 * (i as f64);
            system.v_mut()[i] = 0.5 - 0.05 * (i as f64);
        }

        // Run for a few steps
        let dt = 0.01;
        for _ in 0..5 {
            system.step(dt);
        }

        // Capture output
        let u_out = system.u().to_vec();
        let v_out = system.v().to_vec();

        // Expected values captured from baseline run (preserved from original test)
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
