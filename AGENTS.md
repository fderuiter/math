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

### **4. Verification & Validation**

Your work is not complete until it is proven correct and robust.

* **Testing Strategy**:
    * **Unit Tests**: Test core logic in isolation.
    * **Deterministic Integration Tests**: Write tests that inject a seeded RNG to verify reproducible behavior.
    * **Regression Check**: Run the full `math_explorer` test suite to ensure no breaking changes to existing modules.

* **Quality Review**:
    * Verify that no "God Files" were created.
    * Verify that `mod.rs` files properly re-export public interfaces to maintain backward compatibility where appropriate.
    * Check for "Primitive Obsession" (e.g., passing raw `Vec<f64>` where a `ContingencyTable` struct would be safer).

---

### **5. Journaling & Documentation**

You must document your engineering decisions to maintain the project's historical continuity.

* **Update Engineering Records**:
    * **`systems_core.md`**: Log any major refactors, pattern adoptions (e.g., "Extracted X Strategy"), or trade-offs made during implementation.
    * **`architect.md`**: Log any new domain modules or structural decompositions.
    * **`mason.md`**: Record any architectural violations found and fixed (e.g., "Decoupled Physics from Solver").
* **Documentation**: Ensure all public Rust items have docstrings (`///`).
