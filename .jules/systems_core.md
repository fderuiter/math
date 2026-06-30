# Systems Core Journal

## 2026-06-30: Enforce Global Safety Standards
- Transitioned workspace from opt-in safety to a mandatory standard for domain-critical crates.
- Eliminated recursive implementations in `pure_math` (e.g., `extended_gcd`, `heapify`, `quick_sort`) and replaced with iterative variants.
- Enforced `VerifiedAllocator` as the global allocator to restrict dynamic memory allocation post-initialization.
- Refactored `verified_engine_macros::InjectorVisitor` to inject independent telemetry statements, preventing AST explosion and type inference breakage.
## Purpose
This document tracks the evolution of the core system architecture, including major design patterns, critical structural decisions, and overarching principles for `math_explorer`.

## Established Patterns
- **Strategy Pattern:** Used for solver implementations to allow interchangeable algorithms.
- **Builder Pattern:** Used for complex model initializations to enforce constraints before execution.

## Recent Architectural Changes
- Initial version-controlled integration of architectural records.
- **Automated Plugin Discovery Pattern:** Replaced manual GUI tab registration with a build-time discovery mechanism using `build.rs` to generate instantiation logic, enforcing open-closed principle for module extensions.
- Transitioned from filesystem-based naming parity to a centralized TOML registry (traceability.toml) for theory linking.
