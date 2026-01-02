# Curator's Log - Documentation Decision Records (DDR)

## 2025-12-17 - Applied Mathematics Documentation Strategy
**Gap:** The `math_explorer::applied` module contains eclectic submodules (Favoritism, Clinical Trials, etc.) with zero context in `mod.rs`. Users have to guess what "cannibalism" or "favoritism" modules actually do.
**Strategy:** Implement a "Hub and Spoke" documentation pattern.
1. The `applied/mod.rs` acts as a catalog with one-line descriptions for every submodule.
2. The `favoritism` module gets a "Deep Dive" treatment (Theory + Diagram) as a gold standard example.
**Outcome:** Reduced cognitive load for users browsing the API; explicit humor/satire warnings for `favoritism`.

## 2025-12-17 - Crate-Level Documentation Revamp
**Gap:** The `math_explorer/README.md` was effectively "abandonware", listing incorrect modules (`algebra`, `number_theory`) and providing no usage context, confusing users who navigated directly to the crate folder.
**Strategy:** Mirror the Root README structure but scoped to the crate. Define clear boundaries: Root README = Project Vision; Crate README = Developer Implementation Details.
**Outcome:** Established a reliable "Front Door" for the library crate, matching the actual codebase state.

## 2025-12-18 - Clinical Trials Elevation & Cannibalism Clarification
**Gap:** `clinical_trials` was a hidden gem with full implementation but zero docs. `cannibalism` was a confusing placeholder.
**Strategy:**
1. Applied "Hub and Spoke" to `clinical_trials`: Added Mermaid workflow and full example.
2. Labeled `cannibalism` as "Theoretical" with a warning about placeholder status to manage user expectations.
**Outcome:** Bridged the gap between code reality and user perception. High-value code is now visible; low-value code is honestly labeled.

## 2025-12-18 - Biological Scales Visualization
**Gap:** The `biology` module lacked high-level structure. Users had to open individual files to understand that it covers everything from Enzymes (Molecular) to Evolution (Population).
**Strategy:**
1.  **Taxonomy:** Categorized modules into Molecular, Tissue, and Population scales in `mod.rs`.
2.  **Visuals:** Added a Mermaid diagram to `mod.rs` to show the hierarchy and emergence.
3.  **Deep Dive:** Enhanced `neuroscience.rs` with a state diagram for ion channels, making the Hodgkin-Huxley model accessible to non-biologists.
**Outcome:** Users can now navigate the biology module by "scale" rather than just by filename.
