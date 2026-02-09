# Scribe Journal

## 2025-05-18 - [Heat Equation Grid Resolution]
**Discovery:** The `HeatEquationSolver::new` method constructs a grid based on `(usize, usize)` dimensions but calculates step sizes `du` and `dv` by dividing by `(n - 1)`.
**Definition:** This means a grid resolution of 0 or 1 will cause a panic (division by zero or overflow/underflow if wrapping). The solver implicitly requires at least a 2x2 grid to function correctly, though physically it needs much more for stability.

## 2025-05-19 - [Finite Difference Sliding Window]
**Discovery:** The `FiniteDifference1D` implementation uses `unsafe` pointer arithmetic and manual register rotation (sliding window) to optimize the hot loop of the diffusion operator.
**Definition:** This avoids redundant bounds checks and memory accesses for adjacent stencil points (i-1, i, i+1) by carrying them in local variables across iterations. The safety relies on strict buffer length validation and loop bounds.
