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

## 2025-12-13 - Extracted QSeries Module
**Problem:** `math_explorer/src/pure_math/number_theory/partitions.rs` coupled a general-purpose power series implementation (`QSeries`) with specific partition function logic, violating separation of concerns and limiting reuse.
**Decision:** Applied **Module Extraction**. Moved `QSeries` and its arithmetic implementations to `math_explorer/src/pure_math/number_theory/q_series.rs`. Re-exported `QSeries` in `partitions.rs` to maintain backward compatibility.
**Consequence:** `QSeries` is now a reusable component for other number theory domains (e.g., modular forms, elliptic curves). `partitions.rs` is focused on partition theory. API remains stable.


## 2024-10-24 - Extract GRPO Metrics and Rewards
**Problem:** `math_explorer/src/applied/grpo/formulas.rs` is a mixed-domain file containing logic for string metrics (BLEU), reinforcement learning rewards, and core GRPO optimization formulas. This violates separation of concerns.
**Decision:** Apply "Module Extraction". Split `formulas.rs` into `metrics.rs` (string processing), `rewards.rs` (RL signals), and keep `formulas.rs` for core optimization objectives.
**Consequence:** Improves cohesion and makes it easier to reuse metrics or reward functions independently. Adds a few more files to the file tree.

## 2025-05-27 - Consolidated AI Primitives and Transformer Architecture
**Problem:** The `src/ai` module lacked cohesion, with transformer components (`attention.rs`, `feed_forward.rs`, `positional_encoding.rs`) floating at the top level alongside unrelated modules. Additionally, reusable primitives like `relu`, `softmax`, and `AddRowVector` were scattered or duplicated, hindering reuse.
**Decision:**
1. **Module Extraction & Consolidation:** Moved transformer-specific components into `src/ai/transformer/`.
2. **Refactoring:** Extracted reusable math primitives into `src/ai/activations.rs` and `src/ai/utils.rs`.
3. **Refactoring:** Updated `FeedForward` and `MultiHeadAttention` to use these central primitives.
**Consequence:** The `ai` module structure now reflects the logical hierarchy. Transformer components are encapsulated. Primitives are reusable across other AI models (e.g., SDS, future Neural Networks). Backward compatibility is maintained via public modules, though internal structure is cleaner.

