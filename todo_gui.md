# Math Explorer GUI Development Roadmap

This document outlines the planned roadmap for `math_explorer_gui`, aiming to connect the frontend to the WASM-compiled core engine and provide interactive exploration of mathematical models.

## Phase 1: Web GUI Integration
*Goal: Connect the frontend to the WASM-compiled core engine.*

- [x] **1.1 Web Build Setup**
  - Ensure `math_explorer_gui` relies on `eframe` (egui).
  - Create `index.html` in `apps/math_explorer_gui/`.
  - Install Trunk (`cargo install trunk`).
- [x] **1.2 Link the Engine**
  - Add `math_explorer` to the GUI's `Cargo.toml` with specific features enabled (e.g., `features = ["physics", "math"]`).
- [ ] **1.3 Implement First Trait**
  - Choose a simple model (e.g., Lorenz Attractor in `oxidize_physics`).
  - Implement `ModelConfig`, `ModelState`, and `SimulationModel` for it.
- [ ] **1.4 GUI Rendering Loop**
  - In the egui update loop, call `model.step()` and `model.get_state()`.
  - Render the state to the screen.
  - Verify 60FPS execution in the browser via `trunk serve`.
