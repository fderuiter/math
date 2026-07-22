# 4. Remove Legacy Game Theory Wrappers

Date: 2026-07-21

## Status
Accepted

## Context
Platform developers have been experiencing unnecessary friction and confusion due to dead mathematical methods and legacy validation bypasses that clutter the codebase. Specifically, legacy wrapper structures containing bypass attributes (`#[verified_engine::verified(opt_out = "Legacy wrapper")]`) have been generating persistent noise in our automated integrity debt reports.

## Decision
We removed the deprecated legacy wrappers (`MechanismDesign`, `FixedPointVerifier`, `MeanFieldGame1D`) and their bypass attributes from the game theory module. Tests and examples were updated to interact directly with the modern, verified APIs.

## Consequences
- Zero integrity debt alerts for these modules.
- Developers are no longer confused by legacy wrappers.
- Reduced technical debt.
