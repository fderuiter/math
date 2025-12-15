# Systems Core - Engineering Decision Records

## 2025-12-16 - [High Energy Physics Decomposition] **Violation:** `high_energy.rs` was a "God File" mixing Special Relativity, General Relativity, Radiative Processes, Fluid Dynamics, and Statistics. **Refactor:** Extracted into cohesive submodules (`observer`, `radiation`, `fluid_dynamics`, `general_relativity`, `statistics`) within a directory. **Trade-off:** Increased file count vs significantly improved Separation of Concerns and maintainability.

## 2025-05-23 - [ODE Solver Extraction] **Violation:** `SIRModel` and `SEIRModel` violated DRY by duplicating RK4 integration logic. **Refactor:** Extracted `RungeKutta4` into `analysis::ode` module using the Strategy Pattern (via `OdeSystem` trait). **Trade-off:** Generic solver adds a trait boundary vs flexibility to swap solvers and models independently.
