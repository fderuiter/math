## 2024-05-23 - [Optimization] **Bottleneck:** O(N^2) Correlation Dimension for Fractal Analysis **Strategy:** Spatial Pruning (Sort by X) **Gain:** 6.5x speedup (260ms vs 1.68s) for epsilon=0.05
## 2024-10-24 - [Optimization] **Bottleneck:** O(N^4) Naive 2D DFT in MRI Simulation **Strategy:** Row-Column Decomposition (Separable DFT) **Gain:** 39x speedup (3.2s -> 0.08s) for 64x64 matrix
## 2025-12-17 - [Optimization] **Bottleneck:** O(N^3) Gradient Calculation in Marching Cubes **Strategy:** Lazy Evaluation (Compute only on surface) **Gain:** 6.2x speedup (213ms -> 34ms)
## 2025-05-13 - [Optimization] **Bottleneck:** O(N^2) Partition Function (f_k) Calculation **Strategy:** Euler's Pentagonal Number Theorem (O(sqrt(N))) **Gain:** 41,000x speedup (3.23s -> 78µs) for precision=100000
## 2025-05-20 - [Optimization] **Bottleneck:** Isosurface Extraction Memory/CPU Overhead **Strategy:** Linear Indexing + Inlining + Pre-allocation **Gain:** 20% speedup (277ms -> 220ms)
## 2025-06-15 - [Optimization] **Bottleneck:** Allocations and Branching in Turing Pattern Step **Strategy:** Double Buffering + Loop Split **Gain:** 64% speedup (6.55ms -> 2.33ms)
