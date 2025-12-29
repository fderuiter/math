# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2024-05-22 - Decomposing MRI God Module
**Violation:** Single Responsibility Principle (SRP). `math_explorer/src/physics/mri.rs` was a "God Module" handling Quantum Physics, Classical Dynamics, Spatial Encoding, and Image Reconstruction all in one file. This made the module hard to navigate and maintain, with mixed levels of abstraction.
**Remedy:** Module Extraction. Split `mri.rs` into a directory-based module `mri/` with dedicated files: `proton.rs` (Constants), `bloch.rs` (Dynamics), `scanner.rs` (Encoding), and `reconstruction.rs` (Image Processing), re-exported via `mod.rs` to preserve the API.
