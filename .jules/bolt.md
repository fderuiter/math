## 2024-05-18 - Prevented Grid Clone in Heat Equation Solver
**Learning:** In tight numerical loops like `HeatEquationSolver::step`, cloning the entire grid `Vec<Vec<f64>>` per step causes significant heap allocation overhead.
**Action:** When implementing iterative PDE solvers, add a secondary buffer (like `next_grid`) to the struct state and use `std::mem::swap` to swap the grids at the end of each step, completely eliminating the per-step allocations.
