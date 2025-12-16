## 2024-10-24 - Scientific Code Documentation
**Learning:** Developers are users too. Scientific parameters (like `sigma` in Lorenz equations) are often opaque symbols. Bridging the gap between mathematical notation and physical meaning in docstrings (e.g., explaining `sigma` is the Prandtl number) significantly improves the Developer Experience (DX) and educational value of the library.
**Action:** When documenting scientific code, always define the physical meaning of parameters, not just their variable names.

## 2024-10-24 - Magic Numbers in Simulations
**Learning:** Hardcoded loop counts (magic numbers) in simulation logic (like bifurcation diagrams) prevent users from tuning precision vs. performance.
**Action:** Extract tuning parameters into named constants (public if possible) to allow visibility and potential configuration.
