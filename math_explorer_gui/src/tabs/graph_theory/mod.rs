use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod algorithm_visualizer;
pub mod graph_editor;
pub mod network_metrics;

pub struct GraphTheoryTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for GraphTheoryTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(graph_editor::GraphEditorTool::default()),
                Box::new(algorithm_visualizer::AlgorithmVisualizerTool::default()),
                Box::new(network_metrics::NetworkMetricsTool::default()),
            ]),
        }
    }
}

impl ExplorerTab for GraphTheoryTab {
    fn name(&self) -> &'static str {
        "Graph Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "graph_theory");
    }
}

// [cite:graph_parameters_rust]
