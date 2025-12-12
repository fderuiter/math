# Architectural Decision Records

## 2024-12-12 - Extracted Transformer Module
**Problem:** `math_explorer/src/ai/transformer.rs` was a large file (12K) containing multiple distinct components (`Encoder`, `Decoder`, `Transformer`, `LayerNorm`, `EncoderLayer`, `DecoderLayer`), violating the Single Responsibility Principle and making navigation difficult.
**Decision:** Applied the **Module Extraction** pattern. Split the file into a directory `math_explorer/src/ai/transformer/` with submodules `layer_norm.rs`, `encoder.rs`, `decoder.rs`, and `model.rs`. Created a `mod.rs` to re-export types, preserving the original API.
**Consequence:** Improved cohesion and testability. The `transformer` module is now scalable and easier to read. Import paths remain backward compatible via re-exports.

## 2024-05-23 - Extracted Pharmacokinetics Module & Traitification
**Problem:** `math_explorer/src/applied/pharmacokinetics.rs` contained mixed logic for basic pharmacokinetic models (Bateman function), superposition logic, and composite models (Enantiomer, XR). The logic for superposition and two-pulse XR was tightly coupled to specific structs or duplicated.
**Decision:**
1.  **Module Extraction:** Moved the file to `math_explorer/src/applied/pharmacokinetics/` with submodules (`bateman`, `enantiomer`, `superposition`, `two_pulse`).
2.  **Traitification:** Introduced `PharmacokineticModel` trait with a `concentration(t)` method.
3.  **Generic Promotion:** Refactored `SuperpositionModel` and `TwoPulseModel` to be generic wrappers over any `PharmacokineticModel`.
4.  **Composition:** Refactored `EnantiomerModel` to use these generic components internally while maintaining its original API.
**Consequence:** The system is now composable (e.g., one can easily model superposition of any drug model). The code is more DRY (superposition logic is written once). Backward compatibility was strictly maintained.
