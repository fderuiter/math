## 2026-05-15 - [Biology Domain Decomposition]
**Problem:** `math_explorer/src/biology.rs` was a "God File" combining four disparate biological sub-domains: Enzyme Kinetics, Hodgkin-Huxley Neuroscience, Reaction-Diffusion Morphogenesis, and Evolutionary Game Theory.
**Decision:** Applied "Module Extraction" to split `biology.rs` into a directory-based module `math_explorer/src/biology/` with dedicated files: `kinetics.rs`, `neuroscience.rs`, `morphogenesis.rs`, and `evolution.rs`.
**Consequence:** Improved separation of concerns and scalability. Each biological sub-domain is now isolated, making it easier to extend (e.g., adding more neuroscience models) without cluttering unrelated logic. Backward compatibility is fully preserved via `mod.rs` re-exports.

## 2026-05-21 - [Medical Physics Decomposition]
**Problem:** `math_explorer/src/physics/medical.rs` contained four distinct Treatment Planning domains (Calibration, Dose, Optimization, Evaluation) in a single file, violating the Single Responsibility Principle and limiting scalability.
**Decision:** Applied "Module Extraction" to split `medical.rs` into a directory-based module `math_explorer/src/physics/medical/` with dedicated submodules.
**Consequence:** Improved navigability and allows independent evolution of dose algorithms and optimization strategies.

## 2026-05-28 - [Standard Model Physics Decomposition]
**Problem:** `math_explorer/src/physics/standard_model.rs` was a monolithic file mixing disparate particle physics domains: Gauge Theory, Higgs Mechanism, Flavor Physics (CKM), QCD, and Neutrino Physics. This coupling hindered independent development and navigation.
**Decision:** Applied "Module Extraction" to decompose `standard_model.rs` into a directory-based module `math_explorer/src/physics/standard_model/` with specialized submodules: `gauge.rs`, `higgs.rs`, `flavor.rs`, `qcd.rs`, and `neutrinos.rs`.
**Consequence:** Separation of concerns is enforced, allowing distinct physical theories to evolve independently (e.g., adding PMNS matrix to neutrinos without touching quarks). Backward compatibility is maintained via `mod.rs` re-exports.
