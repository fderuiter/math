## 2024-10-24 - Scientific Code Documentation
**Learning:** Developers are users too. Scientific parameters (like `sigma` in Lorenz equations) are often opaque symbols. Bridging the gap between mathematical notation and physical meaning in docstrings (e.g., explaining `sigma` is the Prandtl number) significantly improves the Developer Experience (DX) and educational value of the library.
**Action:** When documenting scientific code, always define the physical meaning of parameters, not just their variable names.

## 2024-10-24 - Magic Numbers in Simulations
**Learning:** Hardcoded loop counts (magic numbers) in simulation logic (like bifurcation diagrams) prevent users from tuning precision vs. performance.
**Action:** Extract tuning parameters into named constants (public if possible) to allow visibility and potential configuration.

## 2024-10-26 - Developer Experience via CLI Output
**Learning:** For library-based projects, implementing `std::fmt::Display` is a high-leverage UX win. It transforms opaque `Debug` structs into human-readable dashboards, enabling developers to instantly verify complex configurations without mental parsing.
**Action:** Implement `Display` for configuration structs, using visual formatting (lines, icons) to create a "dashboard" effect in the terminal.
