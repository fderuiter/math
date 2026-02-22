use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod randomization;
pub mod sample_size;
pub mod survival;

pub trait ClinicalTrialTool {
    fn name(&self) -> &'static str;
    fn show(&mut self, ui: &mut egui::Ui);
}

pub struct ClinicalTrialsTab {
    tools: Vec<Box<dyn ClinicalTrialTool>>,
    selected_tool: usize,
}

impl Default for ClinicalTrialsTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(survival::SurvivalAnalysisTool::default()),
                Box::new(sample_size::SampleSizeTool::default()),
                Box::new(randomization::RandomizationTool::default()),
            ],
            selected_tool: 0,
        }
    }
}

impl ExplorerTab for ClinicalTrialsTab {
    fn name(&self) -> &'static str {
        "Clinical Trials"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("clinical_trials_side_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Tools");
                ui.separator();
                for (i, tool) in self.tools.iter().enumerate() {
                    if ui
                        .selectable_label(self.selected_tool == i, tool.name())
                        .clicked()
                    {
                        self.selected_tool = i;
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tool) = self.tools.get_mut(self.selected_tool) {
                ui.heading(tool.name());
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    tool.show(ui);
                });
            }
        });
    }
}
