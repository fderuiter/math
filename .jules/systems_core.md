# Systems Core Journal

## Purpose
This document tracks the evolution of the core system architecture, including major design patterns, critical structural decisions, and overarching principles for `math_explorer`.

## Established Patterns
- **Strategy Pattern:** Used for solver implementations to allow interchangeable algorithms.
- **Builder Pattern:** Used for complex model initializations to enforce constraints before execution.

## Recent Architectural Changes
- Initial version-controlled integration of architectural records.
- Transitioned from filesystem-based naming parity to a centralized TOML registry (traceability.toml) for theory linking.
