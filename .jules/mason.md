# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2024-05-24 - Decomposing Solid State Physics and Decoupling Quantum Statistics
**Violation:** Single Responsibility Principle (SRP) - `solid_state.rs` was a God Object handling BCS, Phonons, and Quantum Stats. Open/Closed Principle (OCP) - `ParticleType` used rigid match statements.
**Remedy:** Module Decomposition + Strategy Pattern. Split `solid_state.rs` into submodules. Extracted `ParticleStatistics` trait, implemented by `Fermion`/`Boson` structs and legacy `ParticleType` enum.
