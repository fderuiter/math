# Architectural Decision Records

## 2024-05-23 - MRI Module Extraction
**Problem:** The `math_explorer/src/physics/mri.rs` file mixes distinct domains: Quantum Foundations, Classical Bloch Dynamics, Scanner Spatial Encoding, and Image Reconstruction. This violates the separation of concerns and hampers maintainability.
**Decision:** Extract the `mri.rs` file into a directory `math_explorer/src/physics/mri/` with submodules:
- `proton.rs`: Quantum constants and Larmor frequency.
- `bloch.rs`: Bloch equation simulation and solver logic.
- `scanner.rs`: Gradient fields and k-space math.
- `reconstruction.rs`: DFT and signal processing logic.
- `mod.rs`: Re-exports all components to maintain API backward compatibility.
**Consequence:** The codebase becomes more modular and follows the "Hub and Spoke" pattern. Imports within the module need to be updated to reference `crate::` paths or relative paths correctly. The public API remains unchanged for external consumers.
