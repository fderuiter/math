use super::reaction::ReactionKinetics;
use super::state::TuringState;
use crate::biology::diffusion::SpatialDiffusion;
use crate::pure_math::analysis::ode::{OdeSystem, Solver, VectorOperations};

/// Strategy for time-stepping the Turing System.
///
/// This trait allows decoupling the numerical integration method from the system definition.
pub trait TuringSolverStrategy<const N: usize = 2> {
    /// Performs a single time step of the simulation.
    ///
    /// # Arguments
    /// * `state` - Current system state.
    /// * `next_state` - Buffer for the next system state.
    /// * `kinetics` - Reaction kinetics model.
    /// * `diffusion` - Spatial diffusion model.
    /// * `diffusion_coeffs` - Diffusion coefficients for each species.
    /// * `dt` - Time step size.
    #[allow(clippy::too_many_arguments)]
    fn step<K: ReactionKinetics<N>, D: SpatialDiffusion<N>>(
        &mut self,
        state: &TuringState<N>,
        next_state: &mut TuringState<N>,
        kinetics: &K,
        diffusion: &D,
        diffusion_coeffs: [f64; N],
        dt: f64,
    );
}

/// A descriptor for the Turing system's physics (OdeSystem).
///
/// This struct implements `OdeSystem` to allow standard solvers to integrate
/// the reaction-diffusion equations.
pub struct TuringDynamics<'a, const N: usize, K, D> {
    pub kinetics: &'a K,
    pub diffusion: &'a D,
    pub diffusion_coeffs: [f64; N],
}

impl<'a, const N: usize, K: ReactionKinetics<N>, D: SpatialDiffusion<N>> OdeSystem<TuringState<N>>
    for TuringDynamics<'a, N, K, D>
{
    fn derivative(&self, t: f64, state: &TuringState<N>) -> TuringState<N> {
        let mut out = TuringState::new(state.len());
        self.derivative_in_place(t, state, &mut out);
        out
    }

    fn derivative_in_place(&self, _t: f64, state: &TuringState<N>, out: &mut TuringState<N>) {
        let n = state.len();
        if n == 0 {
            return;
        }

        // SAFETY: Check vector lengths to avoid UB in unsafe block
        for (i, vec) in state.concentrations.iter().enumerate() {
            assert_eq!(vec.len(), n, "State vector {} length mismatch", i);
        }

        // Ensure output buffer is the right size
        if out.len() != n {
            *out = TuringState::new(n);
        } else {
            for (i, vec) in out.concentrations.iter().enumerate() {
                assert_eq!(vec.len(), n, "Output vector {} length mismatch", i);
            }
        }

        // Prepare slices for diffusion application
        let state_slices: [&[f64]; N] = std::array::from_fn(|i| state.concentrations[i].as_slice());

        let mut out_iter = out.concentrations.iter_mut();
        let out_slices: [&mut [f64]; N] =
            std::array::from_fn(|_| out_iter.next().unwrap().as_mut_slice());

        // 1. Compute Diffusion
        self.diffusion
            .apply(state_slices, out_slices, self.diffusion_coeffs);

        // 2. Compute Reaction and Accumulate
        let state_ptrs: [*const f64; N] = std::array::from_fn(|i| state.concentrations[i].as_ptr());
        let out_ptrs: [*mut f64; N] = std::array::from_fn(|i| out.concentrations[i].as_mut_ptr());

        unsafe {
            for i in 0..n {
                // Gather inputs
                let inputs: [f64; N] = std::array::from_fn(|s| *state_ptrs[s].add(i));

                // Compute Reaction
                let rates = self.kinetics.reaction(inputs);

                // Accumulate results
                for s in 0..N {
                    *out_ptrs[s].add(i) += rates[s];
                }
            }
        }
    }
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
/// solver.step(&state, &mut next_state, &kinetics, &diffusion, [1.0, 40.0], 0.01);
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

impl<const N: usize> TuringSolverStrategy<N> for FusedEulerSolver {
    #[inline]
    fn step<K: ReactionKinetics<N>, D: SpatialDiffusion<N>>(
        &mut self,
        state: &TuringState<N>,
        next_state: &mut TuringState<N>,
        kinetics: &K,
        diffusion: &D,
        diffusion_coeffs: [f64; N],
        dt: f64,
    ) {
        let n = state.len();
        if n == 0 {
            return;
        }

        // SAFETY: Check vector lengths to avoid UB in unsafe block
        if next_state.len() != n {
            return;
        }
        // Explicitly check all buffers in next_state to ensure safety for get_unchecked
        for (i, vec) in next_state.concentrations.iter().enumerate() {
            assert!(
                vec.len() >= n,
                "Output buffer {} too small in FusedEulerSolver",
                i
            );
        }

        let concentrations_slices: [&[f64]; N] =
            std::array::from_fn(|i| state.concentrations[i].as_slice());

        // Use stack allocation (array) instead of heap allocation (Vec)
        let mut out_iter = next_state.concentrations.iter_mut();
        let mut next_slices: [&mut [f64]; N] =
            std::array::from_fn(|_| out_iter.next().unwrap().as_mut_slice());

        diffusion.map_diffusion(concentrations_slices, diffusion_coeffs, |i, vals, diffs| {
            let rates = kinetics.reaction(vals);

            for s in 0..N {
                let curr = vals[s];
                let diff = diffs[s];
                let reac = rates[s];

                // SAFETY: We verified that all next_slices have length >= n at the start of the function.
                // map_diffusion iterates i from 0 to n-1.
                // Thus, next_slices[s][i] is safe.
                unsafe {
                    *next_slices.get_unchecked_mut(s).get_unchecked_mut(i) =
                        curr + dt * (diff + reac);
                }
            }
        });
    }
}

/// Adapts a standard ODE Solver to the TuringSolverStrategy trait.
///
/// This adapter allows using any generic `Solver` (like RungeKutta4) within the TuringSystem.
pub struct StandardSolverAdapter<S> {
    pub solver: S,
}

impl<S> StandardSolverAdapter<S> {
    pub fn new(solver: S) -> Self {
        Self { solver }
    }
}

impl<const N: usize, S: Solver<TuringState<N>>> TuringSolverStrategy<N>
    for StandardSolverAdapter<S>
{
    fn step<K: ReactionKinetics<N>, D: SpatialDiffusion<N>>(
        &mut self,
        state: &TuringState<N>,
        next_state: &mut TuringState<N>,
        kinetics: &K,
        diffusion: &D,
        diffusion_coeffs: [f64; N],
        dt: f64,
    ) {
        let dynamics = TuringDynamics {
            kinetics,
            diffusion,
            diffusion_coeffs,
        };

        next_state.copy_from(state);
        self.solver.step(&dynamics, 0.0, next_state, dt);
    }
}
