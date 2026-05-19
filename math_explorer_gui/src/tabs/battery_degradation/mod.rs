use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod capacity_fade;
pub mod lifetime_estimator;

use capacity_fade::CapacityFadeTool;
use lifetime_estimator::LifetimeEstimatorTool;

/// A trait for sub-tools within the Battery Degradation tab.
pub trait BatteryDegradationTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct BatteryDegradationTab {
    tools: Vec<Box<dyn BatteryDegradationTool>>,
    selected_tool_index: usize,
}

impl Default for BatteryDegradationTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(CapacityFadeTool::default()),
                Box::new(LifetimeEstimatorTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for BatteryDegradationTab {
    fn name(&self) -> &'static str {
        "Battery Degradation"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("battery_degradation_tool_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
                egui::ScrollArea::horizontal().show(ui, |ui| {
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
