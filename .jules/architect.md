## 2025-12-12 - [Module Extraction: AI Self-Calibration]
**Problem:** `lib.rs` was becoming a "God File" containing mixed domains, specifically a large inline `self_calibration` module unrelated to the root library concerns.
**Decision:** Extracted the `self_calibration` module into its own file `src/ai/self_calibration.rs`.
**Consequence:**
- `lib.rs` is reduced to an entry point and re-export hub.
- `self_calibration` is correctly grouped under the `ai` domain.
- Public API `math_explorer::self_calibration` is preserved via re-export.

## 2025-05-24 - [Transformer Decomposition and AI Primitives]
**Problem:** `transformer.rs` was becoming a monolithic file (mixing Encoder, Decoder, LayerNorm, and model definition) and duplicated logic existed for activations (Softmax, ReLU) and utilities (AddRowVector) across `attention.rs` and `feed_forward.rs`.
**Decision:**
- Decomposed `transformer.rs` into `ai/transformer/{mod.rs, encoder.rs, decoder.rs}`.
- Extracted reusable primitives into `ai/activations.rs` (softmax, relu), `ai/normalization.rs` (LayerNorm), and `ai/utils.rs` (AddRowVector).
**Consequence:**
- Improved modularity and reusability of basic AI blocks (`LayerNorm`, activations).
- Reduced coupling in `attention.rs` and `feed_forward.rs`.
- `transformer` module is now easier to navigate and extend.
