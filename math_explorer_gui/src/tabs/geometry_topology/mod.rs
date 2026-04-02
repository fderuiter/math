use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod curvature_heatmap;
pub mod simplicial_complexes;
pub mod surface_viewer;

/// A trait for sub-tools within the Geometry & Topology tab.
pub trait GeometryTopologyTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct GeometryTopologyTab {
    tools: Vec<Box<dyn GeometryTopologyTool>>,
    selected_tool_index: usize,
}

impl Default for GeometryTopologyTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(surface_viewer::SurfaceViewer::default()),
                Box::new(curvature_heatmap::CurvatureHeatmap::default()),
                Box::new(simplicial_complexes::SimplicialComplexesTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for GeometryTopologyTab {
    fn name(&self) -> &'static str {
        "Geometry & Topology"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("geometry_topology_tool_selector").show(ctx, |ui| {
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
