## 2025-05-23 - [Builder Pattern for TuringSystem]
**Problem:** The `TuringSystem` struct suffers from "Primitive Obsession" and "Construction Risks". Its constructors take many raw `f64` parameters and expose public fields (`state`, `d_u`, `d_v`) that allow invalid states (e.g., mismatched vector lengths or negative diffusion rates). There is no validation that the state size matches the diffusion topology (e.g., 2D grid dimensions).

**Decision:** Implemented the **Builder Pattern** for `TuringSystem` and enhanced the `SpatialDiffusion` trait.
1. Introduced `TuringSystemBuilder` to enforce valid construction and provide fluent API.
2. Added `expected_size()` method to `SpatialDiffusion` trait to allow diffusion strategies to declare their required state size (e.g., `FiniteDifference2D` requires `width * height`).
3. The Builder validates that the user-provided `size` matches the diffusion strategy's `expected_size`.

**Consequence:**
- **Safety:** Impossible to construct a `TuringSystem` with mismatched dimensions at compile/runtime boundary.
- **Ergonomics:** Clearer API for configuration (`.diffusion_rates(1.0, 40.0)` vs `new(..., 1.0, 40.0, ...)`).
- **Extensibility:** Easier to add new parameters (e.g., noise config) without breaking constructors.
- **Breaking Change:** `TuringSystem::new` and `new_with_kinetics` are deprecated.
