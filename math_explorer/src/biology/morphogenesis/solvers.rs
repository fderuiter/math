use crate::biology::diffusion::SpatialDiffusion;
use super::reaction::ReactionKinetics;
use super::state::TuringState;

/// Strategy for time-stepping the Turing System.
///
/// This trait allows decoupling the numerical integration method from the system definition.
pub trait TuringSolverStrategy {
    /// Performs a single time step of the simulation.
    ///
    /// # Arguments
    /// * `state` - Current system state.
    /// * `next_state` - Buffer for the next system state.
    /// * `kinetics` - Reaction kinetics model.
    /// * `diffusion` - Spatial diffusion model.
    /// * `d_u` - Diffusion coefficient for u.
    /// * `d_v` - Diffusion coefficient for v.
    /// * `dt` - Time step size.
    #[allow(clippy::too_many_arguments)]
    fn step<K: ReactionKinetics, D: SpatialDiffusion<2>>(
        &mut self,
        state: &TuringState,
        next_state: &mut TuringState,
        kinetics: &K,
        diffusion: &D,
        d_u: f64,
        d_v: f64,
        dt: f64,
    );
}

/// A fused Euler integration solver.
///
/// This solver combines diffusion, reaction, and time integration into a single pass
/// using `SpatialDiffusion::map_diffusion` to maximize data locality.
#[derive(Debug, Clone, Copy, Default)]
pub struct FusedEulerSolver;

impl FusedEulerSolver {
    /// Creates a new FusedEulerSolver.
    pub fn new() -> Self {
        Self
    }
}

impl TuringSolverStrategy for FusedEulerSolver {
    fn step<K: ReactionKinetics, D: SpatialDiffusion<2>>(
        &mut self,
        state: &TuringState,
        next_state: &mut TuringState,
        kinetics: &K,
        diffusion: &D,
        d_u: f64,
        d_v: f64,
        dt: f64,
    ) {
        let n = state.len();
        if n == 0 {
            return;
        }

        // Ensure buffers are the right size (safety check, though caller should handle)
        if next_state.len() != n {
            return;
        }

        let u = &state.u;
        let v = &state.v;
        let next_u = &mut next_state.u;
        let next_v = &mut next_state.v;

        // Fused Diffusion-Reaction-Integration Step
        diffusion.map_diffusion(
            [u.as_slice(), v.as_slice()],
            [d_u, d_v],
            |i, vals, diffs| {
                let u_curr = vals[0];
                let v_curr = vals[1];
                let diff_u = diffs[0];
                let diff_v = diffs[1];

                let (reac_u, reac_v) = kinetics.reaction(u_curr, v_curr);

                // Safety: map_diffusion guarantees i is within bounds of u/v.
                // We trust next_u/next_v are large enough.
                if i < next_u.len() {
                    next_u[i] = u_curr + dt * (diff_u + reac_u);
                }
                if i < next_v.len() {
                    next_v[i] = v_curr + dt * (diff_v + reac_v);
                }
            },
        );
    }
}
