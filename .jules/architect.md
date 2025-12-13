## 2024-03-22 - [ProbabilityWinRatio Parameter Object]
Problem: The `calculate_win_probability` and `calculate_loss_probability` functions in `probability_win_ratio.rs` took 8 arguments, violating the "Too Many Arguments" code smell and triggering Clippy warnings.
Decision: Introduce a `ProbabilityWinRatioContext` struct (Builder/Context pattern) to encapsulate the scalar configuration parameters (`c`, `error_tolerance`, `s0_at_c`, `s1_at_c`).
Consequence: Improves readability and extensibility. Call sites need to update to construct this context object.

## 2024-05-23 - [Configuration Object for CERA Model]
Problem: The CERA model in `src/climate/cera.rs` relied on hardcoded `const` values (`IN_CHANNELS`, `NUM_LEVELS`, etc.) for its architecture, making it impossible to reuse the model for different datasets or configurations without code changes.
Decision: Applied the "Configuration Object" pattern by refactoring `CeraConfig` to include all architectural dimensions (`in_channels`, `latent_channels`, `num_levels`, etc.) and updating `Cera::new` and internal methods to use these dynamic values.
Consequence: The model is now flexible and can be instantiated with arbitrary dimensions. The trade-off is slightly more verbose initialization in tests and usage code, requiring all dimensions to be specified in the config.
