# 4. Unified Verification Dependency Audit

Date: 2026-07-21

## Status

Accepted

## Context

Our workspace of 19 crates previously lacked central enforcement for unused dependencies and local compiler warning bypasses. Over time, this allowed dead code and unused third-party dependencies to creep into the codebase, increasing compilation times and technical debt.

## Decision

We have integrated a custom Unified Verification Engine directly into our existing `verify-suite`. This tool analyzes the workspace codebase at an AST level, eliminating the need to download or execute heavy external static-analysis binaries during CI runs.

We pruned unused dependencies and established a root-level `verification_whitelist.toml` configuration to track legitimate exceptions.

## Consequences

- Dependency declarations in Cargo.toml are strictly verified against actual usage in the source code.
- Local `#[allow(dead_code)]` attributes must be explicitly whitelisted.
- The `serde_json` dependency was restored to `math_explorer` dev-dependencies as it is required by the `bench_stochastic` example.
