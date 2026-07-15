use math_commons::math_kernel::types::StepSize;

/// Standard spatial numerical operators for fused stencil evaluations.
#[derive(Debug, Clone, Copy)]
pub struct StencilOperators {
    #[allow(missing_docs)]
    pub dx: StepSize,
    #[allow(missing_docs)]
    pub dy: StepSize,
}

impl StencilOperators {
    #[allow(missing_docs)]
    #[verified_engine::verified]
    pub fn new(dx: StepSize) -> Self {
        Self {
            dx,
            dy: StepSize(1.0),
        }
    }

    #[allow(missing_docs)]
    #[verified_engine::verified]
    pub fn new_2d(dx: StepSize, dy: StepSize) -> Self {
        Self { dx, dy }
    }

    /// Central difference for 1st derivative (gradient): (u_{i+1} - u_{i-1}) / 2dx
    #[inline(always)]
    #[verified_engine::verified]
    pub fn central_diff_1st(&self, prev: f64, next: f64) -> f64 {
        (next - prev) / (2.0 * *self.dx)
    }

    /// Central difference for 2nd derivative (Laplacian) in 1D.
    #[inline(always)]
    #[verified_engine::verified]
    pub fn central_diff_2nd(&self, prev: f64, curr: f64, next: f64) -> f64 {
        (next - 2.0 * curr + prev) / (*self.dx * *self.dx)
    }

    /// Central difference for 2nd derivative (Laplacian) in 2D.
    #[inline(always)]
    #[verified_engine::verified]
    pub fn central_diff_2nd_2d(&self, curr: f64, left: f64, right: f64, up: f64, down: f64) -> f64 {
        let d2x = (right - 2.0 * curr + left) / (*self.dx * *self.dx);
        let d2y = (down - 2.0 * curr + up) / (*self.dy * *self.dy);
        d2x + d2y
    }

    /// Upwind scheme for advection flux: evaluates d/dx (v * m) or similar upwind terms.
    /// `v` is the drift velocity. Returns the first derivative approximation based on upwind direction.
    #[inline(always)]
    #[verified_engine::verified]
    pub fn upwind_flux(&self, v: f64, prev: f64, curr: f64, next: f64) -> f64 {
        if v > 0.0 {
            (curr * v - prev * v) / *self.dx
        } else {
            (next * v - curr * v) / *self.dx
        }
    }
}

/// A generic Fused Stencil Stepper for performance-optimized grid updates.
/// By evaluating operators inside the grid loop, this avoids memory bandwidth bottlenecks
/// caused by temporary derivative buffers.
pub struct FusedStencilStepper {
    #[allow(missing_docs)]
    pub ops: StencilOperators,
}

impl FusedStencilStepper {
    #[allow(missing_docs)]
    #[verified_engine::verified]
    pub fn new(dx: StepSize) -> Self {
        Self {
            ops: StencilOperators::new(dx),
        }
    }

    #[allow(missing_docs)]
    #[verified_engine::verified]
    pub fn new_2d(dx: StepSize, dy: StepSize) -> Self {
        Self {
            ops: StencilOperators::new_2d(dx, dy),
        }
    }

    /// Applies a 1D fused stencil pass over a single array.
    /// `dir`: +1.0 for forward-time, -1.0 for backward-time equations.
    /// Boundaries are not updated and must be handled by the caller.
    #[verified_engine::verified]
    pub fn step_1d_slice<F>(&self, src: &[f64], dst: &mut [f64], dt: f64, dir: f64, mut op: F)
    where
        F: FnMut(usize, f64, f64, f64, &StencilOperators) -> f64,
    {
        let n = src.len();
        if n > 2 && dst.len() >= n {
            for (i, win) in src.windows(3).enumerate() {
                let prev = win[0];
                let curr = win[1];
                let next = win[2];
                let rhs = op(i + 1, prev, curr, next, &self.ops);
                dst[i + 1] = curr + dir * dt * rhs;
            }
        }
    }

    /// Applies a 1D fused stencil pass over multiple coupled arrays with Neumann boundaries.
    #[verified_engine::verified]
    pub fn step_1d_coupled_neumann<const N: usize, F>(
        &self,
        n: usize,
        src: [&[f64]; N],
        dst: [&mut [f64]; N],
        dt: f64,
        dir: f64,
        mut op: F,
    ) where
        F: FnMut(usize, [f64; N], [f64; N], [f64; N], &StencilOperators) -> [f64; N],
    {
        if n == 0 {
            return;
        }

        let get_state = |idx| {
            let mut val = [0.0; N];
            for s in 0..N {
                if idx < src[s].len() {
                    val[s] = src[s][idx];
                }
            }
            val
        };

        // Left boundary
        {
            let curr = get_state(0);
            let prev = curr; // Neumann
            let next = if n > 1 { get_state(1) } else { curr };
            let rhs = op(0, prev, curr, next, &self.ops);
            for s in 0..N {
                if !dst[s].is_empty() {
                    dst[s][0] = curr[s] + dir * dt * rhs[s];
                }
            }
        }

        // Interior
        if n > 2 {
            for i in 1..n - 1 {
                let prev = get_state(i - 1);
                let curr = get_state(i);
                let next = get_state(i + 1);
                let rhs = op(i, prev, curr, next, &self.ops);
                for s in 0..N {
                    if i < dst[s].len() {
                        dst[s][i] = curr[s] + dir * dt * rhs[s];
                    }
                }
            }
        }

        // Right boundary
        if n > 1 {
            let i = n - 1;
            let curr = get_state(i);
            let prev = get_state(i - 1);
            let next = curr; // Neumann
            let rhs = op(i, prev, curr, next, &self.ops);
            for s in 0..N {
                if i < dst[s].len() {
                    dst[s][i] = curr[s] + dir * dt * rhs[s];
                }
            }
        }
    }

    /// Applies a 2D fused stencil pass over multiple coupled arrays with Neumann boundaries.
    #[allow(clippy::too_many_arguments)]
    #[verified_engine::verified]
    pub fn step_2d_coupled_neumann<const N: usize, F>(
        &self,
        dim: (usize, usize),
        src: [&[f64]; N],
        dst: [&mut [f64]; N],
        dt: f64,
        dir: f64,
        mut op: F,
    ) where
        F: FnMut(
            usize,
            [f64; N],
            [f64; N],
            [f64; N],
            [f64; N],
            [f64; N],
            &StencilOperators,
        ) -> [f64; N],
    {
        let (width, height) = dim;
        let n = width * height;
        if n == 0 {
            return;
        }

        let get_state = |idx| {
            let mut val = [0.0; N];
            for s in 0..N {
                if idx < src[s].len() {
                    val[s] = src[s][idx];
                }
            }
            val
        };

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;

                let idx_l = if x > 0 { idx - 1 } else { idx };
                let idx_r = if x < width - 1 { idx + 1 } else { idx };
                let idx_u = if y > 0 { idx - width } else { idx };
                let idx_d = if y < height - 1 { idx + width } else { idx };

                let curr = get_state(idx);
                let left = get_state(idx_l);
                let right = get_state(idx_r);
                let up = get_state(idx_u);
                let down = get_state(idx_d);

                let rhs = op(idx, curr, left, right, up, down, &self.ops);

                for s in 0..N {
                    if idx < dst[s].len() {
                        dst[s][idx] = curr[s] + dir * dt * rhs[s];
                    }
                }
            }
        }
    }
}
