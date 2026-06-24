use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod beam_profiling;
pub mod dose;

use beam_profiling::BeamProfilingTool;
use dose::DoseCalculationTool;

pub struct MedicalTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for MedicalTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(DoseCalculationTool::default()),
                Box::new(BeamProfilingTool::default()),
            ]),
        }
    }
}

impl ExplorerTab for MedicalTab {
    fn name(&self) -> &'static str {
        "Medical Physics"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "medical");
    }
}

// [cite:graph_parameters_rust]
