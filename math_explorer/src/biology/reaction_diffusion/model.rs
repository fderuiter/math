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
//! let mut system = ReactionDiffusionSystem::builder()
//!     .num_species(1)
//!     .grid_size(10)
//!     .reaction(kinetics)
//!     .diffusion(diffusion)
//!     .diffusion_coeffs(vec![0.5])
//!     .build()
//!     .unwrap();
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

/// Errors related to Reaction-Diffusion systems.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReactionDiffusionError {
    #[error("Dimension mismatch: expected {expected} diffusion coefficients, but got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    #[error("System requires at least one species")]
    ZeroSpecies,
    #[error("Grid size cannot be zero")]
    ZeroGridSize,
    #[error("Missing parameter: {0}")]
    MissingParameter(&'static str),
}

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

/// Builder for `ReactionDiffusionSystem`.
///
/// Ensures that parameters are physically valid before creating the system.
#[derive(Debug, Clone)]
pub struct ReactionDiffusionSystemBuilder<R, D, S> {
    num_species: Option<usize>,
    grid_size: Option<usize>,
    reaction: Option<R>,
    diffusion: Option<D>,
    diffusion_coeffs: Option<Vec<f64>>,
    solver: Option<S>,
}

impl<R, D, S> Default for ReactionDiffusionSystemBuilder<R, D, S> {
    fn default() -> Self {
        Self {
            num_species: None,
            grid_size: None,
            reaction: None,
            diffusion: None,
            diffusion_coeffs: None,
            solver: None,
        }
    }
}

impl<R: ReactionModel, D: DiffusionModel>
    ReactionDiffusionSystemBuilder<R, D, Euler<ChemicalState>>
{
    /// Starts a new builder with default type parameters.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<R: ReactionModel, D: DiffusionModel, S: Solver<ChemicalState>>
    ReactionDiffusionSystemBuilder<R, D, S>
{
    /// Sets the number of chemical species.
    pub fn num_species(mut self, num_species: usize) -> Self {
        self.num_species = Some(num_species);
        self
    }

    /// Sets the spatial grid size.
    pub fn grid_size(mut self, grid_size: usize) -> Self {
        self.grid_size = Some(grid_size);
        self
    }

    /// Sets the reaction kinetics model.
    pub fn reaction(mut self, reaction: R) -> Self {
        self.reaction = Some(reaction);
        self
    }

    /// Sets the spatial diffusion model.
    pub fn diffusion(mut self, diffusion: D) -> Self {
        self.diffusion = Some(diffusion);
        self
    }

    /// Sets the diffusion coefficients for each species.
    pub fn diffusion_coeffs(mut self, coeffs: Vec<f64>) -> Self {
        self.diffusion_coeffs = Some(coeffs);
        self
    }

    /// Sets a custom solver strategy.
    pub fn solver<NewS: Solver<ChemicalState>>(
        self,
        solver: NewS,
    ) -> ReactionDiffusionSystemBuilder<R, D, NewS> {
        ReactionDiffusionSystemBuilder {
            num_species: self.num_species,
            grid_size: self.grid_size,
            reaction: self.reaction,
            diffusion: self.diffusion,
            diffusion_coeffs: self.diffusion_coeffs,
            solver: Some(solver),
        }
    }
}

impl<R: ReactionModel, D: DiffusionModel>
    ReactionDiffusionSystemBuilder<R, D, Euler<ChemicalState>>
{
    /// Builds the `ReactionDiffusionSystem` with the default Euler solver.
    pub fn build(
        self,
    ) -> Result<ReactionDiffusionSystem<R, D, Euler<ChemicalState>>, ReactionDiffusionError> {
        let num_species = self
            .num_species
            .ok_or(ReactionDiffusionError::MissingParameter("num_species"))?;
        let grid_size = self
            .grid_size
            .ok_or(ReactionDiffusionError::MissingParameter("grid_size"))?;
        let reaction = self
            .reaction
            .ok_or(ReactionDiffusionError::MissingParameter("reaction"))?;
        let diffusion = self
            .diffusion
            .ok_or(ReactionDiffusionError::MissingParameter("diffusion"))?;
        let diffusion_coeffs = self
            .diffusion_coeffs
            .ok_or(ReactionDiffusionError::MissingParameter("diffusion_coeffs"))?;

        if num_species == 0 {
            return Err(ReactionDiffusionError::ZeroSpecies);
        }

        if grid_size == 0 {
            return Err(ReactionDiffusionError::ZeroGridSize);
        }

        if diffusion_coeffs.len() != num_species {
            return Err(ReactionDiffusionError::DimensionMismatch {
                expected: num_species,
                got: diffusion_coeffs.len(),
            });
        }

        let state = ChemicalState::new(num_species, grid_size);
        let solver = Euler::new(&state);

        Ok(ReactionDiffusionSystem {
            model: ReactionDiffusionModel {
                reaction,
                diffusion,
                diffusion_coeffs,
            },
            state,
            solver,
        })
    }
}

impl<R: ReactionModel, D: DiffusionModel, S: Solver<ChemicalState>>
    ReactionDiffusionSystemBuilder<R, D, S>
{
    /// Builds the `ReactionDiffusionSystem` with a custom solver.
    pub fn build_with_solver(
        self,
    ) -> Result<ReactionDiffusionSystem<R, D, S>, ReactionDiffusionError> {
        let num_species = self
            .num_species
            .ok_or(ReactionDiffusionError::MissingParameter("num_species"))?;
        let grid_size = self
            .grid_size
            .ok_or(ReactionDiffusionError::MissingParameter("grid_size"))?;
        let reaction = self
            .reaction
            .ok_or(ReactionDiffusionError::MissingParameter("reaction"))?;
        let diffusion = self
            .diffusion
            .ok_or(ReactionDiffusionError::MissingParameter("diffusion"))?;
        let diffusion_coeffs = self
            .diffusion_coeffs
            .ok_or(ReactionDiffusionError::MissingParameter("diffusion_coeffs"))?;
        let solver = self
            .solver
            .ok_or(ReactionDiffusionError::MissingParameter("solver"))?;

        if num_species == 0 {
            return Err(ReactionDiffusionError::ZeroSpecies);
        }

        if grid_size == 0 {
            return Err(ReactionDiffusionError::ZeroGridSize);
        }

        if diffusion_coeffs.len() != num_species {
            return Err(ReactionDiffusionError::DimensionMismatch {
                expected: num_species,
                got: diffusion_coeffs.len(),
            });
        }

        Ok(ReactionDiffusionSystem {
            model: ReactionDiffusionModel {
                reaction,
                diffusion,
                diffusion_coeffs,
            },
            state: ChemicalState::new(num_species, grid_size),
            solver,
        })
    }
}

impl<R: ReactionModel, D: DiffusionModel> ReactionDiffusionSystem<R, D, Euler<ChemicalState>> {
    /// Creates a new builder for a Reaction-Diffusion system.
    pub fn builder() -> ReactionDiffusionSystemBuilder<R, D, Euler<ChemicalState>> {
        ReactionDiffusionSystemBuilder::new()
    }
}

impl<R: ReactionModel, D: DiffusionModel, S: Solver<ChemicalState>>
    ReactionDiffusionSystem<R, D, S>
{
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
