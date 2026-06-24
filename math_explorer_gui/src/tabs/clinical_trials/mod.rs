use crate::tabs::ExplorerTab;
use eframe::egui;
use crate::framework::SimulationFramework;

pub mod randomization;
pub mod sample_size;
pub mod survival;

use randomization::RandomizationTool;
use sample_size::SampleSizeCalculatorTool;
use survival::SurvivalAnalysisTool;


pub struct ClinicalTrialsTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for ClinicalTrialsTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![

                Box::new(SurvivalAnalysisTool::default()),
                Box::new(SampleSizeCalculatorTool::default()),
                Box::new(RandomizationTool::default()),
            
            ]),
        }
    }
}

impl ExplorerTab for ClinicalTrialsTab {
    fn name(&self) -> &'static str {
        "Clinical Trials"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "clinical_trials");
    }
}

// [cite:clinical_trials_statistics]
