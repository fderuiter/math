use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod replicator;

/// A trait for sub-tools within the Game Theory tab.
pub trait GameTheoryTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct GameTheoryTab {
    tools: Vec<Box<dyn GameTheoryTool>>,
    selected_tool_index: usize,
}

impl Default for GameTheoryTab {
    fn default() -> Self {
        Self {
            tools: vec![Box::new(replicator::ReplicatorDynamicsTool::default())],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for GameTheoryTab {
    fn name(&self) -> &'static str {
        "Game Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("game_theory_tool_selector").show(ctx, |ui| {
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
