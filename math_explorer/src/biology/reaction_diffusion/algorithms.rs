use super::model::{ChemicalState, DiffusionModel, ReactionDiffusionModel, ReactionModel};
use crate::pure_math::analysis::ode::solvers::Euler;
use crate::pure_math::analysis::ode::traits::Solver;

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
