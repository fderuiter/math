## 2025-12-12 - [Module Extraction: AI Self-Calibration]
**Problem:** `lib.rs` was becoming a "God File" containing mixed domains, specifically a large inline `self_calibration` module unrelated to the root library concerns.
**Decision:** Extracted the `self_calibration` module into its own file `src/ai/self_calibration.rs`.
**Consequence:**
- `lib.rs` is reduced to an entry point and re-export hub.
- `self_calibration` is correctly grouped under the `ai` domain.
- Public API `math_explorer::self_calibration` is preserved via re-export.
