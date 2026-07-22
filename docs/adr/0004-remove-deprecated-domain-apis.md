# ADR 0004: Remove Deprecated Domain APIs and Modernize Tests

## Status
Accepted

## Context
Developers previously faced unnecessary cognitive overhead and slower compilation speeds due to obsolete, allocation-heavy legacy functions and deprecated APIs cluttering the codebase. These deprecated functions were unused in production but persisted within legacy unit tests and verification baseline configurations.

## Decision
To establish a clean, zero-debt baseline and boost developer velocity, we have completely pruned these deprecated endpoints from our domain and applied science libraries. To prevent test coverage regression, we migrated all affected unit and integration tests to use modern, high-performance, and strongly-typed APIs. Crucially, we maintained strict numerical output precision to prevent calculation regressions and preserve compliance with our traceability requirements.

Key Decisions:
- **Strong Type-Safety over Primitive Floats:** Migrated astrophysics calculations from bare float-based and stringly-typed inputs to robust, strongly-typed domain primitives.
- **Unified Stepping Interface:** Replaced custom, procedural stepping loops with a unified `TimeStepper` pattern.
- **Refactor, Don't Delete:** Upgraded all legacy tests to assert the same logic using modern APIs, preserving 100% test coverage.

## Consequences
- **Positive:** Cleaner, easier to maintain codebase. Reduced cognitive overhead. Stronger type guarantees shifting errors to compile time.
- **Negative:** Downstream consumers (if any existed for these deprecated internal endpoints) will experience breaking changes, though none were intended for public production use.
