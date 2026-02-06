## 2026-05-15 - [Biology Domain Decomposition]
**Problem:** `math_explorer/src/biology.rs` was a "God File" combining four disparate biological sub-domains: Enzyme Kinetics, Hodgkin-Huxley Neuroscience, Reaction-Diffusion Morphogenesis, and Evolutionary Game Theory.
**Decision:** Applied "Module Extraction" to split `biology.rs` into a directory-based module `math_explorer/src/biology/` with dedicated files: `kinetics.rs`, `neuroscience.rs`, `morphogenesis.rs`, and `evolution.rs`.
**Consequence:** Improved separation of concerns and scalability. Each biological sub-domain is now isolated, making it easier to extend (e.g., adding more neuroscience models) without cluttering unrelated logic. Backward compatibility is fully preserved via `mod.rs` re-exports.

## 2026-05-21 - [Medical Physics Decomposition]
**Problem:** `math_explorer/src/physics/medical.rs` contained four distinct Treatment Planning domains (Calibration, Dose, Optimization, Evaluation) in a single file, violating the Single Responsibility Principle and limiting scalability.
**Decision:** Applied "Module Extraction" to split `medical.rs` into a directory-based module `math_explorer/src/physics/medical/` with dedicated submodules.
**Consequence:** Improved navigability and allows independent evolution of dose algorithms and optimization strategies.

## 2026-05-25 - [Solid State Physics Decomposition]
**Problem:** `math_explorer/src/physics/solid_state.rs` was a large file (~400 lines) mixing six distinct physical domains: Second Quantization, Screening, Lattice Dynamics, Magnetism, BCS Theory, and Electron-Phonon Interactions.
**Decision:** Applied "Module Extraction" to split `solid_state.rs` into a directory-based module `math_explorer/src/physics/solid_state/` with dedicated files for each domain.
**Consequence:** Greatly improved cohesion. Each file now represents a single physical domain. The API remains backward compatible via re-exports in `mod.rs`, but the codebase is now much more scalable for adding future solid state models.
## 2025-01-10 - Nuclear Physics Decomposition
**Problem:** The `physics/nuclear/mod.rs` file was a "God File" containing mixed concerns: properties, binding energy models, decay logic, and reaction formulas. This coupling made it hard to add new models (like Shell Model corrections) or swap binding energy parameters.
**Decision:** Applied **Module Extraction** to split the domains into `properties.rs`, `decay.rs`, `reactions.rs`. Applied **Traitification** and **Structification** to the Liquid Drop Model, creating a `BindingEnergyModel` trait and a configurable `LiquidDropModel` struct.
**Consequence:** The public API in `mod.rs` is now a set of wrappers maintaining backward compatibility, while the internal architecture is modular and scalable. Users can now instantiate `LiquidDropModel` with custom constants for fitting.

## 2026-06-01 - [Hodgkin-Huxley Model Decomposition]
**Problem:** `math_explorer/src/biology/neuroscience.rs` coupled the mathematical model (differential equations), the state representation (struct), and the numerical solver (explicit Euler) into a single struct `HodgkinHuxleyNeuron`. This prevented the use of higher-order solvers (like Runge-Kutta 4) and made the system hard to test or extend.
**Decision:** Applied "Model-State Separation" and "Strategy Pattern". Split `neuroscience.rs` into `types.rs` (State), `model.rs` (OdeSystem logic), and `neuron.rs` (Facade). Adopted the existing `OdeSystem` trait architecture.
**Consequence:** The `HodgkinHuxleyNeuron` is now a thin facade. Users can still use the simple API, but the underlying system now supports dependency injection of solvers and is strictly typed via `VectorOperations`.

## 2026-06-03 - [Isosurface Extraction Traitification]
**Problem:** `math_explorer/src/applied/isosurface/marching_cubes.rs` used manual component-wise arithmetic (e.g., `x + t * (x2 - x1)`) for vector interpolation and gradient calculation. This was verbose, error-prone, and obscured the mathematical intent.
**Decision:** Applied "Traitification" to `Point3D` in `types.rs`, implementing `Add`, `Sub`, `Mul`, `Div`, `Neg`, and geometric helpers (`dot`, `cross`, `normalize`). Refactored `marching_cubes.rs` to use these operators.
**Consequence:** The isosurface extraction logic is now significantly more readable and idiomatic ("Rust idioms"). Mathematical operations are expressed as vector algebra. Performance remains equivalent (inlined operations).

## 2026-06-15 - [Turing System Traitification]
**Problem:** `math_explorer/src/biology/morphogenesis.rs` implemented a custom solver loop inside `TuringSystem::step`, preventing the use of generic ODE solvers (like Runge-Kutta 4) and duplicating integration logic. The state `TuringState` lacked standard arithmetic operators, making it incompatible with the library's `OdeSystem` ecosystem.
**Decision:** Applied "Traitification". Implemented `VectorOperations`, `OdeSystem`, and `TimeStepper` for `TuringSystem`. Preserved the highly optimized sliding-window stencil logic in `derivative_in_place` and the manual Euler step in `step`.
**Consequence:** `TuringSystem` is now a first-class citizen in the generic analysis module. It can be simulated with any `Solver` while maintaining zero-cost abstraction for the default use case.

## 2026-06-16 - [Reaction-Diffusion Stencil Unification]
**Problem:** The Reaction-Diffusion stencil logic (finite difference Laplacian + kinetics) was duplicated in both `step` (optimized Euler) and `derivative_in_place` (generic ODE support) to preserve performance, leading to a DRY violation.
**Decision:** Extracted the stencil logic into a private `apply_reaction_diffusion_stencil` method accepting a closure.
**Consequence:** Eliminates code duplication while preserving the performance of the hot loop (unsafe access, unrolling).

## 2026-10-24 - [ODE Solvers Modularization]
**Problem:** `math_explorer/src/pure_math/analysis/ode.rs` mixed concerns: core traits (`OdeSystem`), concrete solvers (`Euler`, `RungeKutta4`), state wrappers (`VecState`), and high-level traits (`TimeStepper`). This hampered extensibility and made the file difficult to navigate.
**Decision:** Applied "Module Extraction". Split `ode.rs` into a module `math_explorer/src/pure_math/analysis/ode/` with `traits.rs`, `solvers.rs`, `state.rs`, and `stepper.rs`. Introduced `ArrayState<const N: usize>` in `state.rs` as a zero-dependency, stack-allocated alternative to `VecState` and `nalgebra` types.
**Consequence:** Clearer separation of concerns. Adding new solvers or state types is now modular. The API remains backward compatible via re-exports. `ArrayState` provides a "Newtype" solution for small systems without external dependencies.

## 2026-10-27 - [Advanced Linear Algebra Module]
**Problem:** The library lacked comprehensive support for Advanced Linear Algebra concepts like Canonical Forms, Decomposition analysis (beyond basic `nalgebra` usage), and their physical interpretations.
**Decision:** Created a new module `math_explorer/src/pure_math/algebra/linear_algebra/` decomposed into `eigen.rs`, `canonical.rs`, and `decomposition.rs`.
**Consequence:** Provides a dedicated space for educational and practical implementations of advanced linear algebra, bridging the gap between raw matrix libraries (`nalgebra`) and physical/theoretical applications. Enforced strong typing (e.g., `JordanBlock` struct) and extensive documentation.
## 2027-05-23 - [Tensor & Vector Calculus Implementation]
**Problem:** The `pure_math` module lacked generalized Tensor Analysis and Vector Calculus tools, limiting the project's ability to model physics in curvilinear coordinates or general manifolds.
**Decision:** Implemented `math_explorer/src/pure_math/tensor` (Metric, Christoffel, Differentiation) and `math_explorer/src/pure_math/vector_calculus` (Orthogonal Coordinates, Operators, Integral Theorems).
**Consequence:** Enables rigorous mathematical modeling in arbitrary coordinate systems. Enforced "Newtype" pattern for Tensors and "Strategy Pattern" for Coordinate Systems to maintain architectural standards.

## 2027-05-24 - [Algebra Ring Decomposition]
**Problem:** `math_explorer/src/pure_math/algebra/ring.rs` contained mixed implementations of Finite Fields (`Fp`) and Polynomial Rings (`Polynomial`), conflating two distinct algebraic structures and limiting clarity.
**Decision:** Applied "Module Extraction". Decomposed `ring.rs` into `fields.rs` (containing `Fp`) and `polynomial.rs` (containing `Polynomial`). Updated `mod.rs` to re-export them, maintaining backward compatibility.
**Consequence:** Improved separation of concerns. `Polynomial` is now a standalone module, ready for future expansion (e.g., GCD, factorization), while `Fp` is isolated in `fields.rs`.

## 2025-05-23 - [Safe Fluid Construction]
**Problem:** `FluidProperties` in `physics::fluid_dynamics` allowed invalid physical states (negative density/viscosity), leading to potential silent failures or NaNs in solvers.
**Decision:** Implemented the **Builder Pattern** and strictly validated constructors. `FluidProperties` now enforces `density > 0` and `viscosity >= 0` via a `Result`-returning constructor and a fluent Builder API. Fields are private to prevent bypassing validation.
**Consequence:** Prevents "Illegal States" in the fluid dynamics module. This introduces a breaking change to `FluidProperties::new`, improving safety at the cost of requiring error handling in initialization logic.

## 2027-06-01 - [Linear Algebra Consolidation & Optimization Extraction]
**Problem:** `math_explorer/src/pure_math/analysis/linear_algebra.rs` was a misplaced file containing both numerical linear algebra solvers (which belong in `algebra`) and optimization structures (`L1RegularizedLeastSquares`), blurring the lines between "Analysis" and "Algebra".
**Decision:**
1. Extracted `L1RegularizedLeastSquares` to a new module `math_explorer/src/pure_math/analysis/optimization.rs`.
2. Moved numerical solvers (`solve_linear_system`, `solve_normal_equation`, etc.) to `math_explorer/src/pure_math/algebra/linear_algebra/numerical.rs`.
3. Deleted the legacy `analysis/linear_algebra.rs`.
**Consequence:** Enforced strict domain boundaries. Optimization logic is now distinct from Linear Algebra. All Linear Algebra functionality (abstract and numerical) is consolidated under `pure_math/algebra`.

## 2027-06-10 - [Epidemiology Compartmental Decomposition]
**Problem:** `math_explorer/src/epidemiology/compartmental.rs` was becoming a monolithic file containing multiple distinct disease models (SIR, SEIR) and shared utility logic, violating the Single Responsibility Principle.
**Decision:** Applied "Module Extraction". Decomposed `compartmental.rs` into a directory-based module `math_explorer/src/epidemiology/compartmental/` with `sir.rs`, `seir.rs`, `common.rs` (for validation and R0), and `macros.rs` (for arithmetic ops).
**Consequence:** Improved separation of concerns and scalability. New compartmental models (e.g., SIS, SIRS) can be added as separate files without bloating a single source file. API backward compatibility is preserved via re-exports.

## 2027-06-25 - [Hodgkin-Huxley Neuron Encapsulation]
**Problem:** `HodgkinHuxleyNeuron` exposed raw public fields (`v`, `n`, `m`, `h`), allowing users to construct invalid states (e.g., probability > 1.0) and bypassing internal logic.
**Decision:** Applied "Encapsulation" and "Builder Pattern".
1. Made all fields private.
2. Introduced `HodgkinHuxleyNeuronBuilder` for safe, validated construction.
3. Added getters and validated setters (`set_n` checks bounds).
**Consequence:** The API is now safe against invalid states. This is a breaking change for code accessing fields directly (must now use getters).

## 2027-06-30 - [Stochastic Simulation Extraction]
**Problem:** `math_explorer/src/epidemiology/stochastic.rs` contained the general-purpose Gillespie Algorithm (SSA) and the `StochasticSystem` trait, tightly coupling a fundamental mathematical tool to the Epidemiology domain. This prevented reuse for other stochastic systems (e.g., Chemistry, Physics).
**Decision:** Applied "Module Extraction".
1. Created `math_explorer/src/pure_math/analysis/stochastic.rs`.
2. Moved `StochasticSystem` trait and `GillespieSolver` struct there.
3. Introduced `StochasticError` for explicit error handling (replacing `panic!`).
4. Updated `epidemiology/stochastic.rs` to import/re-export these tools.
**Consequence:** The Gillespie Algorithm is now a first-class citizen in the `pure_math` library, available for any domain. The API is safer (`Result` return type) and more modular.
