use crate::tabs::ExplorerTab;
use eframe::egui;
use crate::framework::SimulationFramework;

pub mod bankroll_growth;
pub mod bet_size;


pub struct FinancialMathTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for FinancialMathTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![

                Box::new(bankroll_growth::BankrollGrowthTool::default()),
                Box::new(bet_size::BetSizeCalculatorTool::default()),
            
            ]),
        }
    }
}

impl ExplorerTab for FinancialMathTab {
    fn name(&self) -> &'static str {
        "Financial Math"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "financial_math");
    }
}

// [cite:clinical_trials_statistics]
