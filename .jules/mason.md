# Mason's Journal - Architectural Discoveries

## 2024-05-21 - Decoupling MRI Physics from Integration Logic
**Violation:** Open/Closed Principle (OCP) and Dependency Inversion Principle (DIP). The `BlochSimulator` was tightly coupled to the Euler integration method.
**Remedy:** Strategy Pattern. Extracted `BlochSystem` implementing `OdeSystem` and injected `Solver` trait into `BlochSimulator`.

## 2026-01-05 - [Clinical Trials: Randomization Strategy]
**Violation:** DIP (Dependency Inversion Principle) and OCP (Open/Closed Principle) were violated in .
- **DIP:** Hardcoded  usage made testing non-deterministic and tightly coupled logic to the global RNG.
- **OCP:** Adding new randomization strategies (e.g., adaptive) required modifying the existing functions.
- **SRP:**  was tightly coupled to .

**Remedy:** Strategy Pattern.
- Extracted  trait.
- Implemented  and  as strategies.
- Created  which accepts a factory for strategies.
- Injected  via the  method.
- Preserved backward compatibility using deprecated wrapper functions.

## 2025-02-18 - [Clinical Trials: Randomization Strategy]
**Violation:** DIP (Dependency Inversion Principle) and OCP (Open/Closed Principle) were violated in `clinical_trials/design.rs`.
- **DIP:** Hardcoded `thread_rng()` usage made testing non-deterministic and tightly coupled logic to the global RNG.
- **OCP:** Adding new randomization strategies (e.g., adaptive) required modifying the existing functions.
- **SRP:** `stratified_randomization` was tightly coupled to `block_randomization`.

**Remedy:** Strategy Pattern.
- Extracted `AllocationStrategy` trait.
- Implemented `SimpleRandomizer` and `BlockRandomizer` as strategies.
- Created `StratifiedRandomizer` which accepts a factory for strategies.
- Injected `Rng` via the `assign` method.
- Preserved backward compatibility using deprecated wrapper functions.
