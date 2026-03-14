use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod network;
pub mod sir;

use network::NetworkPropagationTool;
use sir::SirTool;

/// A trait for sub-tools within the Epidemiology tab.
pub trait EpidemiologyTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ui: &mut egui::Ui);
}

pub struct EpidemiologyTab {
    tools: Vec<Box<dyn EpidemiologyTool>>,
    selected_tool_index: usize,
}

impl Default for EpidemiologyTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(SirTool::default()),
                Box::new(NetworkPropagationTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for EpidemiologyTab {
    fn name(&self) -> &'static str {
        "Epidemiology"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("epidemiology_tool_selector").show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();
            for (i, tool) in self.tools.iter().enumerate() {
                if ui
                    .selectable_label(self.selected_tool_index == i, tool.name())
                    .clicked()
                {
                    self.selected_tool_index = i;
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tool) = self.tools.get_mut(self.selected_tool_index) {
                tool.show(ui);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No tool selected");
                });
            }
        });
    }
}
