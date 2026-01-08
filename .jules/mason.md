# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.
## 2026-01-08 - Decomposed Standard Model God File
**Violation:** Single Responsibility Principle (SRP). The `standard_model.rs` file was a 'God File' containing unrelated physics domains (Gauge, Higgs, Flavor, QCD, Neutrinos) in a single module.
**Remedy:** Module Extraction. Split the file into a directory `standard_model/` with separate files for each domain (`gauge.rs`, `higgs.rs`, etc.) and re-exported them via `mod.rs`. This improves cohesion and navigability.
