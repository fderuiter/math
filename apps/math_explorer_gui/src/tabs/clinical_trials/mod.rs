use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod randomization;
pub mod sample_size;
pub mod survival;

use randomization::RandomizationTool;
use sample_size::SampleSizeCalculatorTool;
use survival::SurvivalAnalysisTool;

/// A trait for sub-tools within the Clinical Trials tab.
pub trait ClinicalTrialsTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ctx: &egui::Context);
}

pub struct ClinicalTrialsTab {
    tools: Vec<Box<dyn ClinicalTrialsTool>>,
    selected_tool_index: usize,
}

impl Default for ClinicalTrialsTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(SurvivalAnalysisTool::default()),
                Box::new(SampleSizeCalculatorTool::default()),
                Box::new(RandomizationTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for ClinicalTrialsTab {
    fn name(&self) -> &'static str {
        "Clinical Trials"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render Top Menu for Tool Selection
        egui::TopBottomPanel::top("clinical_trials_tool_selector").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Tool:");
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
