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

## 2026-02-20 - Dependency Inversion for Battery Degradation
**Violation:** The `battery_degradation` module was tightly coupled to the concrete `PowerLawModel` struct, violating the Open/Closed Principle and preventing the addition of alternative degradation models (e.g., electrochemical or semi-empirical) without modifying existing code.
**Refactor:** Applied **Dependency Inversion**. Extracted a `DegradationModel` trait defining the contract for cycle life and capacity fade. Implemented this trait for `PowerLawModel`.
**Trade-off:** Added a layer of abstraction (the trait), but enabled full extensibility for future battery chemistries while preserving backward compatibility via inherent methods.

## 2026-05-22 - Strategy Pattern for Quantum Statistics
**Violation:** The `occupancy_probability` function in `quantum_stats.rs` contained hardcoded formulas for Fermi-Dirac, Bose-Einstein, and Maxwell-Boltzmann distributions within a `match` statement, violating OCP.
**Refactor:** Applied the **Strategy Pattern**. Extracted a `StatisticalDistribution` trait with an `occupancy` method. Implemented strategies for each distribution type.
**Trade-off:** Minimal boilerplate increase for trait/struct definitions. Enables adding new distributions (e.g., Anyons) without modifying the core logic. Backward compatibility maintained via wrapper.

## 2026-05-24 - Encapsulation of Vector Operations in Isosurface Extraction
**Violation:** The `marching_cubes` module exhibited **Primitive Obsession** and repeated logic by manually performing component-wise vector arithmetic on `Point3D` structs, leading to verbose and brittle code in `interpolate` and `interpolate_normal`.
**Refactor:** Implemented standard `std::ops` traits (`Add`, `Sub`, `Mul`, `Div`) and geometric methods (`dot`, `cross`, `normalize`) for `Point3D`. Refactored `marching_cubes` to use these vector abstractions.
**Trade-off:** Minimal overhead for method calls (likely optimized away), significantly improved readability and maintainability by treating `Point3D` as a mathematical vector entity.
