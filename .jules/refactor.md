# Refactor's Journal

## 2024-05-22 - [Game Theory] **Debt:** Panic in Constructor **Risk:** Panic risk
`ReplicatorDynamics::new` panics if the payoff matrix is not square. This should be a `Result` to allow callers to handle invalid input gracefully.

## 2024-05-22 - [Radar Gating] **Debt:** Silent Failure & Panic **Risk:** Panic risk
`MusicEstimator::add_snapshot` silently ignores input if dimensions don't match. `compute_spectrum` uses `unwrap()` inside `sort_by`, which panics on NaN.

## 2024-05-20 - [Physics/Linear Algebra] **Debt:** Inconsistent Error Handling & Option Returns **Risk:** Panic/Ambiguity
Found inconsistent error handling in physics modules (String errors) and `Option` returns in linear algebra where `Result` would be more informative.
Refactored `HighEnergyError` and `LinearAlgebraError` to unify and improve error reporting.
Updated `li_ma_significance` to return `Result`.
Updated `solve_linear_system` and `solve_normal_equation` to return `Result`.
## 2026-01-20 - [Physics/Chaos, Applied/Isosurface] **Debt:** Stringly Typed Errors **Risk:** Panic/Unpredictable Behavior
Replaced `Result<T, String>` with typed error enums `ChaosError` and `IsosurfaceError`. This improves error handling safety and allows consumers to match on specific error cases instead of parsing strings.

## 2024-05-23 - [Applied/Battery Degradation] **Debt:** Panic in Constructor **Risk:** Panic risk
`DepthOfDischarge::new`, `Capacity::new`, `Cycles::new` panicked on invalid input. Refactored to return `Result<Self, BatteryError>`. Also updated `PowerLawModel` and wrapper functions to propagate errors.

## 2024-05-23 - [Pure Math/Elliptic Curves] **Debt:** Explicit Panic **Risk:** Panic risk
`theorem_1_1_bounds` and `theorem_1_2_bounds` panicked if condition `i + j < psi(N)` was not met. Refactored to return `Result<..., EllipticError>`.

## 2024-05-23 - [Pure Math/Number Theory] **Debt:** Panic in Div **Risk:** Panic risk
`QSeries::div` panicked on division by zero or zero constant term. Added `try_div` returning `Result<..., NumberTheoryError>`.

## 2024-05-23 - [Climate/Cera] **Debt:** Stringly Typed Errors **Risk:** Unpredictable Behavior
`Cera::new` returned `Result<Self, String>`. Refactored to use `Result<Self, ClimateError>`.
