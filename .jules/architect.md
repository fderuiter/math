# Architect Journal

## 2026-06-30: Automated Compliance
- Automated CI compliance rules including statement limits (60 statements max) and assertion density (minimum of two assertions per function).
- Established theory parity enforcement requiring a theoretical reference for each domain module.
## Purpose
This document logs domain decomposition decisions, module extraction standards, and the high-level boundary definitions between crates within the ecosystem.

## Module Guidelines
- The system must remain modular. "God files" are strictly forbidden.
- Each domain (e.g., `domain_physics`, `domain_biology`) must encapsulate its specific logic and communicate through `math_commons`.

## Recent Domain Changes
- Baseline domain architecture established.
- Centralized fundamental physical and mathematical constants into `math_commons` to prevent duplication and ensure consistency across domain crates.
- Implemented Automated Plugin Discovery for GUI tabs. Domain modules implementing `ExplorerTab` are now automatically discovered at build time via `math_explorer_gui/build.rs`, removing the need for manual registration in centralized host files.
- Extracted unit tests for Glicko-2 rating system into a separate tests.rs file to maintain the 500-line limit per file rule.
