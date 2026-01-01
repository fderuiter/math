# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2024-05-24 - Decoupling Randomness in Favoritism Logic
**Violation:** Dependency Inversion Principle (DIP). The `calculate_favoritism_score` function had a hard-coded dependency on `rand::thread_rng()`, making it non-deterministic and impossible to unit test effectively.
**Remedy:** Dependency Injection. Created `FavoritismCalculator<R: Rng>` to inject the RNG strategy. The original function is now a deprecated wrapper maintaining backward compatibility.
