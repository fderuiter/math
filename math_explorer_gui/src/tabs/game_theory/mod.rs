use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod replicator;

pub struct GameTheoryTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for GameTheoryTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![Box::new(
                replicator::ReplicatorDynamicsTool::default(),
            )]),
        }
    }
}

impl ExplorerTab for GameTheoryTab {
    fn name(&self) -> &'static str {
        "Game Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "game_theory");
    }
}

// [cite:graph_parameters_rust]
