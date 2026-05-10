## 2024-05-24 - [Keyboard Accessibility in egui text inputs]
**Learning:** Adding keyboard submission support to egui `text_edit_singleline` significantly improves the usability of forms. `response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))` ensures the action only triggers when Enter is pressed after typing.
**Action:** Always check for Enter key presses when implementing text input forms in egui.
