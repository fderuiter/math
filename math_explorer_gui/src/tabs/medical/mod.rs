use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod beam_profiling;
pub mod dose;

use beam_profiling::BeamProfilingTool;
use dose::DoseCalculationTool;

/// A trait for sub-tools within the Medical Physics tab.
pub trait MedicalTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct MedicalTab {
    tools: Vec<Box<dyn MedicalTool>>,
    selected_tool_index: usize,
}

impl Default for MedicalTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(DoseCalculationTool::default()),
                Box::new(BeamProfilingTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for MedicalTab {
    fn name(&self) -> &'static str {
        "Medical Physics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("medical_tool_selector").show(ctx, |ui| {
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
