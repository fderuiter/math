## 2024-05-08 - Added Tooltips to Neural Network Viz Execution Buttons
**Learning:** Simulation execution buttons often lack sufficient context (missing tooltips).
**Action:** Always verify if execution buttons have `.on_hover_text` explanations for their state changes, particularly for Play/Pause toggles.

## 2024-05-18 - Standardize Execution Button Tooltips and Icons
**Learning:** Found that many execution buttons (like Run, Pause, Reset, Clear) across different simulations were missing `on_hover_text` descriptions, making their effect on the complex simulation states opaque, especially for new users or screen readers. Also, 'Reset' icons were inconsistently applied (some used '↺', some had none).
**Action:** Always append `.on_hover_text()` to state-modifying buttons in `egui` to explicitly explain what the action does to the simulation. Standardize the 'Reset' button icon to `↻` across all tabs.
