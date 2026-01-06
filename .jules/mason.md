# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.
## 2024-05-23 - Decoupling Randomization in Clinical Trials
**Violation:** Dependency Inversion Principle (DIP). The randomization functions (, ) were tightly coupled to , making deterministic testing impossible.
**Remedy:** Strategy Pattern. Extracted  trait and  /  structs that accept an injected  instance.
## 2024-05-23 - Decoupling Randomization in Clinical Trials
**Violation:** Dependency Inversion Principle (DIP). The randomization functions were tightly coupled to rand::thread_rng(), making deterministic testing impossible.
**Remedy:** Strategy Pattern. Extracted AllocationStrategy trait and SimpleRandomizer / BlockRandomizer structs that accept an injected Rng instance.
