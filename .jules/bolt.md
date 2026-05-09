## 2024-05-18 - Prevented Grid Clone in Heat Equation Solver
**Learning:** In tight numerical loops like `HeatEquationSolver::step`, cloning the entire grid `Vec<Vec<f64>>` per step causes significant heap allocation overhead.
**Action:** When implementing iterative PDE solvers, add a secondary buffer (like `next_grid`) to the struct state and use `std::mem::swap` to swap the grids at the end of each step, completely eliminating the per-step allocations.
## 2024-05-18 - Eliminated Allocation Loop in Replicator Dynamics Simulation
**Learning:** In evolutionary game theory simulations, `ReplicatorDynamics::simulate_with_strategy` runs an integration step thousands of times. Using the allocating `solver.solve` method causes a `DVector` clone on every step, severely impacting performance for larger games.
**Action:** Always use the in-place `solver.step(..., &mut state, ...)` method instead of `solve` in hot simulation loops to avoid unnecessary heap allocations.
