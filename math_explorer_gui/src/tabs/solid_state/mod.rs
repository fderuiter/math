use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod crystal_viewer;

/// A trait for sub-tools within the Solid State tab.
pub trait SolidStateTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct SolidStateTab {
    tools: Vec<Box<dyn SolidStateTool>>,
    selected_tool_index: usize,
}

impl Default for SolidStateTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(crystal_viewer::CrystalViewer::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for SolidStateTab {
    fn name(&self) -> &'static str {
        "Solid State Physics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("solid_state_tool_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
                for (i, tool) in self.tools.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_tool_index == i, tool.name())
                        .clicked()
                    {
                        self.selected_tool_index = i;
                    }
                }
            });
        });

        // Delegate to active tool
        if let Some(tool) = self.tools.get_mut(self.selected_tool_index) {
            tool.show(ctx);
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.label("No tool selected");
                });
            });
        }
    }
}
