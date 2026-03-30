use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod algorithm_visualizer;
pub mod graph_editor;
pub mod network_metrics;

/// A trait for sub-tools within the Graph Theory tab.
pub trait GraphTheoryTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct GraphTheoryTab {
    tools: Vec<Box<dyn GraphTheoryTool>>,
    selected_tool_index: usize,
}

impl Default for GraphTheoryTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(graph_editor::GraphEditorTool::default()),
                Box::new(algorithm_visualizer::AlgorithmVisualizerTool::default()),
                Box::new(network_metrics::NetworkMetricsTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for GraphTheoryTab {
    fn name(&self) -> &'static str {
        "Graph Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("graph_theory_tool_selector").show(ctx, |ui| {
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
