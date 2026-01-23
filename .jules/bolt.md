## 2024-05-21 - Isosurface Caching vs Sqrt Removal
**Learning:** Attempting to cache `[f32; 4]` using `Option` in a tight loop to reduce memory reads degraded performance (~5% slower), likely due to branch overhead and stack copying outweighing L1 cache hits.
**Action:** Prefer algorithmic simplifications (like removing redundant `sqrt`) over complex caching logic for small primitive types in hot loops.

## 2024-05-22 - Sliding Window in Marching Cubes
**Learning:** Implementing a sliding window to reuse voxel values and bitwise comparison masks reduced execution time by ~10% (from 25.3ms to 22.6ms). Reusing data between iterations (Right Face -> Left Face) reduced memory loads by 50% and float comparisons by 50%.
**Action:** Look for "overlapping window" patterns in grid/image processing loops where data from iteration `N` can be reused in iteration `N+1`.
