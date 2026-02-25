use super::reaction::ReactionKinetics;
use super::state::TuringState;
use crate::biology::diffusion::SpatialDiffusion;

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
///
/// # Optimization
///
/// Traditional solvers often compute the derivative into a temporary buffer and then add it to the state:
/// `state_next = state + dt * (diffusion + reaction)`.
/// This requires writing the derivative to memory and reading it back.
///
/// The "Fused" solver uses a closure passed to the diffusion engine to compute the reaction
/// and the time update *immediately* after the Laplacian is computed for a grid point (or block).
/// This keeps the data in L1/L2 cache, significantly improving memory bandwidth efficiency.
///
/// # Stability Warning
///
/// This solver implements the **Forward Euler** method, which is an explicit first-order method.
/// It is conditionally stable. If the time step `dt` is too large relative to the diffusion coefficients
/// and grid spacing, the simulation will explode (values go to infinity).
///
/// The stability condition for 1D diffusion is approximately:
/// $$ \Delta t \le \frac{\Delta x^2}{2 D} $$
///
/// # Examples
///
/// Manually stepping a system:
///
/// ```rust
/// use math_explorer::biology::morphogenesis::{FusedEulerSolver, TuringSolverStrategy, TuringState, SchnakenbergKinetics};
/// use math_explorer::biology::diffusion::FiniteDifference1D;
///
/// // 1. Setup System Components
/// let mut solver = FusedEulerSolver::new();
/// let n = 100;
/// let state = TuringState::new(n);
/// let mut next_state = TuringState::new(n);
/// let kinetics = SchnakenbergKinetics::default();
/// let diffusion = FiniteDifference1D::new(1.0);
///
/// // 2. Perform a single time step
/// // D_u = 1.0, D_v = 40.0, dt = 0.01
/// solver.step(&state, &mut next_state, &kinetics, &diffusion, 1.0, 40.0, 0.01);
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct FusedEulerSolver;

impl FusedEulerSolver {
    /// Creates a new FusedEulerSolver.
    ///
    /// Since the solver holds no state (it is a pure strategy), this is effectively a no-op
    /// provided for API consistency.
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
