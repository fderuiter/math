
## 2026-01-12 - [Optimization] **Bottleneck:** Isosurface Extraction Gradient Calculation **Strategy:** Unsafe Interior Optimization **Gain:** ~12.5% Time Saved (184ms -> 161ms)

**Bottleneck:**
The `extract_isosurface` function (Marching Cubes) spends a significant amount of time calculating gradients using central differences.
1. `grid.get(x,y,z)` performs 3 bounds checks and 1 index calculation (multiplication).
2. `get_gradient` calls `grid.get` 6 times per vertex.
3. This overhead accumulates in the hot inner loop.

**Strategy:**
Implemented `get_gradient_interior`, a specialized helper function using `unsafe { *data.get_unchecked(idx) }`.
1. It bypasses bounds checks and coordinate-to-index multiplication.
2. It operates directly on the flat `Vec<f32>` using pre-calculated strides.
3. The main loop identifies "interior" zones (`1..width-1`, etc.) where neighbor access is guaranteed safe.
4. Falls back to the safe `get_gradient` for boundary voxels.

**Gain:**
Benchmark `bench_isosurface` (10 iterations on 100^3 grid):
- Before: 183.9ms
- After: 161.1ms
- Speedup: ~12.5%

## 2024-05-23 - [Optimization] **Bottleneck:** QSeries Multiplication Loop **Strategy:** Hoisting and Bounds Optimization **Gain:** ~60% Time Saved (3.54ms -> 1.43ms)

**Bottleneck:**
The `QSeries::mul` function performs polynomial multiplication with nested loops.
1. The inner loop condition `if i + j < precision` introduced a branch misprediction penalty.
2. `self.coeffs[i].clone()` was repeated inside the inner loop for every `j`, causing redundant memory access and cloning overhead.

**Strategy:**
1. **Hoisting:** Moved `self.coeffs[i].clone()` out of the inner loop, so it is executed only once per outer iteration.
2. **Loop Bounds:** Calculated the exact inner loop limit `min(len2, precision - i)` to eliminate the `if` check inside the hot path.

**Gain:**
Benchmark `profile_qseries` (10 iterations on length 2000 series):
- Before: ~3.54ms
- After: ~1.43ms
- Speedup: ~2.5x (~60% reduction)
