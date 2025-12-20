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

## 2025-12-19 - Battery Degradation Modernization
**Gap:** The `battery_degradation` module relied on deprecated, top-level functions (`n70`, `capacity`) and lacked context about units and model limitations.
**Strategy:**
1. Applied "Hub and Spoke" pattern to `applied/battery_degradation/mod.rs`.
2. Promoted `PowerLawModel` as the primary API with a Quickstart guide.
3. Added rich docstrings and examples to `model.rs` and `types.rs`.
**Outcome:** Improved type safety visibility (users now see `DepthOfDischarge` immediately) and clarified the physical constraints of the model.
