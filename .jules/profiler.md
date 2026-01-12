
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
