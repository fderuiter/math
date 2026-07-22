# 2. Active Documentation Standard

Date: 2026-07-22

## Status

Accepted

## Context

We had isolated helper scripts, unlinked design logs, and disconnected crate documentation previously cluttering the workspace. This resulted in maintenance overhead and contributor confusion. We needed a single, consolidated, compiler-validated source of truth for our documentation.

## Decision

We decided to implement the **Active Documentation Standard** across the workspace crates.
We extracted all existing inner doc-comments from the primary entry points of all workspace crates into localized `README.md` files. We then integrated these files back into the source via `#![doc = include_str!("../README.md")]`.
We also migrated the safety guidelines directly into `CONTRIBUTING.md` and removed deprecated python scripts.

## Consequences

* Any documentation updates made in the markdown files are automatically reflected in Rustdocs upon compilation.
* Centralized safety rules inside the primary developer manual ensures they are read and followed during onboarding.
* Legacy Python maintenance scripts were deleted.
