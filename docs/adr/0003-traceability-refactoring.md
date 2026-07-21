# 2. Traceability Component Refactoring

Date: 2026-07-21

## Status

Accepted

## Context

The `traceability_cli.rs` and `traceability.rs` modules contained overly complex functions that violated NASA Power of 10 constraints (functions exceeding 60 lines) and triggered `clippy::cognitive_complexity` and `clippy::too_many_lines` warnings.

## Decision

We refactored `scan_repository` by extracting logical components into helper functions: `scan_papers`, `read_registry`, and `discover_code_files`. Furthermore, we refactored nested `if` statements in `discover_code_dirs` to use chained logical conditions, addressing `clippy::collapsible_if`.

## Consequences

*   The traceability logic is now modular and strictly adheres to the 60-line function limit.
*   CI builds complete without `clippy` warning failures.
