# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2024-05-22 - Decoupling Reaction Kinetics in Turing Systems
**Violation:** Open/Closed Principle (OCP) and Single Responsibility Principle (SRP). The `TuringSystem` had hardcoded Schnakenberg kinetics inside the simulation loop, preventing extension to other models (e.g., Gray-Scott) without modifying the core class.
**Remedy:** Strategy Pattern. Extracted `ReactionKinetics` trait and injected it into `TuringSystem`, allowing dynamic selection of reaction rules while preserving the diffusion solver logic.
