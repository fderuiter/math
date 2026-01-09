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

## 2024-05-23 - [Neuroscience: Separation of State and Model]
**Problem:** The `HodgkinHuxleyNeuron` struct was a "God Object" mixing state (membrane potential, gating variables), constants (conductances), and integration logic (Euler method) in a single struct and file.
**Decision:** Applied "Module Extraction" and "Traitification".
- Extracted `HodgkinHuxleyState` and `HodgkinHuxleyParameters` into strong types.
- Implemented `OdeSystem` for `HodgkinHuxleyModel` to decouple the physics from the solver.
- Refactored `HodgkinHuxleyNeuron` to act as a Facade that delegates to these internal components.
**Consequence:**
- Scalability: The model can now be used with advanced solvers (e.g., RK4) via the `OdeSystem` trait.
- Complexity: Added more files and type definitions.
- Constraint: The legacy `update` method manually preserves the staggered Euler integration order to ensure strict backward compatibility, preventing the direct use of the generic `OdeSystem` for the default path.
