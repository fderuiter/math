# Mason Journal

## Universal Domain Alignment (PR 1003)

- **Issue**: Mathematical modules in the AI and biology domains were bypassing strict integrity checks. Unlinked modules did not trigger CI failures.
- **Resolution**: Implemented Universal Domain Alignment.
    - Updated `traceability_cli.rs` to enforce strict exit codes when unlinked modules are detected.
    - Upgraded `verify_suite.py` to perform deep, recursive scans for all domains instead of shallow checks.
    - Enforced the use of the `theory_verification!` macro across all `.rs` files within the scanned domains.
- **Impact**: Ensures 100% verification parity across all scientific domains (physics, pure math, AI, biology).

## Unified 3D Framework

- **Issue**: 3D visualization modules (Attractors, Surface Viewer, Spin Viz) independently implemented manual projection and rotation logic, causing inconsistent coordinate systems (Y-up vs Z-up) and navigation behaviors.
- **Resolution**: Extracted 3D camera and projection logic into `math_explorer_gui::framework::Camera3D`. Refactored target modules to use this standardized camera, ensuring a consistent Z-up coordinate system and unified direct-manipulation controls (drag-to-rotate, scroll-to-zoom).
- **Impact**: Zero duplicated projection/rotation logic across 3D modules. Consistent navigation UX.

## Schema-Driven Executable Theory (PR 1013)

- **Issue**: Mathematical model parameters suffered from "theory drift" where the Rust logic, GUI slider code, and LaTeX documentation could become unsynchronized. Communication via string-keyed message passing (`SimCommand::UpdateParam`) was fragile.
- **Resolution**: Introduced JSON-based schemas (`/schemas/`) as a single source of truth for all simulation parameters. Updated `math_explorer_gui/build.rs` to auto-generate `egui` sliders. Integrated a pipeline to export `.tex` tables directly from the schemas. Replaced string-keyed updates with strongly-typed parameter commands.
- **Impact**: Ensures code execution exactly matches theoretical documentation and GUI interfaces while providing compile-time type safety for parameter updates.
