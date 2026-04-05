use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod cera_tool;
pub mod temperature_anomalies;
use cera_tool::CeraTool;
use temperature_anomalies::TemperatureAnomaliesTool;

/// A trait for individual tools within the Climate Tab.
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
                Box::new(TemperatureAnomaliesTool::default()),
                Box::new(CeraTool::default()),
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
        egui::SidePanel::left("climate_tools_panel").show(ctx, |ui| {
            ui.heading("Climate Models");
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
                tool.show(ui);
            }
        });
    }
}
