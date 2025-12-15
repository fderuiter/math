# Systems Core - Engineering Decision Records

## 2025-12-16 - [High Energy Physics Decomposition] **Violation:** `high_energy.rs` was a "God File" mixing Special Relativity, General Relativity, Radiative Processes, Fluid Dynamics, and Statistics. **Refactor:** Extracted into cohesive submodules (`observer`, `radiation`, `fluid_dynamics`, `general_relativity`, `statistics`) within a directory. **Trade-off:** Increased file count vs significantly improved Separation of Concerns and maintainability.
