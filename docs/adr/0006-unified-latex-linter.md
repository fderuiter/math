# 6. Unified LaTeX Linter

Date: 2026-07-22

## Status

Accepted

## Context

Mathematical formulas embedded in Markdown documentation and Rust source docstrings were previously not validated during compilation or CI runs. This made it easy for syntactically invalid formulas (such as unmatched curly braces or environment blocks) to slip through undetected, leading to broken mathematical rendering in the published public documentation.

To prevent this, we need a high-performance, native LaTeX validator integrated directly into our workspace verification CLI, guaranteeing offline verification with zero external dependency overhead.

## Decision

We design and implement a custom LaTeX math linter subcommand (`lint-latex`) inside the high-integrity `unified_verification` crate.

1.  **Custom Rust-Native Parsers**: Designed a lightweight parser optimized specifically for standard math delimiters (`$` and `$$`).
2.  **Modular Extraction Logic**: Split LaTeX math block extraction helper methods and structs into a dedicated `latex_extractor.rs` file, keeping both `latex_extractor.rs` and `latex_linter.rs` under the workspace-wide 500-line limit to satisfy `check-file-lengths`.
3.  **Precise Error Mapping**: Track and preserve absolute line numbers. If validation fails, developers get exact line contexts pointing directly to the source of the error.
4.  **CI/CD Integration**: Integrated the linter into the high-integrity `verify_suite` workflow (`cargo run -p unified_verification --release -- verify-suite`).

## Consequences

*   **Positive:** Guaranteed correctness of mathematical formulas in Markdown files and Rust docstrings.
*   **Positive:** Fully offline, fast, and self-contained with zero network or heavy external toolchain requirements.
*   **Positive:** Both extractor and linter source files are clean, modular, and strictly adhere to file length limits.
*   **Negative:** Developers must write compliant LaTeX math syntax; minor formatting deviations (like unbalanced braces in comments or unescaped percents) may trigger linter warnings.
