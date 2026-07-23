# 6. Parameterized Calculus Scheme for Numerical Differentiation

Date: 2026-07-22

## Status

Accepted

## Context

Previously, downstream mathematical submodules (such as root finders, operators, and surface modules) duplicated finite difference formulas and Jacobian calculations. This duplication resulted in inconsistent step-size values, variable precision behaviors, and increased maintenance overhead across the codebase.

Furthermore, because these operations frequently execute within performance-sensitive and high-integrity execution blocks, we needed a centralized solution that guarantees **zero dynamic heap allocations** at runtime while strictly adhering to static analysis statement limits and stack-only execution requirements.

## Decision

We introduce a centralized, parameterized `CalculusScheme` utility that standardizes numerical differentiation.

1. **Centralized Configuration:** Introduce `DifferentiationConfig` to encapsulate centralized step-size configurations, with default values standardized to `1e-5`.
2. **Stack-Allocated Execution:** The differentiation utility operates completely on the stack with no dynamic heap allocations. This prevents runtime allocator overhead or aborts in critical execution paths.
3. **Closure-Based API:** We designed clean, closure-based interfaces to accept arbitrary mathematical functions, allowing high flexibility without over-engineering complex coordinate-free tensor-field differentiation traits.
4. **Refactoring Downstream Modules:** Refactored mathematical submodules (`roots.rs`, `operators.rs`, `surface.rs`) to eliminate duplicate inline loops and instead leverage `CalculusScheme`.
5. **Static Analysis Compliance:** To comply with static analysis limits, all consolidated functions are optimized and restricted to under 60 statements.

## Consequences

* **Positive:** Standardized, highly precise finite difference step sizes and algorithms across all mathematical modules.
* **Positive:** Guarantees zero runtime heap allocations, ensuring suitability for performance-critical execution loops.
* **Positive:** Reduced cognitive load and code duplication.
* **Negative:** All downstream submodules had to be refactored to conform to the new parameterized interfaces.
