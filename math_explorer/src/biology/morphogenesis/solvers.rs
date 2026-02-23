use super::reaction::ReactionKinetics;
use super::state::TuringState;
use crate::biology::diffusion::SpatialDiffusion;

/// Defines the strategy for time-stepping a Turing system.
///
/// This trait decouples the numerical integration method from the physics model.
pub trait TuringSolverStrategy<K: ReactionKinetics, D: SpatialDiffusion<2>> {
    /// Advances the system state by `dt`.
    ///
    /// # Arguments
    /// * `state` - The current state (read/write, but usually read).
    /// * `next_state` - The buffer for the next state (write).
    /// * `kinetics` - The reaction kinetics model.
    /// * `diffusion` - The spatial diffusion model.
    /// * `d_u` - Diffusion coefficient for u.
    /// * `d_v` - Diffusion coefficient for v.
    /// * `dt` - Time step size.
    fn step(
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

/// A solver that uses a fused Diffusion-Reaction-Integration loop.
///
/// This solver is optimized for CPU cache locality by performing all operations
/// for a grid point in a single pass. It implements a simple Forward Euler integration.
#[derive(Debug, Clone, Copy, Default)]
pub struct FusedEulerSolver;

impl<K: ReactionKinetics, D: SpatialDiffusion<2>> TuringSolverStrategy<K, D> for FusedEulerSolver {
    fn step(
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

        // Ensure buffers are the right size
        // Note: Resizing is typically handled by the system wrapper, but we ensure safety here.
        if next_state.len() != n {
            *next_state = TuringState::new(n);
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

                // Safety: map_diffusion guarantees i is within bounds of input.
                // We ensured next_state is same size.
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
