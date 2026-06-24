use crate::tabs::ExplorerTab;
use eframe::egui;
use crate::framework::SimulationFramework;

pub mod activation_functions;
pub mod attention_maps;
pub mod grid_world;
pub mod loss_landscape;
pub mod q_table_inspector;
pub mod reward_plots;
pub mod tokenization;
pub mod training_monitor;


pub struct AiTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for AiTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![

                Box::new(loss_landscape::LossLandscapeTool::default()),
                Box::new(training_monitor::TrainingMonitorTool::default()),
                Box::new(activation_functions::ActivationFunctionsTool::default()),
                Box::new(attention_maps::AttentionMapsTool::default()),
                Box::new(grid_world::GridWorldTool::default()),
                Box::new(q_table_inspector::QTableInspectorTool::default()),
                Box::new(reward_plots::RewardPlotsTool::default()),
                Box::new(tokenization::TokenizationTool::default()),
            
            ]),
        }
    }
}

impl ExplorerTab for AiTab {
    fn name(&self) -> &'static str {
        "Artificial Intelligence"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "ai");
    }
}

// [cite:graph_parameters_rust]
