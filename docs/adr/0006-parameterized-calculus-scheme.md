# 6. Parameterized Calculus Scheme for Numerical Differentiation

Date: 2026-07-22

## Status

Accepted

## Context

Downstream mathematical submodules (such as root finders, operators, and surface modules) previously duplicated finite difference formulas and Jacobian calculations. This duplication resulted in inconsistent step-size values, variable precision behaviors, and increased cognitive load across the codebase. 

Furthermore, because these operations frequently execute within integrity-critical and performance-sensitive blocks, we needed a centralized solution that guarantees **zero dynamic heap allocations** at runtime.

## Decision

We introduce a centralized `CalculusScheme` utility that standardizes numerical differentiation while strictly adhering to static analysis statement limits and stack-only execution requirements.

1.  **Core Calculus Scheme Utility**:
    *   Introduce `DifferentiationConfig` to encapsulate centralized step-size configuration.
    *   Implement `CalculusScheme` struct supporting first-order and second-order partial derivatives using central finite differences, as well as multi-dimensional Jacobian calculations via static sizing (`jacobian`) and dynamic slicing (`jacobian_slice`).
2.  **Downstream Refactoring**:
    *   Update Newton-Raphson root finder (`roots.rs`) to evaluate numerical derivatives via the parameterized `CalculusScheme`.
    *   Simplify `partial_derivative` calculations in `operators.rs` to leverage the central scheme.
    *   Update first-order partials (`partial_u`, `partial_v`) in `surface.rs` to use `CalculusScheme::jacobian` and second-order fundamental form coefficients to use `CalculusScheme::second_partial_derivative`.
3.  **Stack-Allocated Execution**: The differentiation utility operates completely on the stack with zero dynamic heap allocations at runtime to ensure safety for high-frequency optimization loops.
4.  **Closure-Based API**: We design clean, closure-based interfaces to accept arbitrary mathematical functions.

## Consequences

*   **Positive:** Standardized step-size and differentiation formulas across the codebase, ensuring consistent precision.
*   **Positive:** Elimination of duplicate finite difference loop code in roots, operators, and surfaces.
*   **Positive:** Zero dynamic heap allocations, making the differentiation scheme safe for integrity-critical loops.
*   **Negative:** Callers of dynamic Jacobian calculations must pass pre-allocated scratch buffers to guarantee stack-only execution.
