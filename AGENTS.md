# Agent Instructions for Mathematical Implementation and Academic Composition

This document outlines the mandatory procedure for developing a Rust implementation and an accompanying academic paper within the `math_explorer` ecosystem. 

**Core Philosophy:** This project prioritizes **Separation of Concerns (SoC)**, **Type Safety**, **Determinism**, and **Academic Rigor**. We actively avoid "God Files" and "Primitive Obsession."

---

### **1. Contextual Analysis & Architectural Alignment**

Before generating code or text, you must ground your work in the project's existing engineering standards.

* **Consult the Engineering Journals (`.jules/`)**: 
    * Review `systems_core.md` for established design patterns (e.g., Strategy Pattern for solvers, Builder Pattern for complex structs).
    * Review `architect.md` to understand domain decomposition and module extraction standards.
    * Review `mason.md` for architectural constraints (e.g., Dependency Injection for RNG).
* **Analyze the Source Material**: Deconstruct the provided mathematical framework into core components (State, Dynamics, Solvers, Statistics).
* **Identify Reusable Abstractions**: Determine if existing traits (e.g., `OdeSystem`, `Solver`, `VectorOperations`, `ReactionKinetics`) can be leveraged or extended.

---

### **2. Comprehensive Design & Scoping**

Produce a detailed blueprint. We do not write code without a plan.

* **Rust Architecture Strategy (`math_explorer/`)**:
    * **Module Structure**: Propose a directory-based hierarchy. **Explicitly forbid "God Files"** (monolithic files mixing unrelated domains).
    * **Data Modeling**: 
        * Plan for **Strong Typing** (Newtypes) to avoid Primitive Obsession (e.g., `struct Kelvin(f64)` instead of raw `f64`).
        * Define `struct`s for state management and `trait`s for interchangeable logic.
    * **Interface Design**:
        * Use the **Strategy Pattern** for algorithms that might change (e.g., numerical integrators, sorting, selection).
        * Use the **Builder Pattern** for complex model initialization to ensure validation.
        * Plan for **Dependency Injection**, specifically for Random Number Generators (RNG) to ensure test determinism.

* **GUI Integration Strategy (`math_explorer_gui/`)**:
    * **Roadmap Alignment**: Check `todo_gui.md` to see if the new module has a planned visualization. If not, propose one.
    * **Separation of Logic**: Ensure the core logic resides in `math_explorer` and only visualization/control code is added to `math_explorer_gui`. The GUI should not contain simulation logic.

* **Academic Paper Outline (`papers/`)**:
    * Define a section-by-section LaTeX structure.
    * Identify necessary citations and plan the `.bib` file entries to support the mathematical claims.

---

### **3. Targeted Implementation**

Execute the design using modern Rust idioms and the project's specific coding standards.

* **Rust Implementation Standards**:
    * **Type Safety**: Enforce validity at the type level. Use `Result` for fallible operations.
    * **Generic Solvers**: Decouple models from solvers. Models should implement traits (like `OdeSystem`) rather than containing hardcoded integration loops.
    * **Determinism**: Functions involving randomness must accept an injected RNG (`&mut R: Rng`) rather than using `thread_rng()` internally.
    * **Error Handling**: Create domain-specific error types (e.g., `thiserror` or custom enums) rather than stringly-typed errors.

* **Paper Composition**:
    * Write with high academic rigor in `papers/`.
    * Ensure every mathematical claim is backed by the implementation or a citation.
    * Manage references via BibTeX, ensuring all prior art is credited.

---

### **4. Critical Safety & Quality Standards (The NASA Power of 10)**

To ensure reliability and verifiability, we adopt an adaptation of NASA's "Power of 10" rules for this codebase. Adherence to these rules is mandatory for core logic.

1.  **Simple Control Flow**:
    *   **Rule**: Restrict code to very simple control flow constructs.
    *   **Implementation**: Do not use recursion; use iterative solutions. Avoid complex `break`/`continue` logic where a simple iterator would suffice. No `goto` (obviously).

2.  **Fixed Loop Bounds**:
    *   **Rule**: All loops must have fixed upper bounds.
    *   **Implementation**: Ensure iterators are finite. When using `while` loops, include a safety counter to prevent infinite execution (e.g., `let mut safety = 0; while condition && safety < MAX_ITER { ... safety += 1; }`).

3.  **No Dynamic Memory Allocation (Post-Init)**:
    *   **Rule**: Do not use dynamic memory allocation after initialization.
    *   **Implementation**: Minimize dynamic allocation during the core simulation loop. Pre-allocate `Vec` capacities (`Vec::with_capacity`). Avoid creating new heap-allocated structures inside hot loops.

4.  **Limit Function Length**:
    *   **Rule**: No function should be longer than what can be printed on a single sheet of paper (approx. 60 lines).
    *   **Implementation**: Decompose large functions into smaller, testable helper functions. If a function is too long, it likely violates the Single Responsibility Principle.

5.  **Assertion Density**:
    *   **Rule**: The assertion density of the code should average to a minimum of two assertions per function.
    *   **Implementation**: Use `assert!` to enforce invariants (e.g., "mass must be positive") and `debug_assert!` for performance-critical checks. Validate function inputs and state consistency rigorously.

6.  **Small Data Scope**:
    *   **Rule**: Data objects must be declared at the smallest possible level of scope.
    *   **Implementation**: Avoid `static` or global state. Variables should be local to their usage block. Pass data explicitly via arguments rather than relying on shared mutable state.

7.  **Check Return Values**:
    *   **Rule**: The return value of non-void functions must be checked by each calling function.
    *   **Implementation**: **Never ignore `Result`**. Use `?` propagation or handle errors explicitly. Do not use `let _ = ...` to suppress errors. Compiler warnings about unused results must be resolved.

8.  **Limited Preprocessor Use**:
    *   **Rule**: The use of the preprocessor must be limited.
    *   **Implementation**: Avoid complex macros and conditional compilation (`#[cfg]`) inside function bodies. Use generic traits and polymorphism instead of macros where possible.

9.  **Restricted Pointer Use**:
    *   **Rule**: The use of pointers should be restricted.
    *   **Implementation**: Avoid `unsafe` blocks and raw pointers unless absolutely necessary for FFI or performance (with rigorous justification). Prefer safe references and smart pointers (`Box`, `Rc`, `Arc`).

10. **Compile with All Warnings**:
    *   **Rule**: All code must be compiled with all compiler warnings enabled.
    *   **Implementation**: Treat warnings as errors. Run `cargo clippy -- -D warnings` regularly. Zero tolerance for compiler warnings.

---

### **5. Verification & Validation**

Your work is not complete until it is proven correct and robust.

* **Testing Strategy**:
    * **Unit Tests**: Test core logic in isolation.
    * **Deterministic Integration Tests**: Write tests that inject a seeded RNG to verify reproducible behavior.
    * **Regression Check**: Run the full `math_explorer` test suite (`cargo test -p math_explorer`) to ensure no breaking changes to existing modules.
    * **GUI Verification**: If GUI components were added, verify compilation with `cargo check -p math_explorer_gui`.

* **Quality Review**:
    * Verify that no "God Files" were created.
    * Verify that `mod.rs` files properly re-export public interfaces to maintain backward compatibility where appropriate.
    * Check for "Primitive Obsession" (e.g., passing raw `Vec<f64>` where a `ContingencyTable` struct would be safer).

---

### **6. Journaling & Documentation**

You must document your engineering decisions to maintain the project's historical continuity.

* **Update Engineering Records**:
    * **`systems_core.md`**: Log any major refactors, pattern adoptions (e.g., "Extracted X Strategy"), or trade-offs made during implementation.
    * **`architect.md`**: Log any new domain modules or structural decompositions.
    * **`mason.md`**: Record any architectural violations found and fixed (e.g., "Decoupled Physics from Solver").
* **Documentation**: Ensure all public Rust items have docstrings (`///`).
