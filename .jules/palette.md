## 2024-05-19 - Initialization
**Learning:** Found existing UX patterns to enhance.
**Action:** Starting exploration.
## 2024-05-19 - Horizontal Sub-tool Navigation Scrolling
**Learning:** In the egui `math_explorer_gui`, long horizontal lists of dynamically generated tools inside top panels overflow on smaller screens because `ui.horizontal` does not wrap or provide scrolling by default.
**Action:** Wrapped the `ui.horizontal` loop for `self.tools` into an `egui::ScrollArea::horizontal().show(ui, |ui| { ... })` across all tab modules to ensure sub-tabs are accessible without getting cut off off-screen.
