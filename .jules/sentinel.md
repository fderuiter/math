## 2025-02-27 - [Fix DoS panic in gradient_fast]
**Vulnerability:** Potential index out-of-bounds panics in `gradient_fast` numerical kernel leading to Denial of Service.
**Learning:** High-performance numerical kernels often use array indexing (e.g. `data[idx + 1]`) to bypass bounds checking overhead or assume earlier validation is sufficient. However, unchecked array access combined with mathematical operations on indices can result in panics if inputs are crafted to bypass checks or if arithmetic overflows.
**Prevention:** Use safe access methods like `.get().copied().unwrap_or(0.0)` paired with `checked_add`/`checked_sub` for index calculations, ensuring that out-of-bounds access safely falls back to a default value without triggering panics, maintaining both safety and robustness.
