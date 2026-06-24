use crate::tabs::ExplorerTab;
use eframe::egui;
use crate::framework::SimulationFramework;

pub mod complex_mapping;
pub mod ode;
pub mod riemann;


pub struct AnalysisTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for AnalysisTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![

                Box::new(riemann::RiemannIntegrationTool::default()),
                Box::new(ode::OdeSolverTool::default()),
                Box::new(complex_mapping::ComplexMappingTool::default()),
            
            ]),
        }
    }
}

impl ExplorerTab for AnalysisTab {
    fn name(&self) -> &'static str {
        "Analysis & Calculus"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "analysis");
    }
}

// [cite:graph_parameters_rust]
