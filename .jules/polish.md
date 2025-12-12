# Polish Journal

## 2025-12-12 - Inconsistent Module Declarations
Smell: Inconsistent mixing of inline modules and file-based modules in `lib.rs`.
Remedy: Enforce `pub mod name;` for top-level modules to keep `lib.rs` as a clean directory of modules.
