# Agent Instructions for Mathematical Implementation and Academic Composition

This document outlines the procedure for developing a Rust implementation and an accompanying academic paper based on a given mathematical framework. Adherence to these steps is mandatory to ensure a high-quality, robust, and academically sound submission.

### **1. Mathematical Analysis & Implementation Scoping**

Your first responsibility is to deeply understand the provided material and plan the project scope.

*   **Analyze the Source Material**: Systematically analyze the provided mathematical paper or specification. Your goal is to identify the core concepts, algorithms, and data models that form the foundation of the work.
*   **Scope the Implementation**: Based on your analysis, scope out the primary components required for a robust Rust implementation. All development for this part of the project will take place in the `math_explorer/` directory.
*   **Scope the Academic Paper**: Identify the key sections needed for a thorough academic paper that accurately represents the work. The paper will be developed in the `papers/` directory.
*   **Compile a Bibliography**: Compile all external sources, prior art, and foundational work that will require citation in the paper. This is a critical step for maintaining academic integrity.

***

### **2. Detailed Implementation & Paper Outline**

Before writing any implementation code or prose, you must produce a detailed outline. This outline serves as the blueprint for the project.

*   **Rust Project Structure**:
    *   Propose a clear **file and module structure** for the Rust project within `math_explorer/`. The design should prioritize clarity, maintainability, and scalability.
    *   Provide a clear description of the **key data structures and function signatures**. These must follow idiomatic Rust conventions (e.g., using `Result` for error handling, appropriate use of traits, etc.).

*   **LaTeX Paper Structure**:
    *   Propose a **section-by-section structure** for the LaTeX paper.
    *   This must include a plan for a "References" or "Bibliography" section to manage citations.

***

### **3. Targeted Implementation & Composition**

With an approved outline, proceed to the implementation and composition phase.

*   **Rust Implementation (`math_explorer/`)**:
    *   Implement the most direct and clean version of the solution as defined in your outline.
    *   Write clean, well-documented, and efficient code.
    *   Adhere strictly to modern Rust idioms and **coding best practices**, including:
        *   Robust error handling (e.g., `Result<T, E>`).
        *   Clear and descriptive variable and function names.
        *   A modular design that reflects the structure of the problem domain.

*   **Paper Composition (`papers/`)**:
    *   Compose the academic paper, upholding the highest standards of **academic integrity**.
    *   All concepts, data, and text drawn from other works **must be appropriately cited** within the text and fully detailed in the bibliography. Plagiarism is strictly forbidden.

***

### **4. Verification Through Testing & Review**

Validation is a critical final step. Your work is not complete until it has been thoroughly verified.

*   **Unit and Integration Testing**:
    *   **Write new, targeted tests** for the Rust implementation. These tests must verify the correctness of the core algorithms and handle critical edge cases.
    *   **Run the entire test suite** for the `math_explorer` project. All tests must pass before the project is considered complete. You must ensure your changes have not introduced any regressions.

*   **Final Submission Review**:
    *   **Meticulously review the final submission** before completion.
    *   **Code Quality**: Check the Rust code for quality, adherence to best practices, and clarity.
    *   **Academic Rigor**: Proofread the paper for academic rigor, citation accuracy, and to ensure all content is original and properly attributed. **Verify every citation.**
