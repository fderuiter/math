# Mason Journal

## Purpose
This document records low-level architectural constraints, dependency injection requirements, and specific implementation invariants that developers must adhere to.

## Constraints
- **Dependency Injection:** RNG instances must be injected via `&mut R: Rng` to guarantee test determinism.
- **Type Safety:** Primitive obsession is forbidden. Use strongly typed structures for domain concepts.

## Recent Implementation Invariants
- Enforced zero raw-f64 propagation in state definitions.
