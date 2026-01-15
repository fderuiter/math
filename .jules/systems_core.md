# Systems Core Engineering Log

## 2025-02-14 - Strategy Pattern for MFG Hamiltonian
**Violation:** The Mean Field Game `FixedPointSolver` had hardcoded logic for the Hamiltonian ($H(p) = p^2/2$) and drift calculation, violating OCP and making it impossible to simulate games with different kinetic energy models (e.g., relativistic or control costs).
**Refactor:** Applied the **Strategy Pattern**. Extracted a `Hamiltonian` trait with `evaluate` and `derivative` methods. Updated `FixedPointSolver` to be generic over `H: Hamiltonian`.
**Trade-off:** Minimal complexity increase (generic parameters) for significant flexibility gain. Default implementation (`QuadraticHamiltonian`) preserves backward compatibility.

## 2025-02-14 - Dependency Inversion for Evolutionary Game Dynamics
**Violation:** The `ReplicatorDynamics` struct hardcoded the `RungeKutta4` solver in its `simulate` method, violating the Dependency Inversion Principle and making it impossible to use alternative solvers (e.g., Euler for performance or experimental solvers).
**Refactor:** Applied **Dependency Inversion**. Introduced `simulate_with_strategy` accepting a generic `S: Solver`. Retained `simulate` as a convenience wrapper.
**Trade-off:** User must specify solver strategy for advanced usage, but default API remains unchanged.

## 2026-01-15 - Strategy Pattern for Kalman Filter Physics
**Violation:** `TrackingFilter` was coupled to the Constant Velocity (CV) physics model (hardcoded transition and measurement matrices), violating OCP and hindering the addition of higher-order models like Constant Acceleration.
**Refactor:** Applied the **Strategy Pattern**. Extracted `KalmanModel` trait (defining $F, H, Q, R$). Implemented `ConstantVelocityModel`. Refactored `TrackingFilter` to be generic over `M: KalmanModel`.
**Trade-off:** Constructor API changed (requires injecting model instance), but separated Physics from Estimation logic, enabling zero-cost abstraction for different motion models.
