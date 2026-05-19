use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod complex_mapping;
pub mod ode;
pub mod riemann;

/// A trait for sub-tools within the Analysis & Calculus tab.
pub trait AnalysisTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct AnalysisTab {
    tools: Vec<Box<dyn AnalysisTool>>,
    selected_tool_index: usize,
}

impl Default for AnalysisTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(riemann::RiemannIntegrationTool::default()),
                Box::new(ode::OdeSolverTool::default()),
                Box::new(complex_mapping::ComplexMappingTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for AnalysisTab {
    fn name(&self) -> &'static str {
        "Analysis & Calculus"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("analysis_tool_selector").show(ctx, |ui| {
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
