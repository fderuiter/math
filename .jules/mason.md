# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.
## 2024-05-23 - [Split MRI God File] **Violation:** The `mri.rs` file violated SRP by mixing Quantum Physics, Classical Dynamics (Bloch), Scanner Hardware logic, and Image Reconstruction. **Remedy:** Extracted modules `proton`, `bloch`, `scanner`, and `reconstruction` into separate files within an `mri/` directory, re-exporting via `mod.rs` to maintain API compatibility.
