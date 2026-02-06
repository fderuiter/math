
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

## 2026-11-01 - [Optimization] **Bottleneck:** Ising Model Metropolis Step Overhead **Strategy:** Lookup Table & Batching **Gain:** 85% Speedup (13.9M/s -> 25.7M/s)

**Bottleneck:**
The `SpinLattice::metropolis_step` function was called in a tight loop, causing:
1. `rand::thread_rng()` initialization overhead for every single spin flip.
2. Expensive `exp()` calculations for Boltzmann factors in the hot path.
3. Repetitive neighbor indexing.

**Strategy:**
Implemented `SpinLattice::evolve` for batched updates.
1. **Lookup Table:** Precomputed `exp(-beta * dE)` for all 10 possible local configurations (spin state vs neighbor sum), removing `exp()` from the loop.
2. **RNG Reuse:** Instantiated the RNG once per batch.
3. **Batching:** Processed multiple steps in a single function call to amortize setup costs.

**Gain:**
Benchmark `bench_ising_custom` (10M iterations, 100x100 grid):
- Before: ~0.72s (13.9 M/s)
- After: ~0.39s (25.7 M/s)
- Speedup: ~1.85x

## 2026-11-02 - [Optimization] **Bottleneck:** Ising Model Coordinate Arithmetic & RNG **Strategy:** Precomputed Neighbor Table & RNG Batching **Gain:** ~100% Speedup (14.7 M/s -> 29.4 M/s)

**Bottleneck:**
The `SpinLattice::evolve` function (the optimized batch update) still had significant overhead in its hot loop:
1. Two calls to `rng.gen_range` per iteration (for x and y coordinates).
2. Repeated coordinate arithmetic (multiplication, addition) and conditional branching to handle periodic boundary conditions for every neighbor of every site.
3. This amounted to ~4 branches and ~10 arithmetic ops per spin flip attempt.

**Strategy:**
1. **Precomputed Neighbors:** Moved the neighbor index calculation out of the simulation loop. Added a `neighbors` field (`Vec<[usize; 4]>`) to `SpinLattice`, populated once during initialization.
2. **Reduced RNG Calls:** Replaced the two coordinate RNG calls with a single `rng.gen_range(0..count)` to pick a flattened index.
3. **Table Lookup:** The hot loop now performs a direct memory lookup `self.neighbors[idx]` to get all 4 neighbors instantly, eliminating all coordinate math and boundary checks.

**Gain:**
Benchmark `bench_ising_custom` (10M iterations, 100x100 grid):
- Before: ~14.70 M/s
- After: ~29.41 M/s
- Speedup: ~2.00x (100% improvement)

## 2024-05-21 - [Optimization] **Bottleneck:** Repeated Grid Calculation & Allocations in MFG Solver **Strategy:** Precomputation & In-Place Matrix Ops **Gain:** 26% Time Saved (489ms -> 360ms)

**Bottleneck:**
The `FixedPointSolver::solve` function for Mean Field Games contained several redundant operations in its hot loops:
1. `x = min + i * dx` was recomputed for every spatial point, at every time step, in every iteration (`nx * nt * iterations` times).
2. Initialization of the `m` matrix and its propagation (`copy m0 to all t`) was done via manual nested loops with bounds checking.
3. Normalization logic used manual loops instead of vectorized operations.

**Strategy:**
1. **Precomputation:** Calculated spatial `x` values once into a `Vec<f64>` and reused them via simple indexing.
2. **Vectorization:** Replaced manual initialization loops with `nalgebra`'s `column_mut(n).copy_from(&col)` and `scale_mut()`, which leverage contiguous memory copies (memcpy) and optimized BLAS-like operations.
3. **Allocation Reduction:** Avoided repeated index calculations and bounds checks implicit in the manual loops.

**Gain:**
Benchmark `bench_mfg` (200x2000 grid, 100 iterations):
- Before: ~488.75ms
- After: ~359.67ms
- Speedup: ~26.4%

## 2026-11-03 - [Optimization] **Bottleneck:** Gillespie Stochastic Solver Allocations **Strategy:** Zero-Allocation Buffer Reuse **Gain:** ~38% Time Saved (28.4ms -> 17.6ms)

**Bottleneck:**
The `GillespieSolver::step` loop was allocating a new `Vec<f64>` for propensities in every iteration via the `StochasticSystem::propensities` return value.
For a simulation with 500,000 steps, this meant 500,000 heap allocations and deallocations.

**Strategy:**
1. **Trait Change:** Modified `StochasticSystem::propensities` to accept a mutable buffer `&mut Vec<f64>` instead of returning a new vector.
2. **Buffer Reuse:** Added a `buffer` field to `GillespieSolver` which is reused across steps. The buffer is cleared at the start of each step.
3. **Refactor:** Updated `SIRModel` to append rates to the provided buffer.

**Gain:**
Benchmark `bench_stochastic` (500,000 steps):
- Before: ~28.4ms (56ns/step)
- After: ~17.6ms (35ns/step)
- Speedup: ~38%

## 2024-10-26 - [Optimization] **Bottleneck:** ReplicatorDynamics::derivative allocated 2 vectors per call. **Strategy:** Implemented derivative_in_place using output buffer as scratchpad. **Gain:** 20% simulation speedup for N=10 systems (Zero allocations per RK4 step).

## 2026-05-27 - [Optimization] **Bottleneck:** Turing System Memory Bandwidth **Strategy:** Fused Diffusion-Reaction Loop **Gain:** 45% Time Saved (44.1µs -> 24.0µs)

**Bottleneck:**
The `TuringSystem::step` function was performing two passes over the state arrays:
1. `diffusion.apply`: Reads `u, v`, writes `next_u, next_v` (calculating Laplacian).
2. `step` loop: Reads `next_u, next_v` (diffusion term), Reads `u, v`, writes `next_u, next_v` (final state).
Total memory operations: 6 reads + 4 writes per element.

**Strategy:**
1. **Fused Loop:** Extended `SpatialDiffusion` trait with `apply_step`.
2. **Implementation:** Specialized `FiniteDifference1D::apply_step` to compute the Laplacian and immediately apply the reaction and time integration in the same loop.
3. **Register Reuse:** The diffusion term is computed in registers and consumed immediately, avoiding writing/reading it to/from memory.
New memory operations: 2 reads + 2 writes per element (assuming neighbors are cached).

**Gain:**
Benchmark `bench_morphogenesis` (10,000 steps, size 10,000):
- Before: 44.11µs/step
- After: 23.96µs/step
- Speedup: ~45% (Almost 2x)
