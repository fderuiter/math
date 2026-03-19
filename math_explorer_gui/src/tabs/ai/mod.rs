use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod activation_functions;
pub mod attention_maps;
pub mod grid_world;
pub mod loss_landscape;
pub mod q_table_inspector;
pub mod training_monitor;

/// A trait for sub-tools within the AI tab.
pub trait AiTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct AiTab {
    tools: Vec<Box<dyn AiTool>>,
    selected_tool_index: usize,
}

impl Default for AiTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(loss_landscape::LossLandscapeTool::default()),
                Box::new(training_monitor::TrainingMonitorTool::default()),
                Box::new(activation_functions::ActivationFunctionsTool::default()),
                Box::new(attention_maps::AttentionMapsTool::default()),
                Box::new(grid_world::GridWorldTool::default()),
                Box::new(q_table_inspector::QTableInspectorTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for AiTab {
    fn name(&self) -> &'static str {
        "Artificial Intelligence"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("ai_tool_selector").show(ctx, |ui| {
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
