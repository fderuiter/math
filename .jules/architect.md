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
