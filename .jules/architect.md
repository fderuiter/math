## 2026-05-15 - [Biology Domain Decomposition]
**Problem:** `math_explorer/src/biology.rs` was a "God File" combining four disparate biological sub-domains: Enzyme Kinetics, Hodgkin-Huxley Neuroscience, Reaction-Diffusion Morphogenesis, and Evolutionary Game Theory.
**Decision:** Applied "Module Extraction" to split `biology.rs` into a directory-based module `math_explorer/src/biology/` with dedicated files: `kinetics.rs`, `neuroscience.rs`, `morphogenesis.rs`, and `evolution.rs`.
**Consequence:** Improved separation of concerns and scalability. Each biological sub-domain is now isolated, making it easier to extend (e.g., adding more neuroscience models) without cluttering unrelated logic. Backward compatibility is fully preserved via `mod.rs` re-exports.

## 2026-05-21 - [Medical Physics Decomposition]
**Problem:** `math_explorer/src/physics/medical.rs` contained four distinct Treatment Planning domains (Calibration, Dose, Optimization, Evaluation) in a single file, violating the Single Responsibility Principle and limiting scalability.
**Decision:** Applied "Module Extraction" to split `medical.rs` into a directory-based module `math_explorer/src/physics/medical/` with dedicated submodules.
**Consequence:** Improved navigability and allows independent evolution of dose algorithms and optimization strategies.

## 2026-05-27 - [Standard Model Decomposition]
**Problem:** `math_explorer/src/physics/standard_model.rs` was a single file containing logic for disparate physical domains: Gauge couplings, Higgs mechanism, Flavor physics (CKM), QCD, and Neutrinos. This made it difficult to extend specific sectors (e.g., adding more QCD loop corrections) without navigating unrelated code.
**Decision:** Applied "Module Extraction" to split `standard_model.rs` into a directory-based module `math_explorer/src/physics/standard_model/` with dedicated files for each sector: `gauge.rs`, `higgs.rs`, `flavor.rs`, `qcd.rs`, and `neutrinos.rs`.
**Consequence:** High cohesion within each file and loose coupling between them. This structure sets a scalable pattern for adding future sectors (e.g., lepton sector, cosmology) while preserving the public API via `mod.rs`.
