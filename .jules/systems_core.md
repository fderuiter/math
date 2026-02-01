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

## 2026-06-15 - Composition for Favoritism Model
**Violation:** The `calculate_favoritism_score` function was a monolithic procedural block, mixing integration logic, linear algebra, and business rules, violating SRP and OCP.
**Refactor:** Applied **Composition** and **Strategy Pattern**. Decomposed the scoring equation into granular `ScoringStrategy` components (`GiftStrategy`, `ProximityStrategy`, etc.) and orchestrated them via a `UnifiedFavoritismModel` struct.
**Trade-off:** Increased file count and struct definitions, but achieved full separation of concerns and extensibility for individual scoring factors.

## 2026-07-20 - Strategy Pattern for Root Finding
**Violation:** The `optimal_reserve_price` function in `mechanism_design.rs` contained a hardcoded bisection loop with magic numbers (50 iterations), violating OCP and SRP by mixing auction logic with numerical analysis.
**Refactor:** Applied the **Strategy Pattern**. Extracted a `RootFinder` trait and implemented `Bisection` in `pure_math::analysis::roots`. Refactored `mechanism_design.rs` to use Dependency Injection via `optimal_reserve_price_with_solver`.
**Trade-off:** Extracted logic requires managing `AnalysisError`, but enables reusable solvers and strictly bracketed root finding. Legacy fallback behavior was explicitly preserved.

## 2026-10-27 - Extract Parameter Object for Hodgkin-Huxley
**Violation:** The `HodgkinHuxleyModel` contained hardcoded constants (magic numbers) for conductances and potentials in its `derivative` method, violating the "Hard-coding Constraints" rule and preventing configuration for different neuron types.
**Refactor:** Applied **Extract Parameter Object**. Created `HodgkinHuxleyParameters` struct to hold model coefficients. Updated `HodgkinHuxleyModel` and `HodgkinHuxleyNeuron` to use this configuration object.
**Trade-off:** Added a new struct to the public API, but enabled full configurability of the neuron model (Composability) while maintaining default behavior for the Squid Giant Axon.

## 2026-11-20 - Semantic Type for Survival Analysis
**Violation:** `Observation` struct used primitive `f64` for time, leading to scattered validation checks (`if t < 0.0`) and "Primitive Obsession".
**Refactor:** Applied **Newtype Pattern**. Created `SurvivalTime` struct that enforces non-negativity at construction.
**Trade-off:** `Observation` creation is now more verbose (requires `SurvivalTime::new(...).unwrap()`), but invalid states are now unrepresentable at the type level.
## 2026-11-15 - Dependency Inversion for AI Optimization
**Violation:** The `AdamOptimizer` in `ai::sds::training` was a concrete struct used directly, violating the Dependency Inversion Principle and preventing the interchangeability of optimization algorithms (e.g., SGD vs Adam).
**Refactor:** Applied **Dependency Inversion**. Extracted an `Optimizer` trait with a `step` method. Implemented `Optimizer` for `AdamOptimizer` and added a new `SgdOptimizer`.
**Trade-off:** Introduced a trait abstraction which requires importing the trait to use the `step` method, but enabled OCP for optimization strategies.

## 2026-12-10 - Error Hierarchy for Clinical Trials
**Violation:** Inconsistent error handling in `applied::clinical_trials`, where `design`, `sample_size`, and `survival_analysis` modules used primitive `String` or `&'static str` errors, violating the "Error Hierarchy" pattern and preventing programmatic handling of specific failure modes.
**Refactor:** Applied **Error Hierarchy**. Unified the module to use the existing `ClinicalTrialError` enum. Refactored `AllocationStrategy`, `calculate_sample_size_*`, and `try_estimate_hazard_ratio` to return typed errors.
**Trade-off:** Increased verbosity in error construction (requires matching variants) and legacy wrappers needed updates, but achieved type-safety and consistency across the domain module.

## 2027-02-14 - Strategy Pattern for Stochastic Simulation
**Violation:** The `gillespie_step_time` function in `epidemiology::stochastic` had hardcoded `rand::thread_rng()` dependency (Side Effect) and hardcoded 2-reaction logic, violating Dependency Inversion and Open/Closed Principle.
**Refactor:** Applied the **Strategy Pattern**. Defined `StochasticSystem` trait for reaction networks and `GillespieSolver` struct with injected RNG (`R: Rng`).
**Trade-off:** Increased complexity (Generics, Trait definition) compared to a single function, but enabled deterministic testing (seeded RNG) and support for any reaction network (Composability).

## 2027-04-10 - Strategy Pattern for Fluid Dynamics
**Violation:** The `navier_stokes_time_derivative` and `euler_time_derivative` functions violated OCP (hardcoded equations), DRY (duplicated convection/pressure logic), and suffered from Primitive Obsession (long argument lists).
**Refactor:** Applied **Strategy Pattern** via `MomentumEquation` trait and **Parameter Object** via `SpatialGradients`. Implemented `NavierStokes` and `Euler` strategies.
**Trade-off:** Increased boilerplate (structs/impls) but enabled pluggable flow solvers and cleaner API boundaries, while centralizing derivative data.
