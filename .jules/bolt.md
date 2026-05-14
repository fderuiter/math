## 2024-05-13 - Restore native slice indexing in gradient_fast
**Learning:** Using safe access methods like `.get(idx).copied().unwrap_or(0.0)` in high-performance numerical kernels (e.g., `gradient_fast`) causes silent data corruption and severe performance regressions.
**Action:** Do not swallow out-of-bounds accesses in fast paths; ensure bounds are strictly validated by the caller and use direct native slice indexing.
## 2026-05-14 - Eliminate DVector Allocations in High-Frequency UI Loops
**Learning:** Allocating `DVector` instances inside nested loops within egui's per-frame `show()` function creates severe performance bottlenecks and massive heap allocation overhead.
**Action:** Replaced dynamic vector allocations and library function calls (`mean_squared_error`) with manual scalar loops inside high-frequency rendering blocks.
