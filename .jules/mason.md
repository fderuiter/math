# Mason Journal

## 2026-06-30: Fixed Recursion and Memory Allocation
- Architectural violations found: The codebase relied on recursive algorithms which violate Rule 1 of the Power of 10. Memory allocation was also uncontrolled.
- Fixed: Removed recursion in `extended_gcd`, `heapify`, and `quick_sort`. Implemented `VerifiedAllocator` to prevent heap allocations during the memory lock phase.
## Purpose
This document records low-level architectural constraints, dependency injection requirements, and specific implementation invariants that developers must adhere to.

## Constraints
- **Dependency Injection:** RNG instances must be injected via `&mut R: Rng` to guarantee test determinism.
- **Type Safety:** Primitive obsession is forbidden. Use strongly typed structures for domain concepts.

## Recent Implementation Invariants
- Enforced zero raw-f64 propagation in state definitions.
