## 2024-05-08 - Added Tooltips to Neural Network Viz Execution Buttons
**Learning:** Simulation execution buttons often lack sufficient context (missing tooltips).
**Action:** Always verify if execution buttons have `.on_hover_text` explanations for their state changes, particularly for Play/Pause toggles.
## 2024-05-11 - Enabled Keyboard Submission on Text Input Forms
**Learning:** Text input forms (`ui.text_edit_singleline`) were missing keyboard submission support (Enter key), requiring users to manually click buttons to submit actions.
**Action:** Always check if a single-line text input has an adjacent button and ensure pressing the Enter key inside the text field triggers the same action for better accessibility.
