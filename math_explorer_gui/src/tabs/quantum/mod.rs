use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod clebsch;
pub mod spin_viz;
pub mod wave_sim;

use clebsch::ClebschGordanTool;
use wave_sim::WaveSimulator;

pub struct QuantumTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for QuantumTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(WaveSimulator::default()),
                Box::new(ClebschGordanTool::default()),
                Box::new(spin_viz::SpinVisualizer::default()),
            ]),
        }
    }
}

impl ExplorerTab for QuantumTab {
    fn name(&self) -> &'static str {
        "Quantum Mechanics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "quantum");
    }
}

// [cite:quantum_mechanics]
