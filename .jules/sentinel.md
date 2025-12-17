# Sentinel's Security Journal

## 2024-05-22 - Numerical Instability in Mathematical Models
**Vulnerability:** Unbounded inputs in `calculate_favoritism_score` caused Division by Zero and Infinite propagation (DoS risk).
**Learning:** Mathematical implementations often assume ideal inputs (e.g., $x_0 \neq 0$) and neglect software-level safety checks. In a Rust library, propagating `Inf` or `NaN` can crash downstream systems or cause undefined behavior in logic.
**Prevention:** Always sanitize inputs for mathematical functions (clamp denominators away from zero, clamp log inputs to positive domain) before computation, especially in public APIs.
