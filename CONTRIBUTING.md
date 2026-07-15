# Contributing to Math Explorer

First off, thank you for considering contributing to Math Explorer! It's people like you that make this tool such a great resource.

##  The "Golden Rule" of Contribution

**"Code explains HOW; Docs explain WHY."**

When you submit a PR, you aren't just merging code; you are merging knowledge. Ensure your contributions are accessible to others.

##  Getting Started

1.  **Fork the repository** on GitHub.
2.  **Clone your fork** locally:
    ```bash
    git clone https://github.com/fderuiter/math-explorer.git
    cd math-explorer
    ```
3.  **Create a branch** for your feature or fix:
    ```bash
    git checkout -b feature/amazing-new-math
    ```

##  Workflow

```mermaid
graph TD
    A[Start] --> B[Fork Repository]
    B --> C[Create Branch]
    C --> D[Code Changes]
    D --> E{Tests Pass?}
    E -->|No| D
    E -->|Yes| F[Write Docs]
    F --> G[Submit PR]
    G --> H{Review}
    H -->|Changes Requested| D
    H -->|Approved| I[Merge]
```

## Git Hooks & Guardrails

To ensure immediate feedback and high integrity, standards (such as file-length limits in core directories) are enforced automatically at the commit level. Running the project setup command (`cargo run -p xtask -- setup`) automatically configures a Git pre-commit hook that runs the centralized verification suite. This minimizes CI failures and maintains architectural constraints.

##  Testing

We take reliability seriously. Before submitting, ensure all tests pass.

```bash
# Run tests for the core library
cargo test --package math_explorer

# Run checks for the GUI (if applicable)
cargo check --package math_explorer_gui
```

If you add new functionality, **you must add tests**.
- **Unit Tests**: Place them in the same file as the code, in a `mod tests` module.
- **Integration Tests**: Place them in the `tests/` directory.

##  Contributing to the GUI

The `math_explorer_gui` crate is built with **egui** and **eframe**.

*   **Structure:**
    *   UI code resides in `math_explorer_gui/src`.
    *   The GUI should primarily visualize and control logic defined in the core `math_explorer` library. Avoid implementing heavy mathematical logic directly in the GUI crate.
*   **Dependencies:** We maintain strict version compatibility between `egui`, `eframe`, and `egui_plot`. Check `Cargo.toml` before updating dependencies.

##  Documentation Style Guide

We follow the **Curator's Philosophy**: "Code explains HOW; Docs explain WHY."

*   **Public API**: Every new or updated public struct, enum, trait, and function must have a docstring (`///`). Our automated integration checks will fail if you submit undocumented public APIs.
*   **Legacy Exemptions**: To prevent delaying ongoing work, older undocumented modules are grandfathered in. If you are touching a legacy module that has no documentation, you may leave it exempt. Existing exemptions are declared at the module level using `#[allow(missing_docs)]`. **Do not** add crate-level exemptions (e.g., `#![allow(missing_docs)]` in `lib.rs`), as any new files must be properly documented.
*   **Examples**: Include runnable examples in your docstrings using generic code blocks or doctests.
*   **Clarity**: Avoid fluff words ("simple", "easy"). Be precise.
*   **Mental Compilation**: Ensure your examples are syntactically correct and import necessary dependencies.
*   **Update the "Front Door"**: If you touch a core feature, update the `README.md` summary to reflect it.

Example:

```rust,ignore
/// Checks if a number is a prime number.
///
/// # Arguments
///
/// * `n` - The number to check.
///
/// # Returns
///
/// `true` if `n` is prime, `false` otherwise.
///
/// # Example
///
/// ```
/// use math_explorer::pure_math::number_theory::is_prime;
/// assert!(is_prime(5));
/// assert!(!is_prime(4));
/// ```text
pub fn is_prime(n: u64) -> bool { ... }
```

##  Submission Process

1.  **Commit your changes** with a descriptive message.
2.  **Push to your branch**.
3.  **Open a Pull Request**.
4.  **Wait for review**. We might ask for changes to code or documentation.

###  Documentation Checklist

Before marking your PR as ready, ensure you have:

- [ ] Added docstrings to all new public items.
- [ ] Included a runnable example for complex logic.
- [ ] Verified that `cargo test --doc` passes.
- [ ] Updated `README.md` if you added a new module or feature.
- [ ] Checked for broken links or typos.

Thank you for helping us fight Knowledge Rot!
