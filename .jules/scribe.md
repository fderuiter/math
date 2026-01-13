# Scribe Journal

## 2025-05-18 - [Heat Equation Grid Resolution]
**Discovery:** The `HeatEquationSolver::new` method constructs a grid based on `(usize, usize)` dimensions but calculates step sizes `du` and `dv` by dividing by `(n - 1)`.
**Definition:** This means a grid resolution of 0 or 1 will cause a panic (division by zero or overflow/underflow if wrapping). The solver implicitly requires at least a 2x2 grid to function correctly, though physically it needs much more for stability.
