
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

## 2026-10-15 - [Optimization] **Bottleneck:** RK4 Solver Allocation Churn **Strategy:** Zero-Allocation In-Place Solver **Gain:** 45% Time Saved (31.2ms -> 17.3ms)

**Bottleneck:**
The `RungeKutta4` solver and `VecState` vector operations relied on standard operator overloading (`Add`, `Mul`) which returned new allocated vectors (`Vec::new`) for every intermediate step.
For a system of size 10,000, a single RK4 step performed ~12 large heap allocations/copies.
- `state + k*dt` (Allocates result)
- `k1`, `k2`, `k3`, `k4` (Allocated by `derivative`)

**Strategy:**
1. **Trait Refactor:** Extended `VectorOperations` with `scale_add(&mut self, other, scale)` and `copy_from(&mut self, other)` to allow in-place mutation.
2. **Buffer Reuse:** Updated `RungeKutta4` to allocate workspace vectors (`k`, `tmp`) only once per step and reuse them.
3. **In-Place Arithmetic:** Replaced creating new vectors with `scale_add`, avoiding intermediate allocations.
4. **Manual Specialization:** Implemented optimized `scale_add` for `VecState` and `DVector` using iterators and slices.

**Gain:**
Benchmark (100 steps, system size 10,000):
- Before: 31.19ms
- After: 17.29ms
- Speedup: ~45% (Allocation reduction)

## 2026-10-27 - [Optimization] **Bottleneck:** Turing System Grid Iteration **Strategy:** Unsafe Indexing & Strength Reduction **Gain:** 44% Time Saved (482ms -> 270ms)

**Bottleneck:**
The `TuringSystem::step` function was performing a 1D Laplacian convolution with:
1. Checked indexing (`self.u[i]`) inside the hot loop.
2. Repeated floating-point division (`/ dx_sq`).
3. Closure invocation overhead.
4. `u.powi(2)` inside the reaction kinetics.

**Strategy:**
1. **Unsafe Indexing:** Refactored the loop to use `get_unchecked` within the safe bounds `1..n-1`.
2. **Strength Reduction:** Pre-calculated `inv_dx_sq` to replace division with multiplication.
3. **Inlining:** Manually inlined the update logic into the loop to ensure optimal register usage.
4. **Simplification:** Replaced `powi(2)` with `u * u` in `SchnakenbergKinetics`.

**Gain:**
Benchmark `bench_morphogenesis` (1000 iterations, size 100,000):
- Before: 482.02ms
- After: 270.08ms
- Speedup: ~1.78x (44% reduction)

## 2026-01-17 - [Optimization] **Bottleneck:** Logistic Map Memory Churn **Strategy:** Pre-allocation **Gain:** ~10% Time Saved + Stability

**Bottleneck:**
The `generate_bifurcation_diagram` function in `math_explorer/src/physics/chaos/logistic.rs` initialized `points` with `Vec::new()`, causing repeated reallocation as it grew to ~5 million points (80MB).
Benchmark showed high variance (128ms - 926ms) due to allocator pressure.

**Strategy:**
Implemented `Vec::with_capacity(capacity)` where `capacity = (steps + 1) * ATTRACTOR_POINTS`.
This eliminates re-allocations and `memcpy` operations.

**Gain:**
Benchmark `bench_logistic` (100,000 steps):
- Before: ~128ms (best), often >400ms (worst)
- After: ~115ms (consistent)
- Speedup: ~10% on CPU bound, significantly higher stability on system load.
