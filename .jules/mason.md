# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2024-05-24 - The MRI God File
**Violation:** Single Responsibility Principle (SRP). `math_explorer/src/physics/mri.rs` was a "God File" managing Quantum Constants, Bloch Dynamics, Scanner Hardware, and Image Reconstruction. Changing one domain (e.g., Image Reconstruction) risked regressions in unrelated domains (e.g., Quantum Constants).
**Remedy:** Hub and Spoke. Decomposed `mri.rs` into specialized submodules (`proton`, `bloch`, `scanner`, `reconstruction`) under a new `mri/` directory, while preserving the API via `mod.rs` re-exports.
