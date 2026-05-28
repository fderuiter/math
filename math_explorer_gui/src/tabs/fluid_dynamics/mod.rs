use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod lattice_boltzmann;
pub mod potential_flow;
pub mod turbulence;

use lattice_boltzmann::LatticeBoltzmannTool;
use potential_flow::PotentialFlowTool;
use turbulence::TurbulenceTool;

/// A trait for sub-tools within the Fluid Dynamics tab.
pub trait FluidDynamicsTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct FluidDynamicsTab {
    tools: Vec<Box<dyn FluidDynamicsTool>>,
    selected_tool_index: usize,
}

impl Default for FluidDynamicsTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(PotentialFlowTool::default()),
                Box::new(TurbulenceTool::default()),
                Box::new(LatticeBoltzmannTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for FluidDynamicsTab {
    fn name(&self) -> &'static str {
        "Fluid Dynamics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("fluid_mode_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
                egui::ScrollArea::horizontal().show(ui, |ui| {
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

// [cite:graph_parameters_rust]
