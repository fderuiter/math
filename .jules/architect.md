# Architect's Journal - Architectural Decision Records (ADR)

This journal records significant structural decisions in the `math_explorer` codebase.

## 2024-05-21 - Conservation Strategy
**Problem:** `fluid_dynamics/conservation.rs` contained tightly coupled, explicit functions for different flow regimes (`navier_stokes`, `euler`) with long argument lists, preventing generic simulation loop implementation.
**Decision:** Applied **Strategy Pattern** via `MomentumEquation` trait and introduced `SpatialGradients` **Parameter Object**.
**Consequence:** Simulation loops can now be polymorphic over the physics model. Explicit argument lists are cleaner. Error handling added for invalid fluid properties.
