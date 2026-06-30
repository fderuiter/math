// @explorer_feature = "biology"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod hodgkin_huxley;
pub mod neural_network_viz;
pub mod spike_analysis;

use hodgkin_huxley::HodgkinHuxleyTool;
use neural_network_viz::NeuralNetworkVizTool;
use spike_analysis::SpikeAnalysisTool;

pub struct NeuroscienceTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for NeuroscienceTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(HodgkinHuxleyTool::default()),
                Box::new(SpikeAnalysisTool::default()),
                Box::new(NeuralNetworkVizTool::default()),
            ]),
        }
    }
}

impl ExplorerTab for NeuroscienceTab {
    fn name(&self) -> &'static str {
        "Neuroscience"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "neuroscience");
    }
}

// [cite:graph_parameters_rust]
