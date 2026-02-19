use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod hodgkin_huxley;

use hodgkin_huxley::HodgkinHuxleyTool;

/// A trait for sub-tools within the Neuroscience tab.
pub trait NeuroscienceTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct NeuroscienceTab {
    tools: Vec<Box<dyn NeuroscienceTool>>,
    selected_tool_index: usize,
}

impl Default for NeuroscienceTab {
    fn default() -> Self {
        Self {
            tools: vec![Box::new(HodgkinHuxleyTool::default())],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for NeuroscienceTab {
    fn name(&self) -> &'static str {
        "Neuroscience"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("neuroscience_tool_selector").show(ctx, |ui| {
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
