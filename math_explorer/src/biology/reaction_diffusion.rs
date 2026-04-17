//! # Generic Reaction-Diffusion System
//!
//! This module provides a flexible framework for simulating N-species reaction-diffusion systems.
//!
//! ## Why does this exist?
//!
//! While specific implementations like `TuringSystem` are useful, biological modeling often requires
//! testing novel hypotheses. This module exists to decouple the core mechanics of a simulation
//! (the mathematical integration and spatial grids) from the domain-specific biology (the chemical reactions).
//! By enforcing the Strategy Pattern, researchers can swap solvers, dimensions (1D vs 2D), or kinetics
//! without touching the underlying engine.
//!
//! ## Architecture
//!
//! ```mermaid
//! classDiagram
//!     class ReactionDiffusionSystem {
//!         +model: ReactionDiffusionModel
//!         +state: ChemicalState
//!         +solver: Solver
//!         +step(dt)
//!     }
//!
//!     class ReactionDiffusionModel {
//!         +reaction: ReactionModel
//!         +diffusion: DiffusionModel
//!         +diffusion_coeffs: Vec~f64~
//!     }
//!
//!     class ReactionModel {
//!         <<Trait>>
//!         +reaction()
//!     }
//!
//!     class DiffusionModel {
//!         <<Trait>>
//!         +apply()
//!     }
//!
//!     ReactionDiffusionSystem o-- ReactionDiffusionModel
//!     ReactionDiffusionModel o-- ReactionModel
//!     ReactionDiffusionModel o-- DiffusionModel
//! ```
//!
//! ## Example
//!
//! Implementing a simple decay system using the generic framework:
//!
//! ```rust
//! use math_explorer::biology::reaction_diffusion::{ReactionDiffusionSystem, ReactionModel, ChemicalState};
//! use math_explorer::biology::diffusion::FiniteDifference1D;
//!
//! // 1. Define custom kinetics (e.g., simple exponential decay)
//! struct DecayKinetics { rate: f64 }
//! impl ReactionModel for DecayKinetics {
//!     fn reaction(&self, concs: &[f64], rates: &mut [f64]) {
//!         rates[0] = -self.rate * concs[0];
//!     }
//! }
//!
//! // 2. Setup the generic system (1 species, 10 grid points)
//! let kinetics = DecayKinetics { rate: 0.1 };
//! let diffusion = FiniteDifference1D::new(1.0);
//! let mut system = ReactionDiffusionSystem::new(1, 10, kinetics, diffusion, vec![0.5]);
//!
//! // 3. Initialize state and run
//! system.state.species_mut(0)[5] = 100.0; // Spike in the middle
//! system.step(0.1);
//!
//! assert!(system.state.species(0)[5] < 100.0); // Concentration decayed
//! ```

use crate::pure_math::analysis::ode::solvers::Euler;
use crate::pure_math::analysis::ode::traits::{OdeSystem, Solver, VectorOperations};
use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Represents the state of a multi-species chemical system.
///
/// Stores concentrations in a flattened "Structure of Arrays" format: `Vec<f64>`
/// to ensure contiguous memory allocation and zero double-indirection overhead.
/// The layout is `[Species 0 (0..N), Species 1 (0..N), ...]`.
#[derive(Debug, Clone, PartialEq)]
pub struct ChemicalState {
    num_species: usize,
    grid_size: usize,
    /// Flattened concentrations of each species across the spatial grid.
    pub concentrations: Vec<f64>,
}

impl ChemicalState {
    /// Creates a new zero-initialized chemical state.
    pub fn new(num_species: usize, grid_size: usize) -> Self {
        Self {
            num_species,
            grid_size,
            concentrations: vec![0.0; num_species * grid_size],
        }
    }

    /// Returns the number of chemical species.
    #[inline]
    pub fn num_species(&self) -> usize {
        self.num_species
    }

    /// Returns the size of the spatial grid.
    #[inline]
    pub fn grid_size(&self) -> usize {
        self.grid_size
    }

    /// Returns a reference to the concentration slice for a specific species.
    #[inline]
    pub fn species(&self, index: usize) -> &[f64] {
        let start = index * self.grid_size;
        &self.concentrations[start..start + self.grid_size]
    }

    /// Returns a mutable reference to the concentration slice for a specific species.
    #[inline]
    pub fn species_mut(&mut self, index: usize) -> &mut [f64] {
        let start = index * self.grid_size;
        &mut self.concentrations[start..start + self.grid_size]
    }
}

// Implement standard ops for ODE integration compatibility
impl Add for ChemicalState {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self {
        for (val, r_val) in self
            .concentrations
            .iter_mut()
            .zip(rhs.concentrations.iter())
        {
            *val += r_val;
        }
        self
    }
}

impl AddAssign for ChemicalState {
    fn add_assign(&mut self, rhs: Self) {
        for (val, r_val) in self
            .concentrations
            .iter_mut()
            .zip(rhs.concentrations.iter())
        {
            *val += r_val;
        }
    }
}

impl Mul<f64> for ChemicalState {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self {
        for val in self.concentrations.iter_mut() {
            *val *= scalar;
        }
        self
    }
}

impl MulAssign<f64> for ChemicalState {
    fn mul_assign(&mut self, scalar: f64) {
        for val in self.concentrations.iter_mut() {
            *val *= scalar;
        }
    }
}

impl VectorOperations for ChemicalState {
    fn scale_add(&mut self, other: &Self, scale: f64) {
        for (val, r_val) in self
            .concentrations
            .iter_mut()
            .zip(other.concentrations.iter())
        {
            *val += r_val * scale;
        }
    }

    fn copy_from(&mut self, other: &Self) {
        if self.num_species != other.num_species || self.grid_size != other.grid_size {
            // Reallocate if dimensions mismatch
            self.concentrations = other.concentrations.clone();
            self.num_species = other.num_species;
            self.grid_size = other.grid_size;
            return;
        }
        self.concentrations.copy_from_slice(&other.concentrations);
    }

    fn copy_from_scaled(&mut self, source: &Self, other: &Self, scale: f64) {
        if self.num_species != source.num_species || self.grid_size != source.grid_size {
            // Reallocate if dimensions mismatch.
            self.concentrations = vec![0.0; source.concentrations.len()];
            self.num_species = source.num_species;
            self.grid_size = source.grid_size;
        }

        // Fused 1D loop: self = source + other * scale (highly vectorizable)
        for ((dst, src), oth) in self
            .concentrations
            .iter_mut()
            .zip(source.concentrations.iter())
            .zip(other.concentrations.iter())
        {
            *dst = *src + *oth * scale;
        }
    }
}

/// Defines the local reaction kinetics for N species.
pub trait ReactionModel {
    /// Computes the reaction rates for a single spatial point.
    ///
    /// # Arguments
    /// * `concentrations`: The current concentrations of all species at this point.
    /// * `rates`: Output buffer for the computed reaction rates (dC/dt).
    fn reaction(&self, concentrations: &[f64], rates: &mut [f64]);

    /// Computes and accumulates reaction rates for the entire grid.
    ///
    /// This method allows for vectorized implementations that process multiple grid points
    /// efficiently. The default implementation iterates over grid points and calls `reaction`.
    ///
    /// # Arguments
    /// * `state`: Current chemical state (concentrations).
    /// * `out_rates`: Chemical state buffer to accumulate reaction rates into.
    fn add_reaction_batch(&self, state: &ChemicalState, out_rates: &mut ChemicalState) {
        let n_species = state.num_species();
        if n_species == 0 {
            return;
        }
        let n_grid = state.grid_size();

        let mut local_concs = vec![0.0; n_species];
        let mut local_rates = vec![0.0; n_species];

        for i in 0..n_grid {
            // Gather
            for (s, conc) in local_concs.iter_mut().enumerate().take(n_species) {
                *conc = state.species(s)[i];
            }

            self.reaction(&local_concs, &mut local_rates);

            // Scatter (Accumulate)
            for (s, rate) in local_rates.iter().enumerate().take(n_species) {
                out_rates.species_mut(s)[i] += rate;
            }
        }
    }
}

/// Defines the spatial diffusion strategy for N species.
pub trait DiffusionModel {
    /// Applies the diffusion operator (Laplacian) to the full state.
    ///
    /// # Arguments
    /// * `state`: Current chemical state.
    /// * `out`: Output buffer for the diffusion term (D * Laplacian).
    /// * `coeffs`: Diffusion coefficients for each species.
    fn apply(&self, state: &ChemicalState, out: &mut ChemicalState, coeffs: &[f64]);
}

/// A pure physics definition of a Reaction-Diffusion system.
///
/// This struct encapsulates the immutable laws of the system (reaction kinetics, diffusion strategy,
/// and coefficients), separating them from the mutable simulation state.
///
/// It implements `OdeSystem` to calculate the time derivative ($dC/dt$) given a state.
pub struct ReactionDiffusionModel<R, D> {
    pub reaction: R,
    pub diffusion: D,
    pub diffusion_coeffs: Vec<f64>,
}

impl<R: ReactionModel, D: DiffusionModel> OdeSystem<ChemicalState>
    for ReactionDiffusionModel<R, D>
{
    fn derivative(&self, _t: f64, state: &ChemicalState) -> ChemicalState {
        let mut out = ChemicalState::new(state.num_species(), state.grid_size());
        self.derivative_in_place(_t, state, &mut out);
        out
    }

    fn derivative_in_place(&self, _t: f64, state: &ChemicalState, out: &mut ChemicalState) {
        // Compute Diffusion
        self.diffusion.apply(state, out, &self.diffusion_coeffs);

        // Add Reaction
        // Optimized: Use batch processing to allow vectorization and avoid gather/scatter
        self.reaction.add_reaction_batch(state, out);
    }
}

/// A generic Reaction-Diffusion system for N species.
///
/// This struct manages the simulation state and the integration strategy.
/// By default, it uses the Forward Euler method, but can be configured with other solvers.
pub struct ReactionDiffusionSystem<
    R: ReactionModel,
    D: DiffusionModel,
    S: Solver<ChemicalState> = Euler<ChemicalState>,
> {
    pub model: ReactionDiffusionModel<R, D>,
    pub state: ChemicalState,
    pub solver: S,
}

impl<R: ReactionModel, D: DiffusionModel> ReactionDiffusionSystem<R, D, Euler<ChemicalState>> {
    /// Creates a new Reaction-Diffusion system with the default Euler solver.
    pub fn new(
        num_species: usize,
        grid_size: usize,
        reaction: R,
        diffusion: D,
        diffusion_coeffs: Vec<f64>,
    ) -> Self {
        assert_eq!(diffusion_coeffs.len(), num_species);
        let state = ChemicalState::new(num_species, grid_size);
        let solver = Euler::new(&state);
        Self {
            model: ReactionDiffusionModel {
                reaction,
                diffusion,
                diffusion_coeffs,
            },
            state,
            solver,
        }
    }
}

impl<R: ReactionModel, D: DiffusionModel, S: Solver<ChemicalState>>
    ReactionDiffusionSystem<R, D, S>
{
    /// Creates a new Reaction-Diffusion system with a custom solver.
    pub fn new_with_solver(
        num_species: usize,
        grid_size: usize,
        reaction: R,
        diffusion: D,
        diffusion_coeffs: Vec<f64>,
        solver: S,
    ) -> Self {
        assert_eq!(diffusion_coeffs.len(), num_species);
        Self {
            model: ReactionDiffusionModel {
                reaction,
                diffusion,
                diffusion_coeffs,
            },
            state: ChemicalState::new(num_species, grid_size),
            solver,
        }
    }

    /// Advances the system by a time step `dt` using the configured solver.
    pub fn step(&mut self, dt: f64) {
        // The solver manages the integration logic.
        // We pass 0.0 as the current time since most RD systems are autonomous (time-invariant).
        self.solver.step(&self.model, 0.0, &mut self.state, dt);
    }

    /// Accessor for the reaction model.
    pub fn reaction(&self) -> &R {
        &self.model.reaction
    }

    /// Accessor for the diffusion model.
    pub fn diffusion(&self) -> &D {
        &self.model.diffusion
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::biology::diffusion::FiniteDifference1D;
    use crate::biology::morphogenesis::SchnakenbergKinetics;
    use crate::pure_math::analysis::ode::solvers::RungeKutta4;

    #[test]
    fn test_reaction_diffusion_system_rk4() {
        // Setup a small system with RK4 solver
        let n = 10;
        let d_u = 1.0;
        let d_v = 0.5;
        let dx = 1.0;

        let kinetics = SchnakenbergKinetics::default();
        let diffusion = FiniteDifference1D::new(dx);
        let diffusion_coeffs = vec![d_u, d_v];

        // Explicitly use RK4
        let dummy_state = ChemicalState::new(2, n);
        let solver = RungeKutta4::new(&dummy_state);

        let mut system = ReactionDiffusionSystem::new_with_solver(
            2,
            n,
            kinetics,
            diffusion,
            diffusion_coeffs,
            solver,
        );

        // Initialize with same pattern
        for i in 0..n {
            system.state.species_mut(0)[i] = 1.0 + 0.1 * (i as f64);
            system.state.species_mut(1)[i] = 0.5 - 0.05 * (i as f64);
        }

        // Run for a few steps
        let dt = 0.01;
        for _ in 0..5 {
            system.step(dt);
        }

        // Check values are reasonable (not NaN and changed)
        let u_val = system.state.species(0)[0];
        assert!(!u_val.is_nan());
        assert!((u_val - 1.0).abs() > 1e-3);
    }

    #[test]
    fn test_reaction_diffusion_system_equivalence() {
        // Setup a small system
        let n = 10;
        let d_u = 1.0;
        let d_v = 0.5;
        let dx = 1.0;

        let kinetics = SchnakenbergKinetics::default();
        let diffusion = FiniteDifference1D::new(dx);
        let diffusion_coeffs = vec![d_u, d_v];

        let mut system = ReactionDiffusionSystem::new(2, n, kinetics, diffusion, diffusion_coeffs);

        // Initialize with same pattern as in morphogenesis test
        // u = 1.0 + 0.1 * i
        // v = 0.5 - 0.05 * i
        for i in 0..n {
            system.state.species_mut(0)[i] = 1.0 + 0.1 * (i as f64);
            system.state.species_mut(1)[i] = 0.5 - 0.05 * (i as f64);
        }

        // Run for a few steps
        let dt = 0.01;
        for _ in 0..5 {
            system.step(dt);
        }

        // Capture output
        let u_out = system.state.species(0).to_vec();
        let v_out = system.state.species(1).to_vec();

        // Expected values (same as in morphogenesis.rs)
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
