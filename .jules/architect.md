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

## 2026-10-27 - [Medical Physics Dose Calculation Decomposition]
**Problem:** `math_explorer/src/physics/medical/dose_calculation.rs` was a "Kitchen Sink" file (mixed concerns) containing Radiation Physics (`calculate_terma`, `point_kernel`), Machine Physics (`beam_loading_energy`), Geometry (`tracking_error`), and Signal Processing (`dirac_pulse_count`). This violated the Single Responsibility Principle and hindered scalability.
**Decision:** Applied "Module Extraction" and "Error Modeling".
1. Split `dose_calculation.rs` into `dose.rs` (core radiation), `accelerator.rs` (machine physics), `geometry.rs` (IGRT), and `signal.rs` (processing).
2. Introduced `MedicalPhysicsError` (Error Modeling) to replace stringly-typed errors.
3. Introduced `BeamLoadingModel` (Structification) to encapsulate machine constants.
**Consequence:** Improved cohesion and type safety. Each module now focuses on a single domain. `dose_calculation` file was removed, but functionalities are preserved in more logical homes.
