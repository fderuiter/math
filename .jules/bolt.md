## 2024-05-18 - Prevented Grid Clone in Heat Equation Solver
**Learning:** In tight numerical loops like `HeatEquationSolver::step`, cloning the entire grid `Vec<Vec<f64>>` per step causes significant heap allocation overhead.
**Action:** When implementing iterative PDE solvers, add a secondary buffer (like `next_grid`) to the struct state and use `std::mem::swap` to swap the grids at the end of each step, completely eliminating the per-step allocations.

## 2024-05-18 - Prevented Redundant DVector Allocations in ODE Solvers
**Learning:** Calling `solver.solve(...)` inside iterative simulation loops creates a full clone of the state vector (e.g., `DVector`) on every single timestep because `solve` returns a new state.
**Action:** Always use the in-place `solver.step(..., &mut state, ...)` method instead of `solve` in simulation loops to eliminate $O(N)$ redundant heap allocations.
