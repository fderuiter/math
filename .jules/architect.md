## 2024-03-22 - [ProbabilityWinRatio Parameter Object]
Problem: The `calculate_win_probability` and `calculate_loss_probability` functions in `probability_win_ratio.rs` took 8 arguments, violating the "Too Many Arguments" code smell and triggering Clippy warnings.
Decision: Introduce a `ProbabilityWinRatioContext` struct (Builder/Context pattern) to encapsulate the scalar configuration parameters (`c`, `error_tolerance`, `s0_at_c`, `s1_at_c`).
Consequence: Improves readability and extensibility. Call sites need to update to construct this context object.


## 2025-12-13 - [Favoritism Configuration Decomposition]
Problem: The  struct was a "God Struct" with 21 flat fields mixing distinct domains (temporal, social, gifts, personality).
Decision: Decomposed  into cohesive sub-structs: , , , , , , .
Consequence: Breaking change for API consumers constructing  directly, but significantly improved readability and logical separation of concerns. Access paths are now hierarchical (e.g.,  vs ).

## 2024-10-24 - [Favoritism Configuration Decomposition]
Problem: The `FavoritismInputs` struct was a "God Struct" with 21 flat fields mixing distinct domains (temporal, social, gifts, personality).
Decision: Decomposed `FavoritismInputs` into cohesive sub-structs: `TimeParams`, `GiftParams`, `ContactParams`, `PersonalityParams`, `SocialParams`, `ComplimentParams`, `FamilyParams`.
Consequence: Breaking change for API consumers constructing `FavoritismInputs` directly, but significantly improved readability and logical separation of concerns. Access paths are now hierarchical (e.g., `inputs.personality.wealth` vs `inputs.wealth`).

## 2024-05-23 - [Configuration Object for CERA Model]
Problem: The CERA model in `src/climate/cera.rs` relied on hardcoded `const` values (`IN_CHANNELS`, `NUM_LEVELS`, etc.) for its architecture, making it impossible to reuse the model for different datasets or configurations without code changes.
Decision: Applied the "Configuration Object" pattern by refactoring `CeraConfig` to include all architectural dimensions (`in_channels`, `latent_channels`, `num_levels`, etc.) and updating `Cera::new` and internal methods to use these dynamic values.
Consequence: The model is now flexible and can be instantiated with arbitrary dimensions. The trade-off is slightly more verbose initialization in tests and usage code, requiring all dimensions to be specified in the config.


## 2024-10-25 - [Self-Calibration Module Extraction]
Problem: The `math_explorer/src/ai/self_calibration.rs` file contained multiple internal modules (`types`, `scoring`, `temperature`, `training`) within a single file, violating the "One Class/Concept Per File" principle and making the file difficult to navigate.
Decision: Extract internal modules into their own files within a new `math_explorer/src/ai/self_calibration/` directory.
Consequence: Improves modularity and discoverability. The public API is preserved via `mod.rs` re-exports, so no breaking changes for external consumers.

## 2025-05-15 - [Refactoring LoraHub to Ensemble Pattern]
**Problem:** `lorahub.rs` used a procedural style with loose functions (`combine_loras`), passing large state dictionaries as arguments repeatedly and mixing logic with simple data structures.
**Decision:** Extracted `lorahub` into a submodule and encapsulated logic in `LoraEnsemble` struct.
**Consequence:** Improved cohesion; state validation happens once (or is cleaner); API is more object-oriented.
