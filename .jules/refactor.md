# Refactor Journal

## 2025-05-18 - [Strong Error Typing]
**Discovery:** Several modules (`physics/high_energy`, `physics/standard_model`, `epidemiology`) were returning `Result<T, String>`, which is an anti-pattern (Stringly Typed Errors) that prevents callers from programmatically handling specific failure cases.

**Propagation:**
-   Introduced `thiserror` dependency.
-   Created `HighEnergyError`, `StandardModelError`, and `EpidemiologyError` enums.
-   Refactored `Result<T, String>` to `Result<T, CustomError>` in:
    -   `math_explorer/src/physics/high_energy/general_relativity.rs`
    -   `math_explorer/src/physics/high_energy/fluid_dynamics.rs`
    -   `math_explorer/src/physics/standard_model/qcd.rs`
    -   `math_explorer/src/epidemiology/matrix_dynamics.rs`
-   Verified that tests pass and logic is preserved.
