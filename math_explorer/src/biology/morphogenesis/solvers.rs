use crate::biology::diffusion::SpatialDiffusion;
use super::state::TuringState;
use super::reaction::ReactionKinetics;

/// Defines the strategy for time-stepping the Turing system.
///
/// This trait decouples the numerical integration method from the system definition,
/// allowing for easy swapping of solvers (e.g., Euler vs. Runge-Kutta).
pub trait TuringSolverStrategy {
    /// Advances the system state by one time step `dt`.
    ///
    /// # Arguments
    /// * `state` - The current state of the system.
    /// * `next_state` - A mutable scratchpad buffer (same size as state).
    /// * `kinetics` - The reaction kinetics model.
    /// * `diffusion` - The spatial diffusion model.
    /// * `coeffs` - Diffusion coefficients `[d_u, d_v]`.
    /// * `dt` - The time step size.
    fn step<K: ReactionKinetics, D: SpatialDiffusion<2>>(
        &mut self,
        state: &TuringState,
        next_state: &mut TuringState,
        kinetics: &K,
        diffusion: &D,
        coeffs: [f64; 2],
        dt: f64,
    );
}

/// A Forward Euler solver with Fused Diffusion-Reaction-Integration loop.
///
/// This solver is optimized for performance by combining the diffusion, reaction,
/// and integration steps into a single pass over the data, maximizing cache locality.
#[derive(Debug, Clone, Copy, Default)]
pub struct FusedEulerSolver;

impl TuringSolverStrategy for FusedEulerSolver {
    fn step<K: ReactionKinetics, D: SpatialDiffusion<2>>(
        &mut self,
        state: &TuringState,
        next_state: &mut TuringState,
        kinetics: &K,
        diffusion: &D,
        coeffs: [f64; 2],
        dt: f64,
    ) {
        let n = state.len();
        if n == 0 {
            return;
        }

        // Ensure buffers are the right size
        // Note: The caller (TuringSystem) generally handles this, but we double-check or resize if needed.
        if next_state.len() != n {
            *next_state = TuringState::new(n);
        }

        let u = &state.u;
        let v = &state.v;
        let next_u = &mut next_state.u;
        let next_v = &mut next_state.v;
        let d_u = coeffs[0];
        let d_v = coeffs[1];

        // Fused Diffusion-Reaction-Integration Step
        // This is significantly faster than separate passes because it keeps data in registers/L1 cache.
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
                // We must ensure next_u/next_v are large enough.
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
