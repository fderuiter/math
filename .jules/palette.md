## 2024-10-24 - Scientific Code Documentation
**Learning:** Developers are users too. Scientific parameters (like `sigma` in Lorenz equations) are often opaque symbols. Bridging the gap between mathematical notation and physical meaning in docstrings (e.g., explaining `sigma` is the Prandtl number) significantly improves the Developer Experience (DX) and educational value of the library.
**Action:** When documenting scientific code, always define the physical meaning of parameters, not just their variable names.

## 2024-10-24 - Magic Numbers in Simulations
**Learning:** Hardcoded loop counts (magic numbers) in simulation logic (like bifurcation diagrams) prevent users from tuning precision vs. performance.
**Action:** Extract tuning parameters into named constants (public if possible) to allow visibility and potential configuration.

## 2025-05-14 - First Impressions in CLI Tools
**Learning:** Scientific libraries often output plain text that can be dry and intimidating. Adding small touches like ASCII art banners, semantic colors (green for success), and structured tables to the "Hello World" example creates an immediate sense of polish and approachability for new users.
**Action:** Ensure the "entry point" example of any library is not just functional but visually delightful and self-explanatory.
