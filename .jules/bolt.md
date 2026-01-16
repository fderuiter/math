## 2024-05-21 - Isosurface Caching vs Sqrt Removal
**Learning:** Attempting to cache `[f32; 4]` using `Option` in a tight loop to reduce memory reads degraded performance (~5% slower), likely due to branch overhead and stack copying outweighing L1 cache hits.
**Action:** Prefer algorithmic simplifications (like removing redundant `sqrt`) over complex caching logic for small primitive types in hot loops.
