# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.
## 2025-12-24 - Decoupling Randomness in Favoritism Calculator
**Violation:** Dependency Inversion Principle (DIP). The  function had a hard-coded dependency on , making it stochastic and hard to test deterministically.
**Remedy:** Dependency Injection. Introduced  to accept any RNG implementation, allowing for deterministic testing.
## 2025-12-24 - Decoupling Randomness in Favoritism Calculator
**Violation:** Dependency Inversion Principle (DIP). The `calculate_favoritism_score` function had a hard-coded dependency on `rand::thread_rng()`, making it stochastic and hard to test deterministically.
**Remedy:** Dependency Injection. Introduced `FavoritismCalculator<R: Rng>` to accept any RNG implementation, allowing for deterministic testing.
