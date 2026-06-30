// @explorer_feature = "physics"
use crate::tabs::ExplorerTab;
use eframe::egui;

pub mod attractors;
pub mod bifurcation;
pub mod fractals;

pub struct ChaosTab {
    framework: crate::framework::SimulationFramework,
}

impl Default for ChaosTab {
    fn default() -> Self {
        Self {
            framework: crate::framework::SimulationFramework::new(vec![
                Box::new(attractors::AttractorPlotter::default()),
                Box::new(bifurcation::BifurcationDiagram::default()),
                Box::new(fractals::FractalViewer::default()),
            ]),
        }
    }
}

impl ExplorerTab for ChaosTab {
    fn name(&self) -> &'static str {
        "Chaos Theory"
    }

    fn show(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.framework.show(ctx, "chaos");
    }
}

// [cite:graph_parameters_rust]
