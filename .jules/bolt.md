## 2024-05-13 - Restore native slice indexing in gradient_fast
**Learning:** Using safe access methods like `.get(idx).copied().unwrap_or(0.0)` in high-performance numerical kernels (e.g., `gradient_fast`) causes silent data corruption and severe performance regressions.
**Action:** Do not swallow out-of-bounds accesses in fast paths; ensure bounds are strictly validated by the caller and use direct native slice indexing.

## 2024-05-18 - Optimize multiple iterators via fold
**Learning:** Found separate `.min()` and `.max()` folds on the same dataset (`isis` array) in per-frame rendering code, causing redundant double iteration.
**Action:** Use a single `.fold((min, max), ...)` operation with a tuple accumulator to compute both min and max simultaneously, halving the iteration overhead.
## 2024-05-17 - Spike Analysis History Optimization
**Learning:** In highly frequent per-frame UI rendering loops (like egui simulations), maintaining a rolling window using `Vec::remove(0)` causes severe O(N) memory shifting bottlenecks, especially when the history vector grows large (e.g., up to 20,000 points).
**Action:** Replaced `Vec` with `std::collections::VecDeque` and used `pop_front()` for O(1) element removals, and `.back()` instead of `.last()`.

## 2024-05-20 - Optimize FRACTRAN iteration memory usage
**Learning:** FRACTRAN simulation evaluates `$N \leftarrow N \cdot \frac{a}{b}$` heavily in a loop. Converting `N` (rug::Integer) to `rug::Rational` for the product inside the loop was causing expensive heap allocations (`Rational::from(n.clone())` + cloning).
**Action:** Replace `Rational` arithmetic in the iteration loop with in-place `Integer` operations (`is_divisible`, `*=`, `/=`) to avoid allocating new rationals at every step. This simple change yielded a ~57% reduction in execution time for `FractranProgram::execute`.
## 2024-05-24 - Cache Matrix Transpose in Iterative Loops
**Learning:** In nalgebra, methods like `.transpose()` allocate and return a new matrix. Calling this inside an iterative loop (like a power iteration method) causes unnecessary reallocations every iteration, severely degrading performance.
**Action:** Always extract static matrix transformations (like transpose) out of iterative loops.
