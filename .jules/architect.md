## 2026-05-15 - [Biology Domain Decomposition]
**Problem:** `math_explorer/src/biology.rs` was a "God File" combining four disparate biological sub-domains: Enzyme Kinetics, Hodgkin-Huxley Neuroscience, Reaction-Diffusion Morphogenesis, and Evolutionary Game Theory.
**Decision:** Applied "Module Extraction" to split `biology.rs` into a directory-based module `math_explorer/src/biology/` with dedicated files: `kinetics.rs`, `neuroscience.rs`, `morphogenesis.rs`, and `evolution.rs`.
**Consequence:** Improved separation of concerns and scalability. Each biological sub-domain is now isolated, making it easier to extend (e.g., adding more neuroscience models) without cluttering unrelated logic. Backward compatibility is fully preserved via `mod.rs` re-exports.

## 2026-05-21 - [Medical Physics Decomposition]
**Problem:** `math_explorer/src/physics/medical.rs` contained four distinct Treatment Planning domains (Calibration, Dose, Optimization, Evaluation) in a single file, violating the Single Responsibility Principle and limiting scalability.
**Decision:** Applied "Module Extraction" to split `medical.rs` into a directory-based module `math_explorer/src/physics/medical/` with dedicated submodules.
**Consequence:** Improved navigability and allows independent evolution of dose algorithms and optimization strategies.

## 2026-05-27 - [MRI Physics Decomposition]
**Problem:** `math_explorer/src/physics/mri.rs` was a "God File" (357 lines) mixing Quantum Foundations, Bloch Dynamics (ODE solving), Scanner logic (spatial encoding), and Signal Reconstruction (DFT/IDFT).
**Decision:** Applied "Module Extraction" to decompose `mri.rs` into a `math_explorer/src/physics/mri/` directory with submodules: `proton.rs`, `bloch.rs`, `scanner.rs`, and `reconstruction.rs`.
**Consequence:** Distinct physical regimes (Quantum vs Classical vs Signal Processing) are now structurally isolated. The `BlochSimulator` is now cleaner and easier to test in isolation. Backward compatibility is preserved via `mod.rs` re-exports.
