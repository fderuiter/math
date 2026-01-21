# Sentinel's Security Journal

## 2024-05-22 - Numerical Instability in Mathematical Models
**Vulnerability:** Unbounded inputs in `calculate_favoritism_score` caused Division by Zero and Infinite propagation (DoS risk).
**Learning:** Mathematical implementations often assume ideal inputs (e.g., $x_0 \neq 0$) and neglect software-level safety checks. In a Rust library, propagating `Inf` or `NaN` can crash downstream systems or cause undefined behavior in logic.
**Prevention:** Always sanitize inputs for mathematical functions (clamp denominators away from zero, clamp log inputs to positive domain) before computation, especially in public APIs.

## 2024-05-24 - Buffer Overflow in Isosurface Extraction
**Vulnerability:** `extract_isosurface` used `unsafe { get_unchecked }` for performance optimization ("fast path") but failed to validate that the input `VoxelGrid`'s data buffer actually matched the specified dimensions.
**Learning:** Performance optimizations using `unsafe` in scientific computing must be paired with rigorous input validation at the public API boundary. A `Vec<f32>` does not guarantee it has enough elements for the dimensions claimed by the struct.
**Prevention:** Always validate `data.len() >= width * height * depth` at the entry point of functions that use unsafe indexing based on those dimensions.

## 2024-05-22 - Robust Error Handling in Survival Analysis
**Vulnerability:** The `estimate_hazard_ratio_simple` function in `survival_analysis.rs` returned `f64::NAN` when invalid inputs (like zero total time or negative time) were encountered. This could lead to undefined behavior or silent failures in downstream scientific calculations.
**Learning:** Returning `NaN` is a form of "silent failure" in numerical code that can propagate unnoticed. Using `Result` forces the caller to handle the error explicitly, improving robustness.
**Prevention:** In numerical functions, prefer returning `Result<T, E>` over `NaN` or `Inf` when the input is invalid or the calculation is impossible.
