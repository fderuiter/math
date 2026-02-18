use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod clebsch;
pub mod wave_sim;
pub mod spin_viz;

use clebsch::ClebschGordanTool;
use wave_sim::WaveSimulator;

/// A trait for sub-tools within the Quantum tab.
pub trait QuantumTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct QuantumTab {
    tools: Vec<Box<dyn QuantumTool>>,
    selected_tool_index: usize,
}

impl Default for QuantumTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(WaveSimulator::default()),
                Box::new(ClebschGordanTool::default()),
                Box::new(spin_viz::SpinVisualizer::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for QuantumTab {
    fn name(&self) -> &'static str {
        "Quantum Mechanics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("quantum_tool_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
                for (i, tool) in self.tools.iter().enumerate() {
                    if ui.selectable_label(self.selected_tool_index == i, tool.name()).clicked() {
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
                ui.label("No tool selected");
            });
        }
    }
}
