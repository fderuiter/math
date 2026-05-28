pub mod cera;
pub mod co2_projections;
pub mod temperature;

use crate::tabs::ExplorerTab;
use eframe::egui;

/// A trait for individual tools within the Climate tab.
pub trait ClimateTool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui);
}

pub struct ClimateTab {
    tools: Vec<Box<dyn ClimateTool>>,
    selected_tool: usize,
}

impl Default for ClimateTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(temperature::TemperatureAnomaliesTool::default()),
                Box::new(cera::CeraTool::default()),
                Box::new(co2_projections::Co2ProjectionsTool::default()),
            ],
            selected_tool: 0,
        }
    }
}

impl ExplorerTab for ClimateTab {
    fn name(&self) -> &'static str {
        "Climate"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("climate_tool_selector")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Climate Tools");
                ui.separator();
                for (i, tool) in self.tools.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_tool == i, tool.name())
                        .clicked()
                    {
                        self.selected_tool = i;
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tool) = self.tools.get_mut(self.selected_tool) {
                ui.heading(tool.name());
                ui.separator();
                tool.show(ui);
            }
        });
    }
}

// [cite:graph_parameters_rust]
