
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

## 2026-10-28 - [Optimization] **Bottleneck:** Redundant Memory Loads in Turing System **Strategy:** Sliding Window / Register Rotation **Gain:** ~6% Time Saved (265ms -> 250ms)

**Bottleneck:**
The `TuringSystem::step` function's hot loop was performing a 3-point stencil convolution by loading 3 values (`i-1`, `i`, `i+1`) from memory for both `u` and `v` arrays in every iteration.
Total loads per iteration: 6 floats.
Most of these loads were redundant (e.g., `u[i+1]` in iteration `i` becomes `u[i]` in iteration `i+1`).

**Strategy:**
Implemented a "Sliding Window" (Scalar Replacement) optimization.
1. Maintained `prev` and `curr` values in local variables (registers).
2. In each iteration, only loaded `next` (`i+1`) from memory.
3. Updated `prev = curr` and `curr = next` at the end of the loop.
4. Reduced memory loads from 6 to 2 per iteration.

**Gain:**
Benchmark `bench_morphogenesis` (1000 iterations, size 100,000):
- Before: ~265.58ms
- After: ~250.46ms
- Speedup: ~6%

## 2026-10-29 - [Optimization] **Bottleneck:** Isosurface Mesh Reallocation **Strategy:** Improved Heuristic Pre-allocation **Gain:** ~3.4% Time Saved (26.5ms -> 25.6ms)

**Bottleneck:**
The `extract_isosurface` function uses a conservative heuristic (`2 * N^2`) for pre-allocating the triangle buffer.
For complex or dense isosurfaces (like the tested Sphere SDF), the actual triangle count exceeded this estimate, causing the `Vec` to reallocate and copy data (likely doubling capacity).
Reallocating a large buffer (~5MB) is expensive.

**Strategy:**
Increased the heuristic multiplier from 2 to 5 (`5 * N^2`).
This provides sufficient capacity for dense meshes, avoiding runtime reallocations while keeping memory usage within reasonable bounds (stack/heap tradeoff).
This aligns with the "The Reserve" Profiler move.

**Gain:**
Benchmark `profile_isosurface` (Sphere SDF 128x128x128):
- Before: ~26.5ms
- After: ~25.6ms
- Speedup: ~3.4% (Eliminated reallocations)

## 2026-10-30 - [Optimization] **Bottleneck:** Redundant Initialization & Branches in Marching Cubes **Strategy:** Deferred Init & Branchless Logic **Gain:** ~7.2% Time Saved (27.6ms -> 25.6ms)

**Bottleneck:**
The `extract_isosurface` inner loop runs 2 million times (for 128^3).
1. `corner_pos` and `corner_normals` (48 floats) were zero-initialized for *every* voxel, even though ~80% are empty and skipped immediately.
2. `cube_index` calculation involved 8 conditional branches (`if v < threshold`) per voxel, causing potential pipeline stalls.

**Strategy:**
1. **Deferred Initialization:** Moved the declaration and initialization of `corner_pos` and `corner_normals` *after* the `edge_flags == 0` check. This eliminates initialization overhead for empty voxels.
2. **Branchless Logic:** Replaced `if` checks with bitwise arithmetic (`((v < threshold) as usize) * bit`) to compute `cube_index` without branching.
3. **Direct Array Init:** Initialized `corner_values` directly from loaded data instead of zero-filling then assigning.

**Gain:**
Benchmark `profile_isosurface` (Sphere SDF 128x128x128):
- Before: ~27.6ms
- After: ~25.6ms
- Speedup: ~7.2%
## 2026-10-30 - [Optimization] **Bottleneck:** Matrix Allocation in LoRA Combination **Strategy:** Zero-Allocation Slice Iteration **Gain:** 91% Time Saved (729ms -> 66ms)

**Bottleneck:**
The `LinearCombinationStrategy` for LoRA ensembles was performing heavy matrix allocations inside the accumulation loop.
- `tensor * weights[i]` allocated a new temporary `DMatrix` for every layer of every module.
- `*final_tensor += ...` performed addition, but the intermediate product allocation was the killer.
For 10 modules of size 1000x1000, this resulted in significant memory traffic (~270MB).

**Strategy:**
Refactored the inner loop to use "The Zero-Copy" approach with `nalgebra`.
1. Accessed the underlying data using `as_mut_slice()` and `as_slice()`.
2. Replaced the matrix multiplication `tensor * weight` with an in-place iterator loop: `*f += *t * weight`.
3. Eliminated all intermediate matrix allocations during the accumulation phase.

**Gain:**
Benchmark `bench_lorahub` (10 modules, 1000x1000 matrices):
- Before: 728.78ms
- After: 66.35ms
- Speedup: ~11x (91% reduction)
