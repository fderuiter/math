pub mod cera;
pub mod co2_projections;
pub mod temperature;

use crate::tabs::ExplorerTab;
use eframe::egui;

pub struct ClimateTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for ClimateTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(temperature::TemperatureAnomaliesTool::default()),
                Box::new(cera::CeraTool::default()),
                Box::new(co2_projections::Co2ProjectionsTool::default()),
            ]),
        }
    }
}

impl ExplorerTab for ClimateTab {
    fn name(&self) -> &'static str {
        "Climate"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "climate");
    }
}

// [cite:graph_parameters_rust]
