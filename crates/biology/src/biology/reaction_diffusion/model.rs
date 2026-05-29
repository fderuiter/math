use math_core::ode::solvers::Euler;
use math_core::ode::traits::{OdeSystem, Solver};

use super::{ChemicalState, DiffusionModel, ReactionDiffusionError, ReactionModel};

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
