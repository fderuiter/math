// @explorer_feature = "applied"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod capacity_fade;
pub mod lifetime_estimator;

use capacity_fade::CapacityFadeTool;
use lifetime_estimator::LifetimeEstimatorTool;

pub struct BatteryDegradationTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for BatteryDegradationTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(CapacityFadeTool::default()),
                Box::new(LifetimeEstimatorTool::default()),
            ]),
        }
    }
}

impl ExplorerTab for BatteryDegradationTab {
    fn name(&self) -> &'static str {
        "Battery Degradation"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "battery_degradation");
    }
}

// [cite:algorithmic_information_rust]
