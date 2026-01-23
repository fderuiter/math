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

## 2026-06-04 - [Turing System Standardization]
**Problem:** `TuringSystem` in `morphogenesis.rs` used a bespoke, manual Euler integration step, preventing the use of higher-order solvers (RK4) and decoupling from the analysis framework.
**Decision:** Implemented `VectorOperations` for `TuringState` and `OdeSystem` for `TuringSystem`. Maintained the legacy `step` for backward compatibility/performance but enabled generic solver usage.
**Consequence:** `TuringSystem` can now be driven by any solver in `pure_math::analysis::ode`. Logic for `derivative` had to be duplicated/adapted from `step` to fit the `OdeSystem` interface.
