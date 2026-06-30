# Architect Journal

## Purpose
This document logs domain decomposition decisions, module extraction standards, and the high-level boundary definitions between crates within the ecosystem.

## Module Guidelines
- The system must remain modular. "God files" are strictly forbidden.
- Each domain (e.g., `domain_physics`, `domain_biology`) must encapsulate its specific logic and communicate through `math_commons`.

## Recent Domain Changes
- Baseline domain architecture established.
- Centralized fundamental physical and mathematical constants into `math_commons` to prevent duplication and ensure consistency across domain crates.
