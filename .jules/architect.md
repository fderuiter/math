## 2024-03-22 - [ProbabilityWinRatio Parameter Object]
Problem: The `calculate_win_probability` and `calculate_loss_probability` functions in `probability_win_ratio.rs` took 8 arguments, violating the "Too Many Arguments" code smell and triggering Clippy warnings.
Decision: Introduce a `ProbabilityWinRatioContext` struct (Builder/Context pattern) to encapsulate the scalar configuration parameters (`c`, `error_tolerance`, `s0_at_c`, `s1_at_c`).
Consequence: Improves readability and extensibility. Call sites need to update to construct this context object.
