## 2024-05-13 - Restore native slice indexing in gradient_fast
**Learning:** Using safe access methods like `.get(idx).copied().unwrap_or(0.0)` in high-performance numerical kernels (e.g., `gradient_fast`) causes silent data corruption and severe performance regressions.
**Action:** Do not swallow out-of-bounds accesses in fast paths; ensure bounds are strictly validated by the caller and use direct native slice indexing.

## 2024-05-18 - Optimize multiple iterators via fold
**Learning:** Found separate `.min()` and `.max()` folds on the same dataset (`isis` array) in per-frame rendering code, causing redundant double iteration.
**Action:** Use a single `.fold((min, max), ...)` operation with a tuple accumulator to compute both min and max simultaneously, halving the iteration overhead.

## 2024-05-16 - Prevent O(N) Array Shifts in Per-Frame UI Loops
**Learning:** Using `Vec::remove(0)` in frequently executed per-frame loops (like `step_simulation` in `SpikeAnalysisTool` keeping a rolling history window) causes severe O(N) memory shifting bottlenecks. This blocks the main thread in a highly responsive egui frontend.
**Action:** Always prefer `std::collections::VecDeque` with `pop_front()` for O(1) rolling time-series data or simulation history ring buffers inside `math_explorer_gui`.
