// @explorer_feature = "pure_math"
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
            framework: crate::framework::SimulationFramework::new("graph_theory"),
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
