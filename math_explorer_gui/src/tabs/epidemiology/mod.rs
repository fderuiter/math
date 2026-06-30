// @explorer_feature = "epidemiology"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod network_propagation;
pub mod sir;

use network_propagation::NetworkPropagationTool;
use sir::SirTool;

pub struct EpidemiologyTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for EpidemiologyTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(SirTool::default()),
                Box::new(NetworkPropagationTool::default()),
            ]),
        }
    }
}

impl ExplorerTab for EpidemiologyTab {
    fn name(&self) -> &'static str {
        "Epidemiology"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "epidemiology");
    }
}

// [cite:graph_parameters_rust]
