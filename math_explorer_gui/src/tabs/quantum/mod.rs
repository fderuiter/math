use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod clebsch;
pub mod wave_sim;

use clebsch::ClebschGordanTool;
use wave_sim::WaveSimulator;

#[derive(Debug, PartialEq, Clone, Copy)]
enum QuantumTool {
    WaveSim,
    Clebsch,
}

pub struct QuantumTab {
    active_tool: QuantumTool,
    wave_sim: WaveSimulator,
    clebsch: ClebschGordanTool,
}

impl Default for QuantumTab {
    fn default() -> Self {
        Self {
            active_tool: QuantumTool::WaveSim,
            wave_sim: WaveSimulator::default(),
            clebsch: ClebschGordanTool::default(),
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
                ui.selectable_value(&mut self.active_tool, QuantumTool::WaveSim, "Wave Simulator");
                ui.selectable_value(&mut self.active_tool, QuantumTool::Clebsch, "Clebsch-Gordan");
            });
        });

        // Delegate to active tool
        match self.active_tool {
            QuantumTool::WaveSim => self.wave_sim.show(ctx),
            QuantumTool::Clebsch => self.clebsch.show(ctx),
        }
    }
}
