use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod bankroll_growth;
pub mod bet_size;

/// A trait for sub-tools within the Financial Math tab.
pub trait FinancialMathTool {
    /// Returns the name of the tool.
    fn name(&self) -> &'static str;

    /// Renders the tool's UI.
    fn show(&mut self, ui: &mut egui::Ui);
}

pub struct FinancialMathTab {
    tools: Vec<Box<dyn FinancialMathTool>>,
    selected_tool_index: usize,
}

impl Default for FinancialMathTab {
    fn default() -> Self {
        Self {
            tools: vec![
                Box::new(bankroll_growth::BankrollGrowthTool::default()),
                Box::new(bet_size::BetSizeCalculatorTool::default()),
            ],
            selected_tool_index: 0,
        }
    }
}

impl ExplorerTab for FinancialMathTab {
    fn name(&self) -> &'static str {
        "Financial Math"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("financial_math_tool_selector").show(ctx, |ui| {
            ui.heading("Tools");
            ui.separator();
            for (i, tool) in self.tools.iter().enumerate() {
                if ui
                    .selectable_label(self.selected_tool_index == i, tool.name())
                    .clicked()
                {
                    self.selected_tool_index = i;
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tool) = self.tools.get_mut(self.selected_tool_index) {
                tool.show(ui);
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("No tool selected");
                });
            }
        });
    }
}

// [cite:clinical_trials_statistics]
