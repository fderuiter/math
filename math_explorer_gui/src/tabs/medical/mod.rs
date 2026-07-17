// @explorer_feature = "physics"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod beam_profiling;
pub mod dose;


pub struct MedicalTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for MedicalTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new("medical"),
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

// [cite:stat_mech]
