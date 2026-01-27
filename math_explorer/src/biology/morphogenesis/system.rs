use crate::pure_math::analysis::ode::{OdeSystem, TimeStepper};
use super::kinetics::{ReactionKinetics, SchnakenbergKinetics};
use super::state::TuringState;

/// Represents a 1D Reaction-Diffusion system.
pub struct TuringSystem<K: ReactionKinetics = SchnakenbergKinetics> {
    /// The current state of the system.
    pub state: TuringState,

    // Double buffer for the next state.
    next_state: TuringState,

    /// Diffusion coefficient for u
    pub d_u: f64,
    /// Diffusion coefficient for v
    pub d_v: f64,
    /// Grid spacing
    pub dx: f64,
    /// Reaction kinetics strategy
    pub kinetics: K,
}

impl TuringSystem<SchnakenbergKinetics> {
    /// Creates a new Turing System with default Schnakenberg kinetics.
    pub fn new(size: usize, d_u: f64, d_v: f64, dx: f64) -> Self {
        Self::new_with_kinetics(size, d_u, d_v, dx, SchnakenbergKinetics::default())
    }
}

impl<K: ReactionKinetics> TuringSystem<K> {
    /// Creates a new Turing System with custom kinetics.
    pub fn new_with_kinetics(size: usize, d_u: f64, d_v: f64, dx: f64, kinetics: K) -> Self {
        Self {
            state: TuringState::new(size),
            next_state: TuringState::new(size),
            d_u,
            d_v,
            dx,
            kinetics,
        }
    }

    /// Accessor for the activator concentrations (backward compatibility/convenience).
    pub fn u(&self) -> &[f64] {
        self.state.u()
    }

    /// Accessor for the inhibitor concentrations (backward compatibility/convenience).
    pub fn v(&self) -> &[f64] {
        self.state.v()
    }

    /// Mutable accessor for the activator concentrations.
    pub fn u_mut(&mut self) -> &mut [f64] {
        self.state.u_mut()
    }

    /// Mutable accessor for the inhibitor concentrations.
    pub fn v_mut(&mut self) -> &mut [f64] {
        self.state.v_mut()
    }

    /// Generic stencil applicator for 1D reaction-diffusion.
    ///
    /// This method abstracts the finite difference stencil and sliding window optimization.
    /// It applies the operation:
    /// `rate = D * Laplacian(u) + Reaction(u, v)`
    ///
    /// The `op` closure determines how this rate is used (e.g., added to state vs written to derivative).
    ///
    /// # Arguments
    /// * `op` - A closure `FnMut(index, u_curr, v_curr, rate_u, rate_v)`
    #[allow(clippy::too_many_arguments)]
    #[inline(always)]
    fn apply_reaction_diffusion_stencil<F>(
        state: &TuringState,
        kinetics: &K,
        d_u: f64,
        d_v: f64,
        dx: f64,
        mut op: F,
    ) where
        F: FnMut(usize, f64, f64, f64, f64),
    {
        let n = state.len();
        if n == 0 {
            return;
        }

        let dx_sq = dx * dx;
        let inv_dx_sq = 1.0 / dx_sq;

        let u = &state.u;
        let v = &state.v;

        // 1. Handle i = 0
        {
            let i = 0;
            // Safety: n > 0 checked above
            let u_curr = unsafe { *u.get_unchecked(i) };
            let v_curr = unsafe { *v.get_unchecked(i) };

            let u_prev = u_curr;
            let v_prev = v_curr;
            let (u_next, v_next) = if n > 1 {
                unsafe { (*u.get_unchecked(1), *v.get_unchecked(1)) }
            } else {
                (u_curr, v_curr)
            };

            let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
            let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

            let (reaction_u, reaction_v) = kinetics.reaction(u_curr, v_curr);

            let rate_u = d_u * lap_u + reaction_u;
            let rate_v = d_v * lap_v + reaction_v;

            op(i, u_curr, v_curr, rate_u, rate_v);
        }

        // 2. Handle i = 1..n-1 (Hot Path)
        if n > 2 {
            // Optimization: Sliding Window / Register Rotation
            unsafe {
                let mut u_prev = *u.get_unchecked(0);
                let mut u_curr = *u.get_unchecked(1);
                let mut v_prev = *v.get_unchecked(0);
                let mut v_curr = *v.get_unchecked(1);

                for i in 1..n - 1 {
                    let u_next = *u.get_unchecked(i + 1);
                    let v_next = *v.get_unchecked(i + 1);

                    let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                    let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                    let (reaction_u, reaction_v) = kinetics.reaction(u_curr, v_curr);

                    let rate_u = d_u * lap_u + reaction_u;
                    let rate_v = d_v * lap_v + reaction_v;

                    op(i, u_curr, v_curr, rate_u, rate_v);

                    // Shift window
                    u_prev = u_curr;
                    u_curr = u_next;
                    v_prev = v_curr;
                    v_curr = v_next;
                }
            }
        }

        // 3. Handle i = n-1
        if n > 1 {
            let i = n - 1;
            unsafe {
                let u_curr = *u.get_unchecked(i);
                let v_curr = *v.get_unchecked(i);
                let u_prev = *u.get_unchecked(i - 1);
                let v_prev = *v.get_unchecked(i - 1);
                let u_next = u_curr;
                let v_next = v_curr;

                let lap_u = (u_next - 2.0 * u_curr + u_prev) * inv_dx_sq;
                let lap_v = (v_next - 2.0 * v_curr + v_prev) * inv_dx_sq;

                let (reaction_u, reaction_v) = kinetics.reaction(u_curr, v_curr);

                let rate_u = d_u * lap_u + reaction_u;
                let rate_v = d_v * lap_v + reaction_v;

                op(i, u_curr, v_curr, rate_u, rate_v);
            }
        }
    }

    /// Updates the grid using a finite-difference Laplacian and reaction kinetics.
    pub fn step(&mut self, dt: f64) {
        let n = self.state.len();
        if n == 0 {
            return;
        }

        // Ensure buffers are the right size
        if self.next_state.len() != n {
            self.next_state = TuringState::new(n);
        }

        let next_u = &mut self.next_state.u;
        let next_v = &mut self.next_state.v;

        Self::apply_reaction_diffusion_stencil(
            &self.state,
            &self.kinetics,
            self.d_u,
            self.d_v,
            self.dx,
            |i, u, v, du, dv| {
                // Safety: apply_stencil guarantees i is within bounds 0..n
                unsafe {
                    *next_u.get_unchecked_mut(i) = u + dt * du;
                    *next_v.get_unchecked_mut(i) = v + dt * dv;
                }
            },
        );

        // Swap buffers (states)
        std::mem::swap(&mut self.state, &mut self.next_state);
    }
}

impl<K: ReactionKinetics> OdeSystem<TuringState> for TuringSystem<K> {
    fn derivative(&self, t: f64, state: &TuringState) -> TuringState {
        let mut out = TuringState::new(state.len());
        self.derivative_in_place(t, state, &mut out);
        out
    }

    fn derivative_in_place(&self, _t: f64, state: &TuringState, out: &mut TuringState) {
        let n = state.len();
        if n == 0 {
            return;
        }

        // Ensure output buffer is the right size
        if out.len() != n {
            *out = TuringState::new(n);
        }

        let out_u = &mut out.u;
        let out_v = &mut out.v;

        Self::apply_reaction_diffusion_stencil(
            state,
            &self.kinetics,
            self.d_u,
            self.d_v,
            self.dx,
            |i, _u, _v, du, dv| {
                unsafe {
                    *out_u.get_unchecked_mut(i) = du;
                    *out_v.get_unchecked_mut(i) = dv;
                }
            },
        );
    }
}

impl<K: ReactionKinetics> TimeStepper<TuringState> for TuringSystem<K> {
    fn get_state(&self) -> &TuringState {
        &self.state
    }

    fn get_state_mut(&mut self) -> &mut TuringState {
        &mut self.state
    }

    fn step(&mut self, dt: f64) {
        // Delegate to the optimized inherent method
        // Inherent method shadows trait method, so self.step refers to TuringSystem::step
        self.step(dt);
    }
}
