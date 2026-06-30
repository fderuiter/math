# Systems Core Journal

## Purpose
This document tracks the evolution of the core system architecture, including major design patterns, critical structural decisions, and overarching principles for `math_explorer`.

## Established Patterns
- **Strategy Pattern:** Used for solver implementations to allow interchangeable algorithms.
- **Builder Pattern:** Used for complex model initializations to enforce constraints before execution.

## Recent Architectural Changes
- Initial version-controlled integration of architectural records.
- **Automated Plugin Discovery Pattern:** Replaced manual GUI tab registration with a build-time discovery mechanism using `build.rs` to generate instantiation logic, enforcing open-closed principle for module extensions.
