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

## 2025-05-21 - [Sorting Algorithms Module Extraction]
Problem: `math_explorer/src/applied/algorithms/sorting.rs` was a large file (~420 lines) mixing statistical tracking, information theory, comparison sorts, and non-comparison sorts, violating the Single Responsibility Principle.
Decision: Extracted `sorting.rs` into a module directory `math_explorer/src/applied/algorithms/sorting/` with dedicated submodules: `stats.rs`, `theory.rs`, `elementary.rs`, `divide_conquer.rs`, `heap.rs`, and `linear.rs`.
Consequence: Improves code organization and maintainability. The public API is preserved via `mod.rs` re-exports, ensuring backward compatibility.

## 2025-06-25 - [Decomposition of CERA Climate Model]
**Problem:** `src/climate/cera.rs` was a "God File" combining model definition (`Cera`), configuration (`CeraConfig`), and training loop logic (`train`, `optimizer_step`), violating Separation of Concerns.
**Decision:** Extracted `CeraConfig` to `src/climate/config.rs` and training logic to `src/climate/training.rs` (introducing `CeraTrainer`). `Cera` struct remains in `cera.rs` as a pure model definition.
**Consequence:** Improved modularity and testability. Training logic is now decoupled from the model architecture. Callers must use `CeraTrainer` to train the model, which is a cleaner API.

## 2025-06-25 - [Freesurfer Module Extraction]
**Problem:** `src/applied/freesurfer/mod.rs` was a "God Module" containing four distinct mathematical domains: surface mesh operations, Bayesian segmentation, cortical thickness geometry, and GLM statistics.
**Decision:** Extracted these domains into separate submodules: `surface.rs`, `segmentation.rs`, `thickness.rs`, and `glm.rs`. The `mod.rs` file now acts as a facade, re-exporting the public API.
**Consequence:** Significantly improved separation of concerns. Each module now focuses on a single mathematical domain, making the code easier to test, read, and maintain. Backward compatibility is preserved via re-exports.
## 2025-12-14 - [Favoritism Module Extraction]
**Problem:** `math_explorer/src/applied/favoritism/mod.rs` was a "God Module" containing all parameter struct definitions and the scoring logic, mixing data type definitions with algorithmic implementation.
**Decision:** Extract parameter structs to `types.rs` and scoring logic to `scoring.rs`, leaving `mod.rs` to handle re-exports.
**Consequence:** Improved file-level modularity and separation of concerns. Types and Logic are now physically separated, making the code easier to navigate and maintain.
