## 2024-05-13 - Restore native slice indexing in gradient_fast
**Learning:** Using safe access methods like `.get(idx).copied().unwrap_or(0.0)` in high-performance numerical kernels (e.g., `gradient_fast`) causes silent data corruption and severe performance regressions.
**Action:** Do not swallow out-of-bounds accesses in fast paths; ensure bounds are strictly validated by the caller and use direct native slice indexing.

## 2024-05-18 - Optimize multiple iterators via fold
**Learning:** Found separate `.min()` and `.max()` folds on the same dataset (`isis` array) in per-frame rendering code, causing redundant double iteration.
**Action:** Use a single `.fold((min, max), ...)` operation with a tuple accumulator to compute both min and max simultaneously, halving the iteration overhead.
## 2024-05-16 - O(N) Array shifting in high-frequency rendering loop

**Learning:** Using `Vec::remove(0)` to limit history arrays in hot simulation loops forces O(N) memory shifting. Given simulation frames execute multiple steps per render (e.g. 10x per frame) and array bounds were up to 20,000, this resulted in hundreds of thousands of redundant element copies per frame.
**Action:** Always use `VecDeque` for any time-series array in `math_explorer_gui` where we keep a rolling window (i.e. bounding max length), enabling `pop_front` which is O(1).
