# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2025-12-21 - [MRI Module Decomposition]
**Violation:** Single Responsibility Principle (SRP) broken in `math_explorer/src/physics/mri.rs`. The file was a "God File" mixing Quantum Physics, ODE Simulation, Hardware Logic, and Image Reconstruction.
**Remedy:** Applied "Extract Class" / "Module Decomposition". Split `mri.rs` into a directory `mri/` with dedicated submodules: `proton.rs`, `bloch.rs`, `scanner.rs`, and `reconstruction.rs`. Preserved API via `mod.rs` re-exports.
