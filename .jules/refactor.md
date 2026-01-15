# Refactor's Journal

## 2024-05-22 - [Game Theory] **Debt:** Panic in Constructor **Risk:** Panic risk
`ReplicatorDynamics::new` panics if the payoff matrix is not square. This should be a `Result` to allow callers to handle invalid input gracefully.

## 2024-05-22 - [Radar Gating] **Debt:** Silent Failure & Panic **Risk:** Panic risk
`MusicEstimator::add_snapshot` silently ignores input if dimensions don't match. `compute_spectrum` uses `unwrap()` inside `sort_by`, which panics on NaN.
