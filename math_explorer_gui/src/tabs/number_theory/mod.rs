// @explorer_feature = "pure_math"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod factorization;
pub mod partitions_widget;
pub mod prime_spiral;
pub mod ualbf_widget;

pub struct NumberTheoryTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for NumberTheoryTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new("number_theory"),
        }
    }
}

impl ExplorerTab for NumberTheoryTab {
    fn name(&self) -> &'static str {
        "Number Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "number_theory");
    }
}

// [cite:stat_mech]
