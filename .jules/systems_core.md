# Systems Core Engineering Log

## 2025-02-14 - Strategy Pattern for MFG Hamiltonian
**Violation:** The Mean Field Game `FixedPointSolver` had hardcoded logic for the Hamiltonian ($H(p) = p^2/2$) and drift calculation, violating OCP and making it impossible to simulate games with different kinetic energy models (e.g., relativistic or control costs).
**Refactor:** Applied the **Strategy Pattern**. Extracted a `Hamiltonian` trait with `evaluate` and `derivative` methods. Updated `FixedPointSolver` to be generic over `H: Hamiltonian`.
**Trade-off:** Minimal complexity increase (generic parameters) for significant flexibility gain. Default implementation (`QuadraticHamiltonian`) preserves backward compatibility.

## 2025-05-20 - Strategy Pattern for Kalman Filtering
**Violation:** The Radar Gating `TrackingFilter` was hardcoded to a Constant Velocity (CV) model, violating OCP and preventing the use of higher-order models (e.g., Constant Acceleration) or different state spaces without copying the core Kalman logic.
**Refactor:** Applied the **Strategy Pattern**. Extracted a `KalmanModel` trait (providing $F, Q, H, R$) and a generic `KalmanFilter<T, D, M, Model>` struct. The original `TrackingFilter` was preserved as a wrapper around the CV implementation for backward compatibility.
**Trade-off:** Increased generic complexity (`T, D, M` bounds) and required explicit allocator bounds for `nalgebra`, but gained ability to plug in any linear dynamic system model.
