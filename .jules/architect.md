# Architectural Decision Records

## 2024-12-12 - Extracted Transformer Module
**Problem:** `math_explorer/src/ai/transformer.rs` was a large file (12K) containing multiple distinct components (`Encoder`, `Decoder`, `Transformer`, `LayerNorm`, `EncoderLayer`, `DecoderLayer`), violating the Single Responsibility Principle and making navigation difficult.
**Decision:** Applied the **Module Extraction** pattern. Split the file into a directory `math_explorer/src/ai/transformer/` with submodules `layer_norm.rs`, `encoder.rs`, `decoder.rs`, and `model.rs`. Created a `mod.rs` to re-export types, preserving the original API.
**Consequence:** Improved cohesion and testability. The `transformer` module is now scalable and easier to read. Import paths remain backward compatible via re-exports.
